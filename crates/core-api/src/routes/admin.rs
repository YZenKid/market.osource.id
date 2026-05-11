use crate::{
    authz::{require_super_admin, AuthenticatedActor},
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

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/brands", get(list_brands).post(create_brand))
        .route("/products", get(list_products).post(create_product))
        .route("/packages", get(list_packages))
        .route("/diagnostics/export", get(diagnostics::export))
        .route("/brand-members", axum::routing::post(create_brand_member))
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
    let brands = match actor.role_code.as_str() {
        "super_admin" => core_db::list_brands(pool).await.map_err(repo_error)?,
        "seller" => core_db::list_brands_for_user(pool, actor.user_id)
            .await
            .map_err(repo_error)?,
        _ => {
            return Err(forbidden(
                "seller or super admin access is required".to_string(),
            ))
        }
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
    require_super_admin(&actor)?;
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
    let products = match actor.role_code.as_str() {
        "super_admin" => core_db::list_admin_products(pool)
            .await
            .map_err(repo_error)?,
        "seller" => core_db::list_admin_products_for_user(pool, actor.user_id)
            .await
            .map_err(repo_error)?,
        _ => {
            return Err(forbidden(
                "seller or super admin access is required".to_string(),
            ))
        }
    };
    Ok(Json(ProductsResponse {
        products,
        gate_e_note:
            "admin product reads now allow seller brand scope; mutation remains restricted by role and brand membership.",
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
    match actor.role_code.as_str() {
        "super_admin" => {}
        "seller" => {
            let assigned = core_db::user_is_brand_member(pool, actor.user_id, body.brand_id)
                .await
                .map_err(repo_error)?;
            if !assigned {
                return Err(forbidden(
                    "seller can only create products for assigned brands".to_string(),
                ));
            }
        }
        _ => {
            return Err(forbidden(
                "seller or super admin access is required".to_string(),
            ))
        }
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
}
