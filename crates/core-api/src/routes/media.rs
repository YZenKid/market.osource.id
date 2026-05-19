use axum::{
    body::Bytes,
    extract::{ConnectInfo, Multipart, Path, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use core_app::AppState;
use core_storage::{FileVisibility, PutObjectInput, StorageProvider};
use serde::Serialize;
use serde_json::Value;
use std::net::SocketAddr;
use tower_http::limit::RequestBodyLimitLayer;

use crate::{
    authz::{resolve_authenticated_actor, AuthenticatedActor},
    routes::auth::{client_identity, require_csrf, AuthErrorResponse},
};

const PAYMENT_PROOF_BUCKET: &str = "payment-proofs";
const PAYMENT_PROOF_UPLOAD_MAX_BODY_BYTES: usize = core_storage::MAX_UPLOAD_BYTES + (64 * 1024);

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/storefront/orders/{tracking_token}/payment-proof",
            post(upload_payment_proof).route_layer(RequestBodyLimitLayer::new(
                PAYMENT_PROOF_UPLOAD_MAX_BODY_BYTES,
            )),
        )
        .route("/media/{file_id}", get(get_protected_media))
        .route(
            "/api/admin/payment-proofs/{proof_id}/verify",
            post(verify_payment_proof),
        )
        .route(
            "/api/admin/payment-proofs/{proof_id}/reject",
            post(reject_payment_proof),
        )
}

#[derive(Debug, Serialize)]
pub struct PaymentProofUploadResponse {
    pub payment_proof_id: uuid::Uuid,
    pub file_id: uuid::Uuid,
    pub status: String,
    pub gate_f_note: &'static str,
}

#[derive(Debug, Serialize)]
pub struct PaymentProofDecisionResponse {
    pub payment_proof_id: uuid::Uuid,
    pub order_id: uuid::Uuid,
    pub status: String,
    pub gate_f_note: &'static str,
}

