use axum::{
    extract::ConnectInfo,
    http::{header, HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use core_app::AppState;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::SocketAddr;

use crate::{
    authz::read_cookie,
    routes::auth::{client_identity, create_session_cookie},
};

#[derive(Debug, Serialize)]
pub struct InstallStateResponse {
    pub state: String,
    pub locked: bool,
    pub installed: bool,
    pub runtime_mode: Option<String>,
    pub core_version: Option<String>,
    pub database_connected: bool,
}

#[derive(Debug, Deserialize)]
pub struct SetupRequestBody {
    pub marketplace_name: String,
    pub admin_name: String,
    pub admin_email: String,
    pub admin_password: secrecy::SecretString,
}

#[derive(Debug, Serialize)]
pub struct SetupResponse {
    pub state: String,
    pub locked: bool,
    pub authenticated: bool,
}

#[derive(Debug, Serialize)]
pub struct InstallErrorResponse {
    pub error: &'static str,
    pub message: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/state", get(state))
        .route("/preflight", get(preflight))
        .route("/setup", post(setup))
}

async fn state(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Json<InstallStateResponse> {
    let Some(pool) = &state.db_pool else {
        return Json(InstallStateResponse {
            state: "database_unavailable".to_string(),
            locked: false,
            installed: false,
            runtime_mode: None,
            core_version: None,
            database_connected: false,
        });
    };

    match core_db::get_installation_state(pool).await {
        Ok(Some(record)) => Json(InstallStateResponse {
            state: record.state,
            locked: record.locked,
            installed: record.installed,
            runtime_mode: Some(record.runtime_mode),
            core_version: Some(record.core_version),
            database_connected: true,
        }),
        Ok(None) => Json(InstallStateResponse {
            state: "unconfigured".to_string(),
            locked: false,
            installed: false,
            runtime_mode: None,
            core_version: None,
            database_connected: true,
        }),
        Err(error) => {
            tracing::warn!(error = %error, "failed to read installation state");
            Json(InstallStateResponse {
                state: "unavailable".to_string(),
                locked: false,
                installed: false,
                runtime_mode: None,
                core_version: None,
                database_connected: true,
            })
        }
    }
}

async fn preflight(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Json<core_installer::PreflightReport> {
    let db_connected = state.db_pool.is_some();
    let migrations_ready = if let Some(pool) = &state.db_pool {
        core_db::migration_table_ready(pool).await
    } else {
        false
    };
    let storage_ready = state.storage.ensure_root().await.is_ok();

    Json(core_installer::install_preflight(
        db_connected,
        migrations_ready,
        storage_ready,
    ))
}

async fn setup(
    axum::extract::State(state): axum::extract::State<AppState>,
    ConnectInfo(peer_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(body): Json<SetupRequestBody>,
) -> Result<(HeaderMap, Json<SetupResponse>), (StatusCode, Json<InstallErrorResponse>)> {
    require_csrf(&headers)?;
    // TODO: Replace this single-process limiter with a distributed, proxy-aware limiter once
    // reverse proxy trusted headers and multi-node deployment are wired.
    if !state
        .rate_limiter
        .check(format!(
            "install_setup:{}",
            client_identity(peer_addr, &headers)
        ))
        .allowed
    {
        audit_install_event(
            state.db_pool.as_ref(),
            "failure",
            &[("reason", Value::String("rate_limited".to_string()))],
        )
        .await;
        return Err(install_error(
            StatusCode::TOO_MANY_REQUESTS,
            "rate_limited",
            "too many attempts; try again later".to_string(),
        ));
    }

    let Some(pool) = &state.db_pool else {
        return Err(install_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "database_unavailable",
            "database is required before setup".to_string(),
        ));
    };

    let request = core_installer::SetupRequest {
        marketplace_name: body.marketplace_name,
        admin_name: body.admin_name,
        admin_email: body.admin_email,
        admin_password: body.admin_password,
    };
    let validated = match core_installer::validate_setup_request(&request) {
        Ok(validated) => validated,
        Err(error) => {
            audit_install_event(
                Some(pool),
                "failure",
                &[("reason", Value::String("invalid_setup_request".to_string()))],
            )
            .await;
            return Err(install_error(
                StatusCode::BAD_REQUEST,
                "invalid_setup_request",
                error.to_string(),
            ));
        }
    };

    let password_hash = match core_auth::hash_password(&request.admin_password) {
        Ok(hash) => hash,
        Err(error) => {
            audit_install_event(
                Some(pool),
                "failure",
                &[("reason", Value::String("password_hash_failed".to_string()))],
            )
            .await;
            return Err(install_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "password_hash_failed",
                error.to_string(),
            ));
        }
    };

    let bootstrap = core_db::bootstrap_installation(
        pool,
        core_db::BootstrapInstallInput {
            marketplace_name: validated.marketplace_name,
            base_url: state.config.base_url.clone(),
            admin_name: validated.admin_name,
            admin_email: validated.admin_email,
            admin_password_hash: password_hash,
            runtime_mode: state.config.runtime_mode.to_string(),
            core_version: env!("CARGO_PKG_VERSION").to_string(),
            storage_path: state.config.storage_path.clone(),
        },
    )
    .await;

    let bootstrap = match bootstrap {
        Ok(bootstrap) => bootstrap,
        Err(core_db::InstallRepositoryError::AlreadyLocked) => {
            audit_install_event(
                Some(pool),
                "failure",
                &[("reason", Value::String("already_locked".to_string()))],
            )
            .await;
            return Err(install_error(
                StatusCode::CONFLICT,
                "already_locked",
                "installation is already locked".to_string(),
            ));
        }
        Err(core_db::InstallRepositoryError::Database(error)) => {
            tracing::warn!(error = %error, "install setup failed");
            audit_install_event(
                Some(pool),
                "failure",
                &[("reason", Value::String("setup_failed".to_string()))],
            )
            .await;
            return Err(install_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "setup_failed",
                "setup failed; check backend logs for details".to_string(),
            ));
        }
    };

    let created_session = create_session_cookie(
        pool,
        bootstrap.admin_user_id,
        state.config.cookie_secure,
    )
    .await
    .map_err(|error| {
        tracing::warn!(error = %error, "failed to create setup session");
        install_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "session_create_failed",
            "setup completed, but sign-in session could not be created; use login with the Super Admin account".to_string(),
        )
    })?;

    audit_install_event(
        Some(pool),
        "success",
        &[
            ("locked", Value::Bool(bootstrap.installation.locked)),
            ("session_created", Value::Bool(true)),
        ],
    )
    .await;

    let mut response_headers = HeaderMap::new();
    response_headers.insert(header::SET_COOKIE, created_session.header_value);

    Ok((
        response_headers,
        Json(SetupResponse {
            state: bootstrap.installation.state,
            locked: bootstrap.installation.locked,
            authenticated: true,
        }),
    ))
}

fn install_error(
    status: StatusCode,
    error: &'static str,
    message: String,
) -> (StatusCode, Json<InstallErrorResponse>) {
    (status, Json(InstallErrorResponse { error, message }))
}

fn require_csrf(headers: &HeaderMap) -> Result<(), (StatusCode, Json<InstallErrorResponse>)> {
    let cookie_token = read_cookie(headers, core_auth::CSRF_COOKIE_NAME).ok_or_else(csrf_error)?;
    let header_token = headers
        .get(core_auth::CSRF_HEADER_NAME)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(csrf_error)?;

    if core_auth::csrf_tokens_match(&cookie_token, header_token) {
        Ok(())
    } else {
        Err(csrf_error())
    }
}

fn csrf_error() -> (StatusCode, Json<InstallErrorResponse>) {
    install_error(
        StatusCode::FORBIDDEN,
        "csrf_token_invalid",
        "a valid CSRF cookie and X-CSRF-Token header are required".to_string(),
    )
}

async fn audit_install_event(
    pool: Option<&sqlx::PgPool>,
    result: &str,
    metadata_entries: &[(&str, Value)],
) {
    let Some(pool) = pool else {
        return;
    };
    let metadata = match core_db::safe_audit_metadata(metadata_entries) {
        Ok(metadata) => metadata,
        Err(error) => {
            tracing::warn!(error = %error, "install audit metadata rejected");
            serde_json::json!({})
        }
    };

    if let Err(error) = core_db::write_audit_event(
        pool,
        core_db::AuditEventInput {
            actor_user_id: None,
            action: "install.setup".to_string(),
            target_type: "installation_state".to_string(),
            target_id: None,
            result: result.to_string(),
            metadata,
        },
    )
    .await
    {
        tracing::warn!(error = %error, "failed to write install setup audit event");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{header, HeaderValue};

    #[test]
    fn setup_csrf_validation_accepts_matching_cookie_and_header() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("market_csrf=token-123"),
        );
        headers.insert(
            core_auth::CSRF_HEADER_NAME,
            HeaderValue::from_static("token-123"),
        );

        assert!(require_csrf(&headers).is_ok());
    }

    #[test]
    fn setup_csrf_validation_rejects_mismatch() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("market_csrf=token-123"),
        );
        headers.insert(
            core_auth::CSRF_HEADER_NAME,
            HeaderValue::from_static("token-456"),
        );

        assert_eq!(require_csrf(&headers).unwrap_err().0, StatusCode::FORBIDDEN);
    }

    #[test]
    fn setup_response_can_mark_created_session_authenticated() {
        let response = SetupResponse {
            state: "locked".to_string(),
            locked: true,
            authenticated: true,
        };

        let json = serde_json::to_value(response).expect("setup response serializes");
        assert_eq!(json["state"], "locked");
        assert_eq!(json["locked"], true);
        assert_eq!(json["authenticated"], true);
    }
}
