use axum::{
    extract::{ConnectInfo, Path, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use core_app::AppState;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::limit::RequestBodyLimitLayer;

use crate::routes::auth::client_identity;

const CHECKOUT_MAX_BODY_BYTES: usize = 64 * 1024;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/products", get(list_products))
        .route("/products/{id}", get(product_detail))
        .route(
            "/orders/{tracking_token}",
            get(order_summary_by_tracking_token),
        )
        .route(
            "/checkout",
            post(checkout).route_layer(RequestBodyLimitLayer::new(CHECKOUT_MAX_BODY_BYTES)),
        )
}

#[derive(Debug, Deserialize)]
pub struct CheckoutRequest {
    pub customer_name: String,
    pub customer_contact: String,
    pub shipping_address: String,
    pub items: Vec<core_db::CheckoutItemInput>,
}

#[derive(Debug, Serialize)]
struct ProductsResponse {
    products: Vec<core_db::ProductWithVariantsRecord>,
}

#[derive(Debug, Serialize)]
pub struct CheckoutResponse {
    pub order: core_db::OrderRecord,
}

#[derive(Debug, Serialize)]
pub struct StorefrontOrderSummaryResponse {
    pub order_number: String,
    pub payment_status: String,
    pub global_status: String,
    pub total_snapshot: String,
}

#[derive(Debug, Serialize)]
pub struct StorefrontErrorResponse {
    pub error: &'static str,
    pub message: String,
}

async fn list_products(
    State(state): State<AppState>,
) -> Result<Json<ProductsResponse>, (StatusCode, Json<StorefrontErrorResponse>)> {
    let pool = require_pool(&state)?;
    let products = core_db::list_storefront_products(pool)
        .await
        .map_err(repo_error)?;
    Ok(Json(ProductsResponse { products }))
}

async fn product_detail(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<core_db::ProductWithVariantsRecord>, (StatusCode, Json<StorefrontErrorResponse>)> {
    let pool = require_pool(&state)?;
    let product = core_db::storefront_product_detail(pool, id)
        .await
        .map_err(repo_error)?
        .ok_or_else(|| {
            storefront_error(
                StatusCode::NOT_FOUND,
                "not_found",
                "product not found".to_string(),
            )
        })?;
    Ok(Json(product))
}

async fn checkout(
    State(state): State<AppState>,
    ConnectInfo(peer_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(body): Json<CheckoutRequest>,
) -> Result<(StatusCode, Json<CheckoutResponse>), (StatusCode, Json<StorefrontErrorResponse>)> {
    // TODO: Replace this single-process limiter with a distributed, proxy-aware limiter once
    // reverse proxy trusted headers and multi-node deployment are wired.
    if !state
        .rate_limiter
        .check(checkout_rate_limit_key(peer_addr, &headers))
        .allowed
    {
        return Err(rate_limited());
    }

    let pool = require_pool(&state)?;
    let order = core_db::checkout_submitted_items(
        pool,
        core_db::CheckoutInput {
            customer_name: body.customer_name,
            customer_contact: body.customer_contact,
            shipping_address: body.shipping_address,
            items: body.items,
        },
    )
    .await
    .map_err(repo_error)?;

    Ok((StatusCode::CREATED, Json(CheckoutResponse { order })))
}

async fn order_summary_by_tracking_token(
    State(state): State<AppState>,
    ConnectInfo(peer_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Path(tracking_token): Path<String>,
) -> Result<Json<StorefrontOrderSummaryResponse>, (StatusCode, Json<StorefrontErrorResponse>)> {
    // TODO: Replace this single-process limiter with a distributed, proxy-aware limiter once
    // reverse proxy trusted headers and multi-node deployment are wired.
    if !state
        .rate_limiter
        .check(order_lookup_rate_limit_key(
            peer_addr,
            &headers,
            &tracking_token,
        ))
        .allowed
    {
        return Err(rate_limited());
    }

    let pool = require_pool(&state)?;
    let summary = core_db::find_storefront_order_summary_by_tracking_token(pool, &tracking_token)
        .await
        .map_err(repo_error)?
        .ok_or_else(|| {
            storefront_error(
                StatusCode::NOT_FOUND,
                "not_found",
                "order tracking token was not found".to_string(),
            )
        })?;

    Ok(Json(StorefrontOrderSummaryResponse {
        order_number: summary.order_number,
        payment_status: summary.payment_status,
        global_status: summary.global_status,
        total_snapshot: summary.total_snapshot,
    }))
}

fn require_pool(
    state: &AppState,
) -> Result<&sqlx::PgPool, (StatusCode, Json<StorefrontErrorResponse>)> {
    state.db_pool.as_ref().ok_or_else(|| {
        storefront_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "database_unavailable",
            "database is required for storefront routes".to_string(),
        )
    })
}

fn repo_error(
    error: core_db::CommerceRepositoryError,
) -> (StatusCode, Json<StorefrontErrorResponse>) {
    let status = match error {
        core_db::CommerceRepositoryError::EmptyCheckout
        | core_db::CommerceRepositoryError::InvalidQuantity
        | core_db::CommerceRepositoryError::VariantNotFound
        | core_db::CommerceRepositoryError::OutOfStock
        | core_db::CommerceRepositoryError::Overstock => StatusCode::BAD_REQUEST,
        core_db::CommerceRepositoryError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
    tracing::warn!(error = %error, "storefront commerce operation failed");
    storefront_error(status, "commerce_operation_failed", error.to_string())
}

fn storefront_error(
    status: StatusCode,
    error: &'static str,
    message: String,
) -> (StatusCode, Json<StorefrontErrorResponse>) {
    (status, Json(StorefrontErrorResponse { error, message }))
}

fn rate_limited() -> (StatusCode, Json<StorefrontErrorResponse>) {
    storefront_error(
        StatusCode::TOO_MANY_REQUESTS,
        "rate_limited",
        "too many attempts; try again later".to_string(),
    )
}

fn checkout_rate_limit_key(peer_addr: SocketAddr, headers: &HeaderMap) -> String {
    format!(
        "storefront_checkout:{}",
        client_identity(peer_addr, headers)
    )
}

fn order_lookup_rate_limit_key(
    peer_addr: SocketAddr,
    headers: &HeaderMap,
    tracking_token: &str,
) -> String {
    format!(
        "storefront_order_lookup:{}:{tracking_token}",
        client_identity(peer_addr, headers)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkout_rate_limit_key_is_route_specific() {
        let headers = HeaderMap::new();
        let key = checkout_rate_limit_key(SocketAddr::from(([127, 0, 0, 1], 8080)), &headers);
        assert!(key.starts_with("storefront_checkout:"));
    }

    #[test]
    fn order_lookup_rate_limit_key_includes_tracking_token() {
        let headers = HeaderMap::new();
        let key = order_lookup_rate_limit_key(
            SocketAddr::from(([127, 0, 0, 1], 8080)),
            &headers,
            "token-abc",
        );
        assert!(key.contains("storefront_order_lookup:"));
        assert!(key.ends_with(":token-abc"));
    }
}