async fn upload_payment_proof(
    State(state): State<AppState>,
    ConnectInfo(peer_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Path(tracking_token): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<PaymentProofUploadResponse>, (StatusCode, Json<AuthErrorResponse>)> {
    // TODO: Replace this single-process limiter with a distributed, proxy-aware limiter once
    // reverse proxy trusted headers and multi-node deployment are wired.
    if !state
        .rate_limiter
        .check(payment_proof_upload_rate_limit_key(
            peer_addr,
            &headers,
            &tracking_token,
        ))
        .allowed
    {
        return Err(api_error(
            StatusCode::TOO_MANY_REQUESTS,
            "rate_limited",
            "too many attempts; try again later".to_string(),
        ));
    }

    let Some(pool) = &state.db_pool else {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "database_unavailable",
            "database is required for payment proof upload".to_string(),
        ));
    };

    let Some((filename, bytes)) = read_single_payment_proof_file(&mut multipart).await? else {
        audit_media_event(
            Some(pool),
            None,
            "payment_proof.upload",
            "payment_proof",
            None,
            "denied",
            &[("reason", Value::String("missing_file".to_string()))],
        )
        .await;
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "missing_file",
            "multipart field `file` is required".to_string(),
        ));
    };

    if core_db::find_order_id_by_tracking_token(pool, &tracking_token)
        .await
        .map_err(|error| {
            tracing::warn!(error = %error, "failed to validate payment proof tracking token");
            api_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "order_lookup_failed",
                "order lookup failed".to_string(),
            )
        })?
        .is_none()
    {
        audit_media_event(
            Some(pool),
            None,
            "payment_proof.upload",
            "payment_proof",
            None,
            "denied",
            &[("reason", Value::String("order_not_found".to_string()))],
        )
        .await;
        return Err(api_error(
            StatusCode::NOT_FOUND,
            "order_not_found",
            "order tracking token was not found".to_string(),
        ));
    }

    let stored = state
        .storage
        .put_object(PutObjectInput {
            bucket: PAYMENT_PROOF_BUCKET.to_string(),
            original_filename: filename.clone(),
            bytes,
            visibility: FileVisibility::PrivateAdmin,
        })
        .await
        .map_err(|error| {
            tracing::warn!(error = %error, "payment proof upload rejected by storage validation");
            api_error(
                StatusCode::BAD_REQUEST,
                "invalid_upload",
                "payment proof upload is invalid or exceeds allowed limits".to_string(),
            )
        })?;

    let file = core_db::create_file_object(
        pool,
        core_db::CreateFileObjectInput {
            storage_provider: "local".to_string(),
            bucket: stored.bucket,
            object_key: stored.object_key,
            // Keep original filename only as metadata; storage paths use generated object_key.
            original_filename: filename,
            mime_type: stored.mime_type,
            size_bytes: stored.size_bytes as i64,
            checksum: stored.checksum,
            visibility: "private_admin".to_string(),
            created_by_user_id: None,
        },
    )
    .await
    .map_err(|error| {
        tracing::warn!(error = %error, "failed to create payment proof file metadata");
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "file_metadata_create_failed",
            "payment proof could not be recorded".to_string(),
        )
    })?;

    let proof = match core_db::create_payment_proof_for_tracking_token(
        pool,
        &tracking_token,
        file.id,
        None,
    )
    .await
    {
        Ok(proof) => proof,
        Err(core_db::MediaRepositoryError::OrderNotFound) => {
            audit_media_event(
                Some(pool),
                None,
                "payment_proof.upload",
                "file_object",
                Some(file.id),
                "denied",
                &[("reason", Value::String("order_not_found".to_string()))],
            )
            .await;
            return Err(api_error(
                StatusCode::NOT_FOUND,
                "order_not_found",
                "order tracking token was not found".to_string(),
            ));
        }
        Err(error) => {
            tracing::warn!(error = %error, file_id = %file.id, "failed to create payment proof");
            return Err(api_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "payment_proof_create_failed",
                "payment proof could not be recorded".to_string(),
            ));
        }
    };

    audit_media_event(
        Some(pool),
        None,
        "payment_proof.upload",
        "payment_proof",
        Some(proof.id),
        "allowed",
        &[
            ("file_id", Value::String(file.id.to_string())),
            ("size_bytes", Value::Number(file.size_bytes.into())),
        ],
    )
    .await;

    Ok(Json(PaymentProofUploadResponse {
        payment_proof_id: proof.id,
        file_id: file.id,
        status: proof.status,
        gate_f_note:
            "Payment proof stored as private media; guest/customer media reads remain denied.",
    }))
}

fn payment_proof_upload_rate_limit_key(
    peer_addr: SocketAddr,
    headers: &HeaderMap,
    tracking_token: &str,
) -> String {
    format!(
        "payment_proof_upload:{}:{tracking_token}",
        client_identity(peer_addr, headers)
    )
}

async fn get_protected_media(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(file_id): Path<uuid::Uuid>,
) -> Result<Response, (StatusCode, Json<AuthErrorResponse>)> {
    let Some(pool) = &state.db_pool else {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "database_unavailable",
            "database is required for protected media".to_string(),
        ));
    };

    let actor = match resolve_authenticated_actor(&state, &headers).await {
        Ok(actor) => actor,
        Err(_) => {
            audit_media_event(
                Some(pool),
                None,
                "media.read",
                "file_object",
                Some(file_id),
                "denied",
                &[("reason", Value::String("unauthenticated".to_string()))],
            )
            .await;
            return Err(api_error(
                StatusCode::UNAUTHORIZED,
                "unauthenticated",
                "a valid session is required".to_string(),
            ));
        }
    };

    let Some((file, proof)) = core_db::find_payment_proof_file(pool, file_id)
        .await
        .map_err(|error| {
            tracing::warn!(error = %error, file_id = %file_id, "failed to read protected media metadata");
            api_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "media_lookup_failed",
                "protected media lookup failed".to_string(),
            )
        })?
    else {
        return Err(api_error(
            StatusCode::NOT_FOUND,
            "media_not_found",
            "protected media was not found".to_string(),
        ));
    };

    if !can_read_payment_proof(pool, &actor, proof.order_id).await? {
        audit_media_event(
            Some(pool),
            Some(actor.user_id),
            "media.read",
            "file_object",
            Some(file.id),
            "denied",
            &[("reason", Value::String("permission_denied".to_string()))],
        )
        .await;
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "forbidden",
            "payment proof media access is not allowed".to_string(),
        ));
    }

    let object = state
        .storage
        .read_object(&file.bucket, &file.object_key)
        .await
        .map_err(|error| {
            tracing::warn!(error = %error, file_id = %file.id, "failed to read protected media object");
            api_error(
                StatusCode::NOT_FOUND,
                "media_object_not_found",
                "protected media object was not found".to_string(),
            )
        })?;

    audit_media_event(
        Some(pool),
        Some(actor.user_id),
        "media.read",
        "file_object",
        Some(file.id),
        "allowed",
        &[("proof_id", Value::String(proof.id.to_string()))],
    )
    .await;

    let mut response_headers = HeaderMap::new();
    response_headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&file.mime_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
    );
    response_headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("private, no-store"),
    );
    Ok((response_headers, Bytes::from(object.bytes)).into_response())
}

