use crate::{
    authz::{is_brand_scoped, is_full_access, require_admin_or_super, require_super_admin, AuthenticatedActor},
    diagnostics,
    routes::auth::{require_csrf, AuthErrorResponse},
};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::get,
    Json, Router,
};
use core_app::AppState;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/brands", get(list_brands).post(create_brand))
        .route("/products", get(list_products).post(create_product))
        .route("/orders", get(list_orders))
        .route("/payment-proofs", get(list_payment_proofs))
        .route("/users", get(list_users).post(create_operator))
        .route("/packages", get(list_packages))
        .route("/dashboard/summary", get(dashboard_summary))
        .route("/demo/seed", axum::routing::post(seed_demo_data))
        .route("/demo/clear", axum::routing::delete(clear_demo_data))
        .route("/install/reset", axum::routing::post(reset_install))
        .route("/diagnostics/export", get(diagnostics::export))
        .route("/brand-members", axum::routing::post(create_brand_member))
        .route("/brand-member-permissions", axum::routing::post(grant_brand_member_permission))
        .route("/order-brand-groups/{group_id}/fulfillment", axum::routing::post(update_fulfillment))
}

#[derive(Debug, Deserialize)]
pub struct CreateBrandRequest {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub brand_id: uuid::Uuid,
    pub category_id: Option<uuid::Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub default_variant: CreateDefaultVariantRequest,
}