async fn verify_payment_proof(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(proof_id): Path<uuid::Uuid>,
) -> Result<Json<PaymentProofDecisionResponse>, (StatusCode, Json<AuthErrorResponse>)> {
    decide_payment_proof_endpoint(
        state,
        headers,
        proof_id,
        core_db::PaymentProofDecision::Verified,
    )
    .await
}

async fn reject_payment_proof(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(proof_id): Path<uuid::Uuid>,
) -> Result<Json<PaymentProofDecisionResponse>, (StatusCode, Json<AuthErrorResponse>)> {
    decide_payment_proof_endpoint(
        state,
        headers,
        proof_id,
        core_db::PaymentProofDecision::Rejected,
    )
    .await
}

async fn decide_payment_proof_endpoint(
    state: AppState,
    headers: HeaderMap,
    proof_id: uuid::Uuid,
    decision: core_db::PaymentProofDecision,
) -> Result<Json<PaymentProofDecisionResponse>, (StatusCode, Json<AuthErrorResponse>)> {
    require_csrf(&headers)?;
    let Some(pool) = &state.db_pool else {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "database_unavailable",
            "database is required for payment proof decisions".to_string(),
        ));
    };
    let actor = resolve_authenticated_actor(&state, &headers).await?;

    let Some(existing_proof) = core_db::find_payment_proof_by_id(pool, proof_id)
        .await
        .map_err(|error| {
            tracing::warn!(error = %error, proof_id = %proof_id, "failed to lookup payment proof scope");
            api_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "payment_proof_lookup_failed",
                "payment proof lookup failed".to_string(),
            )
        })?
    else {
        return Err(api_error(
            StatusCode::NOT_FOUND,
            "payment_proof_not_found",
            "payment proof was not found".to_string(),
        ));
    };

    let allowed = can_decide_payment_proof(pool, &actor, existing_proof.order_id, decision).await?;
    if !allowed {
        audit_media_event(
            Some(pool),
            Some(actor.user_id),
            "payment_proof.decide",
            "payment_proof",
            Some(proof_id),
            "denied",
            &[("reason", Value::String("permission_denied".to_string()))],
        )
        .await;
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "forbidden",
            "payment proof decision access is not allowed".to_string(),
        ));
    }

    let proof = core_db::decide_payment_proof(pool, proof_id, actor.user_id, decision, None)
        .await
        .map_err(|error| match error {
            core_db::MediaRepositoryError::PaymentProofNotFound => api_error(StatusCode::NOT_FOUND, "payment_proof_not_found", "payment proof was not found".to_string()),
            other => {
                tracing::warn!(error = %other, proof_id = %proof_id, "failed to decide payment proof");
                api_error(StatusCode::INTERNAL_SERVER_ERROR, "payment_proof_decision_failed", "payment proof decision failed".to_string())
            }
        })?;

    audit_media_event(
        Some(pool),
        Some(actor.user_id),
        "payment_proof.decide",
        "payment_proof",
        Some(proof.id),
        "allowed",
        &[("status", Value::String(proof.status.clone()))],
    )
    .await;

    Ok(Json(PaymentProofDecisionResponse {
        payment_proof_id: proof.id,
        order_id: proof.order_id,
        status: proof.status,
        gate_f_note: "Payment proof decision enforces explicit brand/order-scoped permissions for sellers; super admin remains full access.",
    }))
}

async fn can_decide_payment_proof(
    pool: &sqlx::PgPool,
    actor: &AuthenticatedActor,
    order_id: uuid::Uuid,
    decision: core_db::PaymentProofDecision,
) -> Result<bool, (StatusCode, Json<AuthErrorResponse>)> {
    use crate::authz::{is_brand_scoped, is_full_access};

    if is_full_access(actor) {
        return Ok(true);
    }
    if !is_brand_scoped(actor) {
        return Ok(false);
    }

    let assigned = core_db::order_contains_assigned_seller_brand(pool, order_id, actor.user_id)
        .await
        .map_err(media_auth_lookup_failed)?;
    if !assigned {
        return Ok(false);
    }

    let permission = match decision {
        core_db::PaymentProofDecision::Verified => core_db::PAYMENT_PROOF_VERIFY,
        core_db::PaymentProofDecision::Rejected => core_db::PAYMENT_PROOF_REJECT,
    };

    core_db::user_has_brand_scoped_permission_for_order(pool, actor.user_id, order_id, permission)
        .await
        .map_err(media_auth_lookup_failed)
}

async fn can_read_payment_proof(
    pool: &sqlx::PgPool,
    actor: &AuthenticatedActor,
    order_id: uuid::Uuid,
) -> Result<bool, (StatusCode, Json<AuthErrorResponse>)> {
    use crate::authz::{is_brand_scoped, is_full_access};

    if is_full_access(actor) {
        return Ok(true);
    }
    if !is_brand_scoped(actor) {
        return Ok(false);
    }

    let assigned = core_db::order_contains_assigned_seller_brand(pool, order_id, actor.user_id)
        .await
        .map_err(media_auth_lookup_failed)?;
    if !assigned {
        return Ok(false);
    }

    core_db::user_has_brand_scoped_permission_for_order(
        pool,
        actor.user_id,
        order_id,
        core_db::PAYMENT_PROOF_VIEW_ASSIGNED,
    )
    .await
    .map_err(media_auth_lookup_failed)
}

async fn read_single_payment_proof_file(
    multipart: &mut Multipart,
) -> Result<Option<(String, Vec<u8>)>, (StatusCode, Json<AuthErrorResponse>)> {
    while let Some(field) = multipart.next_field().await.map_err(|_| {
        api_error(
            StatusCode::BAD_REQUEST,
            "invalid_multipart",
            "invalid multipart upload".to_string(),
        )
    })? {
        if field.name() != Some("file") {
            continue;
        }
        let filename = field.file_name().unwrap_or("proof.bin").to_string();
        let bytes = field.bytes().await.map_err(|_| {
            api_error(
                StatusCode::BAD_REQUEST,
                "invalid_multipart",
                "invalid multipart file".to_string(),
            )
        })?;
        return Ok(Some((filename, bytes.to_vec())));
    }
    Ok(None)
}

fn media_auth_lookup_failed(
    error: core_db::MediaRepositoryError,
) -> (StatusCode, Json<AuthErrorResponse>) {
    tracing::warn!(error = %error, "failed protected media authorization lookup");
    api_error(
        StatusCode::INTERNAL_SERVER_ERROR,
        "media_authorization_failed",
        "protected media authorization failed".to_string(),
    )
}

fn api_error(
    status: StatusCode,
    error: &'static str,
    message: String,
) -> (StatusCode, Json<AuthErrorResponse>) {
    (
        status,
        Json(AuthErrorResponse {
            error,
            message,
            gate_d_note: None,
        }),
    )
}