#[derive(Debug, Deserialize)]
pub struct CreateDefaultVariantRequest {
    pub sku: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<serde_json::Value>,
    pub price: String,
    pub stock: i32,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBrandMemberRequest {
    pub brand_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub member_role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSellerRequest {
    pub name: String,
    pub email: String,
    pub password: secrecy::SecretString,
}

/// Unified operator creation request — supports seller, karyawan, admin.
/// `role` must be one of: "seller", "karyawan", "admin".
/// Only super_admin can create admin; admin can create seller/karyawan.
#[derive(Debug, Deserialize)]
pub struct CreateOperatorRequest {
    pub name: String,
    pub email: String,
    pub password: secrecy::SecretString,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResetInstallRequest {
    /// Current super_admin password for double-confirm.
    pub password: secrecy::SecretString,
    /// Must be true to proceed.
    pub confirmed: bool,
}

#[derive(Debug, Serialize)]
pub struct DashboardSummaryResponse {
    pub brands_active: i64,
    pub products_published: i64,
    pub orders_open: i64,
    pub payments_pending_verification: i64,
    pub scope: &'static str,
}

#[derive(Debug, Serialize)]
pub struct ResetInstallResponse {
    pub reset: bool,
    pub message: &'static str,
}

#[derive(Debug, Deserialize)]
pub struct GrantBrandMemberPermissionRequest {
    pub brand_member_id: uuid::Uuid,
    pub permission_code: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFulfillmentRequest {
    pub fulfillment_status: String,
    pub note: Option<String>,
}

#[derive(Debug, Serialize)]
struct BrandsResponse {
    brands: Vec<core_db::BrandRecord>,
}

#[derive(Debug, Serialize)]
struct ProductsResponse {
    products: Vec<core_db::ProductWithVariantsRecord>,
    gate_e_note: &'static str,
}

#[derive(Debug, Serialize)]
struct OrdersResponse {
    orders: Vec<core_db::AdminOrderRecord>,
}

#[derive(Debug, Serialize)]
struct UsersResponse {
    users: Vec<core_db::UserSummaryRecord>,
}

#[derive(Debug, Serialize)]
struct PaymentProofsResponse {
    payment_proofs: Vec<PaymentProofSummaryResponse>,
}

#[derive(Debug, Serialize)]
struct PaymentProofSummaryResponse {
    id: uuid::Uuid,
    order_id: uuid::Uuid,
    file_object_id: uuid::Uuid,
    status: String,
    created_at: chrono::DateTime<chrono::Utc>,
    can_view_file: bool,
    can_verify: bool,
    can_reject: bool,
}

#[derive(Debug, Serialize)]
struct BrandMemberResponse {
    id: uuid::Uuid,
    brand_id: uuid::Uuid,
    user_id: uuid::Uuid,
    member_role: String,
}

#[derive(Debug, Serialize)]
struct AdminPackagesResponse {
    packages: Vec<core_db::PackageStatusRecord>,
    degraded: bool,
}

async fn list_brands(
    State(state): State<AppState>,
    actor: AuthenticatedActor,
) -> Result<Json<BrandsResponse>, (StatusCode, Json<AuthErrorResponse>)> {
    let pool = require_pool(&state)?;
    let brands = if is_full_access(&actor) {
        core_db::list_brands(pool).await.map_err(repo_error)?
    } else if is_brand_scoped(&actor) {
        core_db::list_brands_for_user(pool, actor.user_id)
            .await
            .map_err(repo_error)?
    } else {
        return Err(forbidden("operator access is required".to_string()));
    };
    Ok(Json(BrandsResponse { brands }))
}

async fn create_brand(
    State(state): State<AppState>,
    headers: HeaderMap,
    actor: AuthenticatedActor,
    Json(body): Json<CreateBrandRequest>,
) -> Result<(StatusCode, Json<core_db::BrandRecord>), (StatusCode, Json<AuthErrorResponse>)> {
    require_csrf(&headers)?;
    require_admin_or_super(&actor)?;
    let pool = require_pool(&state)?;
    let brand = core_db::create_brand(
        pool,
        core_db::CreateBrandInput {
            name: body.name,
            slug: body.slug,
            description: body.description,
            status: body.status,
        },
    )
    .await
    .map_err(repo_error)?;

    Ok((StatusCode::CREATED, Json(brand)))
}

async fn list_products(
    State(state): State<AppState>,
    actor: AuthenticatedActor,
) -> Result<Json<ProductsResponse>, (StatusCode, Json<AuthErrorResponse>)> {
    let pool = require_pool(&state)?;
    let products = if is_full_access(&actor) {
        core_db::list_admin_products(pool)
            .await
            .map_err(repo_error)?
    } else if is_brand_scoped(&actor) {
        core_db::list_admin_products_for_user(pool, actor.user_id)
            .await
            .map_err(repo_error)?
    } else {
        return Err(forbidden("operator access is required".to_string()));
    };
    Ok(Json(ProductsResponse {
        products,
        gate_e_note:
            "admin product reads now allow admin cross-brand scope and seller/karyawan brand scope; mutation remains backend authoritative.",
    }))
}

async fn list_packages(
    State(state): State<AppState>,
    actor: AuthenticatedActor,
) -> Result<Json<AdminPackagesResponse>, (StatusCode, Json<AuthErrorResponse>)> {
    require_super_admin(&actor)?;
    let pool = require_pool(&state)?;
    core_db::sync_static_package_registry(pool, state.package_registry.as_ref())
        .await
        .map_err(package_repo_error)?;
    let packages = core_db::list_package_statuses(pool, state.package_registry.as_ref())
        .await
        .map_err(package_repo_error)?;

    Ok(Json(AdminPackagesResponse {
        packages,
        degraded: false,
    }))
}

async fn list_orders(
    State(state): State<AppState>,
    actor: AuthenticatedActor,
) -> Result<Json<OrdersResponse>, (StatusCode, Json<AuthErrorResponse>)> {
    let pool = require_pool(&state)?;
    let orders = if is_full_access(&actor) {
        core_db::list_admin_orders(pool).await.map_err(repo_error)?
    } else if is_brand_scoped(&actor) {
        core_db::list_admin_orders_for_user(pool, actor.user_id)
            .await
            .map_err(repo_error)?
    } else {
        return Err(forbidden("operator access is required".to_string()));
    };

    Ok(Json(OrdersResponse { orders }))
}

async fn list_users(
    State(state): State<AppState>,
    actor: AuthenticatedActor,
) -> Result<Json<UsersResponse>, (StatusCode, Json<AuthErrorResponse>)> {
    require_super_admin(&actor)?;
    let pool = require_pool(&state)?;
    let users = core_db::list_active_users_with_roles(pool)
        .await
        .map_err(auth_repo_error)?;
    Ok(Json(UsersResponse { users }))
}

async fn create_operator(
    State(state): State<AppState>,
    headers: HeaderMap,
    actor: AuthenticatedActor,
    Json(body): Json<CreateOperatorRequest>,
) -> Result<(StatusCode, Json<core_db::UserSummaryRecord>), (StatusCode, Json<AuthErrorResponse>)> {
    require_csrf(&headers)?;
    let pool = require_pool(&state)?;

    // Determine target role. Default to "seller" when not specified.
    let target_role = body.role.as_deref().unwrap_or("seller");

    // Only super_admin can create admin accounts.
    // admin can create seller and karyawan.
    match target_role {
        "admin" => require_super_admin(&actor)?,
        "seller" | "karyawan" => require_admin_or_super(&actor)?,
        _ => {
            return Err(api_error(
                StatusCode::BAD_REQUEST,
                "invalid_role",
                format!("role must be one of: seller, karyawan, admin; got `{target_role}`"),
            ))
        }
    }

    let password_hash = core_auth::hash_password(&body.password).map_err(|error| {
        api_error(
            StatusCode::BAD_REQUEST,
            "password_hash_failed",
            error.to_string(),
        )
    })?;

    let user = core_db::create_user_with_role(
        pool,
        core_db::CreateUserWithRoleInput {
            name: body.name,
            email: body.email.trim().to_ascii_lowercase(),
            password_hash,
            role_code: target_role.to_string(),
        },
    )
    .await
    .map_err(auth_repo_error)?;

    Ok((StatusCode::CREATED, Json(user)))
}

async fn list_payment_proofs(
    State(state): State<AppState>,
    actor: AuthenticatedActor,
) -> Result<Json<PaymentProofsResponse>, (StatusCode, Json<AuthErrorResponse>)> {
    let pool = require_pool(&state)?;
    let payment_proofs = if is_full_access(&actor) {
        core_db::list_payment_proofs(pool)
            .await
            .map_err(media_repo_error)?
            .into_iter()
            .map(|proof| PaymentProofSummaryResponse {
                id: proof.id,
                order_id: proof.order_id,
                file_object_id: proof.file_object_id,
                status: proof.status,
                created_at: proof.created_at,
                can_view_file: true,
                can_verify: true,
                can_reject: true,
            })
            .collect()
    } else if is_brand_scoped(&actor) {
        let proofs = core_db::list_payment_proofs_for_user(pool, actor.user_id)
            .await
            .map_err(media_repo_error)?;
        let mut responses = Vec::with_capacity(proofs.len());

        for proof in proofs {
            let can_view_file = core_db::user_has_brand_scoped_permission_for_order(
                pool,
                actor.user_id,
                proof.order_id,
                core_db::PAYMENT_PROOF_VIEW_ASSIGNED,
            )
            .await
            .map_err(media_repo_error)?;
            let can_verify = core_db::user_has_brand_scoped_permission_for_order(
                pool,
                actor.user_id,
                proof.order_id,
                core_db::PAYMENT_PROOF_VERIFY,
            )
            .await
            .map_err(media_repo_error)?;
            let can_reject = core_db::user_has_brand_scoped_permission_for_order(
                pool,
                actor.user_id,
                proof.order_id,
                core_db::PAYMENT_PROOF_REJECT,
            )
            .await
            .map_err(media_repo_error)?;

            responses.push(PaymentProofSummaryResponse {
                id: proof.id,
                order_id: proof.order_id,
                file_object_id: proof.file_object_id,
                status: proof.status,
                created_at: proof.created_at,
                can_view_file,
                can_verify,
                can_reject,
            });
        }

        responses
    } else {
        return Err(forbidden("operator access is required".to_string()));
    };
    Ok(Json(PaymentProofsResponse { payment_proofs }))
}

async fn grant_brand_member_permission(
    State(state): State<AppState>,
    headers: HeaderMap,
    actor: AuthenticatedActor,
    Json(body): Json<GrantBrandMemberPermissionRequest>,
) -> Result<StatusCode, (StatusCode, Json<AuthErrorResponse>)> {
    require_csrf(&headers)?;
    require_super_admin(&actor)?;
    let pool = require_pool(&state)?;
    core_db::grant_brand_member_permission(pool, body.brand_member_id, &body.permission_code, actor.user_id)
        .await
        .map_err(media_repo_error)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn update_fulfillment(
    State(state): State<AppState>,
    headers: HeaderMap,
    actor: AuthenticatedActor,
    axum::extract::Path(group_id): axum::extract::Path<uuid::Uuid>,
    Json(body): Json<UpdateFulfillmentRequest>,
) -> Result<Json<core_db::OrderBrandGroupRecord>, (StatusCode, Json<AuthErrorResponse>)> {
    require_csrf(&headers)?;
    let pool = require_pool(&state)?;
    if is_brand_scoped(&actor) {
        let allowed = core_db::seller_can_access_order_brand_group(pool, group_id, actor.user_id)
            .await
            .map_err(repo_error)?;
        if !allowed {
            return Err(forbidden("brand-scoped operator can only update assigned brand-group fulfillment".to_string()));
        }
    } else if !is_full_access(&actor) {
        return Err(forbidden("operator access is required".to_string()));
    }

    let next_status = validate_fulfillment_status(&body.fulfillment_status)?;

    let group = core_db::update_order_brand_group_fulfillment(
        pool,
        group_id,
        next_status,
        actor.user_id,
        body.note.as_deref(),
    )
    .await
    .map_err(repo_error)?;

    Ok(Json(group))
}

fn validate_fulfillment_status(
    status: &str,
) -> Result<&str, (StatusCode, Json<AuthErrorResponse>)> {
    match status {
        "not_ready" | "processing" | "ready_to_ship" | "completed" => Ok(status),
        _ => Err(api_error(
            StatusCode::BAD_REQUEST,
            "invalid_fulfillment_status",
            format!(
                "fulfillment_status must be one of: not_ready, processing, ready_to_ship, completed; got `{status}`"
            ),
        )),
    }
}

async fn create_product(
    State(state): State<AppState>,
    headers: HeaderMap,
    actor: AuthenticatedActor,
    Json(body): Json<CreateProductRequest>,
) -> Result<
    (StatusCode, Json<core_db::ProductWithVariantsRecord>),
    (StatusCode, Json<AuthErrorResponse>),
> {
    require_csrf(&headers)?;
    let pool = require_pool(&state)?;
    if is_brand_scoped(&actor) {
        let assigned = core_db::user_is_brand_member(pool, actor.user_id, body.brand_id)
            .await
            .map_err(repo_error)?;
        if !assigned {
            return Err(forbidden(
                "brand-scoped operator can only create products for assigned brands".to_string(),
            ));
        }
    } else if !is_full_access(&actor) {
        return Err(forbidden("operator access is required".to_string()));
    }
    let product = core_db::create_product_with_default_variant(
        pool,
        core_db::CreateProductInput {
            brand_id: body.brand_id,
            category_id: body.category_id,
            name: body.name,
            slug: body.slug,
            description: body.description,
            status: body.status,
            default_variant: core_db::CreateVariantInput {
                sku: body.default_variant.sku,
                name: body
                    .default_variant
                    .name
                    .unwrap_or_else(|| "Default".to_string()),
                attributes: body
                    .default_variant
                    .attributes
                    .unwrap_or_else(|| serde_json::json!({ "internal": true })),
                price: body.default_variant.price,
                stock: body.default_variant.stock,
                status: body.default_variant.status,
            },
        },
    )
    .await
    .map_err(repo_error)?;

    Ok((StatusCode::CREATED, Json(product)))
}

async fn create_brand_member(
    State(state): State<AppState>,
    headers: HeaderMap,
    actor: AuthenticatedActor,
    Json(body): Json<CreateBrandMemberRequest>,
) -> Result<(StatusCode, Json<BrandMemberResponse>), (StatusCode, Json<AuthErrorResponse>)> {
    require_csrf(&headers)?;
    require_super_admin(&actor)?;
    let pool = require_pool(&state)?;
    let member = core_db::create_brand_member(pool, body.brand_id, body.user_id, body.member_role)
        .await
        .map_err(repo_error)?;

    Ok((
        StatusCode::CREATED,
        Json(BrandMemberResponse {
            id: member.id,
            brand_id: member.brand_id,
            user_id: member.user_id,
            member_role: member.member_role,
        }),
    ))
}

async fn dashboard_summary(
    State(state): State<AppState>,
    actor: AuthenticatedActor,
) -> Result<Json<DashboardSummaryResponse>, (StatusCode, Json<AuthErrorResponse>)> {
    let pool = require_pool(&state)?;
    let summary = if is_full_access(&actor) {
        core_db::load_dashboard_summary(pool, None)
            .await
            .map_err(repo_error)?
    } else if is_brand_scoped(&actor) {
        core_db::load_dashboard_summary(pool, Some(actor.user_id))
            .await
            .map_err(repo_error)?
    } else {
        return Err(forbidden("operator access is required".to_string()));
    };

    Ok(Json(DashboardSummaryResponse {
        brands_active: summary.brands_active,
        products_published: summary.products_published,
        orders_open: summary.orders_open,
        payments_pending_verification: summary.payments_pending_verification,
        scope: if is_full_access(&actor) { "all" } else { "assigned" },
    }))
}

async fn seed_demo_data(
    State(state): State<AppState>,
    headers: HeaderMap,
    actor: AuthenticatedActor,
) -> Result<StatusCode, (StatusCode, Json<AuthErrorResponse>)> {
    require_csrf(&headers)?;
    require_super_admin(&actor)?;
    let pool = require_pool(&state)?;

    let install = core_db::get_installation_state(pool)
        .await
        .map_err(install_repo_error)?;
    if !install.as_ref().is_some_and(|record| record.locked) {
        return Err(api_error(
            StatusCode::CONFLICT,
            "installation_not_locked",
            "demo seed can only run after installation is locked".to_string(),
        ));
    }

    core_installer::seed_demo_clothing(pool)
        .await
        .map_err(installer_error)?;
    write_admin_audit_event(
        Some(pool),
        Some(actor.user_id),
        "demo.seed",
        "demo_data",
        None,
        "success",
        &[("seeded", Value::Bool(true))],
    )
    .await;

    Ok(StatusCode::NO_CONTENT)
}

async fn clear_demo_data(
    State(state): State<AppState>,
    headers: HeaderMap,
    actor: AuthenticatedActor,
) -> Result<StatusCode, (StatusCode, Json<AuthErrorResponse>)> {
    require_csrf(&headers)?;
    require_super_admin(&actor)?;
    let pool = require_pool(&state)?;

    core_installer::clear_demo_clothing(pool)
        .await
        .map_err(installer_error)?;
    write_admin_audit_event(
        Some(pool),
        Some(actor.user_id),
        "demo.clear",
        "demo_data",
        None,
        "success",
        &[("cleared", Value::Bool(true))],
    )
    .await;

    Ok(StatusCode::NO_CONTENT)
}

async fn reset_install(
    State(state): State<AppState>,
    headers: HeaderMap,
    actor: AuthenticatedActor,
    Json(body): Json<ResetInstallRequest>,
) -> Result<Json<ResetInstallResponse>, (StatusCode, Json<AuthErrorResponse>)> {
    require_csrf(&headers)?;
    require_super_admin(&actor)?;
    if !body.confirmed {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "confirmation_required",
            "confirmation checkbox is required".to_string(),
        ));
    }

    let pool = require_pool(&state)?;
    let session_user = core_db::find_active_user_by_id_with_role(pool, actor.user_id)
        .await
        .map_err(auth_repo_error)?
        .ok_or_else(|| {
            api_error(
                StatusCode::UNAUTHORIZED,
                "user_not_found",
                "current operator was not found".to_string(),
            )
        })?;

    let password_valid = core_auth::verify_password(&body.password, &session_user.password_hash)
        .map_err(|error| {
            api_error(
                StatusCode::UNAUTHORIZED,
                "invalid_credentials",
                error.to_string(),
            )
        })?;
    if !password_valid {
        write_admin_audit_event(
            Some(pool),
            Some(actor.user_id),
            "install.reset",
            "installation_state",
            None,
            "failure",
            &[("reason", Value::String("invalid_password".to_string()))],
        )
        .await;
        return Err(api_error(
            StatusCode::UNAUTHORIZED,
            "invalid_credentials",
            "invalid password".to_string(),
        ));
    }

    let success_audit_metadata = reset_install_success_audit_metadata(
        session_user.id,
        &session_user.role_code,
    );

    core_db::reset_installation_state(pool)
        .await
        .map_err(install_repo_error)?;
    write_admin_audit_event(
        Some(pool),
        None,
        "install.reset",
        "installation_state",
        None,
        "success",
        &success_audit_metadata,
    )
    .await;

    Ok(Json(ResetInstallResponse {
        reset: true,
        message: "installation reset completed",
    }))
}

fn require_pool(state: &AppState) -> Result<&sqlx::PgPool, (StatusCode, Json<AuthErrorResponse>)> {
    state.db_pool.as_ref().ok_or_else(|| {
        api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "database_unavailable",
            "database is required for commerce admin routes".to_string(),
        )
    })
}

fn repo_error(error: core_db::CommerceRepositoryError) -> (StatusCode, Json<AuthErrorResponse>) {
    tracing::warn!(error = %error, "commerce admin repository operation failed");
    api_error(
        StatusCode::BAD_REQUEST,
        "commerce_operation_failed",
        error.to_string(),
    )
}

fn package_repo_error(
    error: core_db::PackageRepositoryError,
) -> (StatusCode, Json<AuthErrorResponse>) {
    tracing::warn!(error = %error, "admin package repository operation failed");
    api_error(
        StatusCode::BAD_REQUEST,
        "package_operation_failed",
        error.to_string(),
    )
}

fn auth_repo_error(error: core_db::AuthRepositoryError) -> (StatusCode, Json<AuthErrorResponse>) {
    tracing::warn!(error = %error, "admin auth repository operation failed");
    api_error(
        StatusCode::BAD_REQUEST,
        "auth_operation_failed",
        error.to_string(),
    )
}

fn media_repo_error(error: core_db::MediaRepositoryError) -> (StatusCode, Json<AuthErrorResponse>) {
    tracing::warn!(error = %error, "admin media repository operation failed");
    api_error(
        StatusCode::BAD_REQUEST,
        "media_operation_failed",
        error.to_string(),
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

fn forbidden(message: String) -> (StatusCode, Json<AuthErrorResponse>) {
    api_error(StatusCode::FORBIDDEN, "forbidden", message)
}

fn install_repo_error(
    error: core_db::InstallRepositoryError,
) -> (StatusCode, Json<AuthErrorResponse>) {
    tracing::warn!(error = %error, "admin install repository operation failed");
    api_error(
        StatusCode::INTERNAL_SERVER_ERROR,
        "install_operation_failed",
        error.to_string(),
    )
}

fn installer_error(error: core_installer::DemoSeedError) -> (StatusCode, Json<AuthErrorResponse>) {
    tracing::warn!(error = %error, "demo seed/clear operation failed");
    api_error(
        StatusCode::INTERNAL_SERVER_ERROR,
        "demo_operation_failed",
        error.to_string(),
    )
}

fn reset_install_success_audit_metadata(
    prior_actor_user_id: uuid::Uuid,
    prior_actor_role_code: &str,
) -> Vec<(&'static str, Value)> {
    vec![
        ("confirmed", Value::Bool(true)),
        (
            "prior_actor_user_id",
            Value::String(prior_actor_user_id.to_string()),
        ),
        (
            "prior_actor_role_code",
            Value::String(prior_actor_role_code.to_string()),
        ),
    ]
}

async fn write_admin_audit_event(
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
            tracing::warn!(error = %error, action, "admin audit metadata rejected");
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
        tracing::warn!(error = %error, action, "failed to write admin audit event");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{header, HeaderMap, HeaderValue, StatusCode};

    #[test]
    fn admin_mutation_csrf_enforcement_rejects_mismatch() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("market_csrf=token-123"),
        );
        headers.insert("x-csrf-token", HeaderValue::from_static("token-456"));

        assert_eq!(require_csrf(&headers).unwrap_err().0, StatusCode::FORBIDDEN);
    }

    #[test]
    fn validate_fulfillment_status_accepts_canonical_values() {
        for status in ["not_ready", "processing", "ready_to_ship", "completed"] {
            assert_eq!(validate_fulfillment_status(status).unwrap(), status);
        }
    }

    #[test]
    fn validate_fulfillment_status_rejects_unknown_values() {
        let error = validate_fulfillment_status("anything_else").unwrap_err();

        assert_eq!(error.0, StatusCode::BAD_REQUEST);
        assert_eq!(error.1.error, "invalid_fulfillment_status");
    }

    #[test]
    fn reset_install_success_audit_metadata_captures_non_pii_actor_context() {
        let actor_user_id = uuid::Uuid::new_v4();

        let metadata = reset_install_success_audit_metadata(actor_user_id, "super_admin");
        let metadata = core_db::safe_audit_metadata(&metadata).expect("metadata should be safe");

        assert_eq!(metadata["confirmed"], true);
        assert_eq!(metadata["prior_actor_user_id"], actor_user_id.to_string());
        assert_eq!(metadata["prior_actor_role_code"], "super_admin");
        assert!(metadata.get("prior_actor_email").is_none());
    }
}