async fn audit_media_event(
    pool: Option<&sqlx::PgPool>,
    actor_user_id: Option<uuid::Uuid>,
    action: &str,
    target_type: &str,
    target_id: Option<uuid::Uuid>,
    result: &str,
    metadata_entries: &[(&str, Value)],
) {
    let Some(pool) = pool else {
        return;
    };
    let metadata = match core_db::safe_audit_metadata(metadata_entries) {
        Ok(metadata) => metadata,
        Err(error) => {
            tracing::warn!(error = %error, action, "media audit metadata rejected");
            serde_json::json!({})
        }
    };

    if let Err(error) = core_db::write_audit_event(
        pool,
        core_db::AuditEventInput {
            actor_user_id,
            action: action.to_string(),
            target_type: target_type.to_string(),
            target_id,
            result: result.to_string(),
            metadata,
        },
    )
    .await
    {
        tracing::warn!(error = %error, action, "failed to write media audit event");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::app_router;
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use core_runtime::{AppConfig, RuntimeMode};
    use secrecy::SecretString;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use tower::ServiceExt;

    fn test_state() -> AppState {
        AppState::new(AppConfig {
            runtime_mode: RuntimeMode::Vps,
            bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
            base_url: "http://127.0.0.1:7301".to_string(),
            database_url: SecretString::from("".to_string()),
            storage_path: std::env::temp_dir().to_string_lossy().to_string(),
            cors_allowed_origins: Vec::new(),
            cookie_secure: false,
        })
    }

    #[tokio::test]
    async fn seller_private_media_permission_defaults_to_deny_without_database() {
        let pool = sqlx::PgPool::connect_lazy("postgres://example.invalid/db").unwrap();
        let actor = AuthenticatedActor {
            user_id: uuid::Uuid::new_v4(),
            role_code: "seller".to_string(),
            role_name: "Seller".to_string(),
        };

        let result = can_read_payment_proof(&pool, &actor, uuid::Uuid::new_v4()).await;

        assert_eq!(result.unwrap_err().0, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn karyawan_private_media_permission_defaults_to_deny_without_database() {
        let pool = sqlx::PgPool::connect_lazy("postgres://example.invalid/db").unwrap();
        let actor = AuthenticatedActor {
            user_id: uuid::Uuid::new_v4(),
            role_code: "karyawan".to_string(),
            role_name: "Karyawan".to_string(),
        };

        // karyawan is brand-scoped so it hits the DB lookup and fails with 500.
        let result = can_read_payment_proof(&pool, &actor, uuid::Uuid::new_v4()).await;
        assert_eq!(result.unwrap_err().0, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn non_operator_cannot_read_payment_proof_without_db_lookup() {
        let pool = sqlx::PgPool::connect_lazy("postgres://example.invalid/db").unwrap();
        let actor = AuthenticatedActor {
            user_id: uuid::Uuid::new_v4(),
            role_code: "customer".to_string(),
            role_name: "Customer".to_string(),
        };

        assert!(!can_read_payment_proof(&pool, &actor, uuid::Uuid::new_v4())
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn non_operator_cannot_decide_payment_proof_without_db_lookup() {
        let pool = sqlx::PgPool::connect_lazy("postgres://example.invalid/db").unwrap();
        let actor = AuthenticatedActor {
            user_id: uuid::Uuid::new_v4(),
            role_code: "customer".to_string(),
            role_name: "Customer".to_string(),
        };

        assert!(!can_decide_payment_proof(
            &pool,
            &actor,
            uuid::Uuid::new_v4(),
            core_db::PaymentProofDecision::Verified,
        )
        .await
        .unwrap());
    }

    #[test]
    fn payment_proof_rate_limit_key_is_route_specific() {
        let headers = HeaderMap::new();
        let key = payment_proof_upload_rate_limit_key(
            SocketAddr::from(([127, 0, 0, 1], 8080)),
            &headers,
            "token-xyz",
        );
        assert!(key.contains("payment_proof_upload:"));
        assert!(key.ends_with(":token-xyz"));
    }

    #[tokio::test]
    async fn payment_proof_verify_requires_csrf_before_session_authz() {
        let proof_id = uuid::Uuid::new_v4();
        let response = app_router(test_state())
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(format!("/api/admin/payment-proofs/{proof_id}/verify"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}
