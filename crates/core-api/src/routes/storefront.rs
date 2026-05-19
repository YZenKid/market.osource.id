use axum::{
    extract::{ConnectInfo, Path, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    routing::{delete, get, post},
    Json, Router,
};
use core_app::AppState;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::limit::RequestBodyLimitLayer;

use crate::authz::read_cookie;
use crate::routes::auth::client_identity;

const CHECKOUT_MAX_BODY_BYTES: usize = 64 * 1024;
const CART_COOKIE_NAME: &str = "market_guest_cart";
const CART_COOKIE_MAX_AGE_SECONDS: i64 = 60 * 60 * 24 * 90; // 90 days

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
        .route("/cart", get(get_cart).post(upsert_cart_item).delete(clear_cart))
        .route("/cart/items/{variant_id}", delete(delete_cart_item))
}

// ── Request / Response types ──────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CheckoutRequest {
    pub customer_name: String,
    pub customer_contact: String,
    pub shipping_address: String,
    pub items: Vec<core_db::CheckoutItemInput>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertCartItemRequest {
    pub product_id: uuid::Uuid,
    pub variant_id: uuid::Uuid,
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
struct ProductsResponse {
    products: Vec<core_db::ProductWithVariantsRecord>,
}

#[derive(Debug, Serialize)]
pub struct StorefrontCartResponse {
    pub items: Vec<core_db::StorefrontCartItem>,
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

// ── Handlers ──────────────────────────────────────────────────────────────────

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

async fn get_cart(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<(HeaderMap, Json<StorefrontCartResponse>), (StatusCode, Json<StorefrontErrorResponse>)>
{
    let pool = require_pool(&state)?;
    let (token, set_cookie) = resolve_guest_cart_token(&state, &headers);
    let items = core_db::list_guest_cart_items(pool, &token)
        .await
        .map_err(repo_error)?;
    Ok((cookie_headers(set_cookie), Json(StorefrontCartResponse { items })))
}

async fn upsert_cart_item(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<UpsertCartItemRequest>,
) -> Result<(HeaderMap, Json<StorefrontCartResponse>), (StatusCode, Json<StorefrontErrorResponse>)>
{
    let pool = require_pool(&state)?;
    let (token, set_cookie) = resolve_guest_cart_token(&state, &headers);

    let cart = core_db::create_guest_cart(pool, token.clone())
        .await
        .map_err(repo_error)?;

    core_db::upsert_guest_cart_item(pool, cart.id, body.product_id, body.variant_id, body.quantity)
        .await
        .map_err(repo_error)?;

    let items = core_db::list_guest_cart_items(pool, &token)
        .await
        .map_err(repo_error)?;

    Ok((cookie_headers(set_cookie), Json(StorefrontCartResponse { items })))
}

async fn delete_cart_item(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(variant_id): Path<uuid::Uuid>,
) -> Result<(HeaderMap, Json<StorefrontCartResponse>), (StatusCode, Json<StorefrontErrorResponse>)>
{
    let pool = require_pool(&state)?;
    let (token, set_cookie) = resolve_guest_cart_token(&state, &headers);

    core_db::delete_guest_cart_item(pool, &token, variant_id)
        .await
        .map_err(repo_error)?;

    let items = core_db::list_guest_cart_items(pool, &token)
        .await
        .map_err(repo_error)?;

    Ok((cookie_headers(set_cookie), Json(StorefrontCartResponse { items })))
}

async fn clear_cart(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<(HeaderMap, Json<StorefrontCartResponse>), (StatusCode, Json<StorefrontErrorResponse>)>
{
    let pool = require_pool(&state)?;
    let (token, set_cookie) = resolve_guest_cart_token(&state, &headers);

    core_db::clear_guest_cart(pool, &token)
        .await
        .map_err(repo_error)?;

    Ok((cookie_headers(set_cookie), Json(StorefrontCartResponse { items: Vec::new() })))
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

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Returns the guest cart token from the cookie, or mints a new one.
/// The second element is a `Set-Cookie` header value to send when a new token was minted.
fn resolve_guest_cart_token(
    state: &AppState,
    headers: &HeaderMap,
) -> (String, Option<HeaderValue>) {
    if let Some(token) = read_cookie(headers, CART_COOKIE_NAME) {
        return (token, None);
    }
    let token = uuid::Uuid::new_v4().to_string();
    let cookie = HeaderValue::from_str(&guest_cart_cookie_string(&token, state.config.cookie_secure))
        .expect("guest cart cookie value is generated from safe characters");
    (token, Some(cookie))
}

fn guest_cart_cookie_string(token: &str, secure: bool) -> String {
    let secure_suffix = if secure { "; Secure" } else { "" };
    format!(
        "{CART_COOKIE_NAME}={token}; HttpOnly; SameSite=Lax; Path=/; Max-Age={CART_COOKIE_MAX_AGE_SECONDS}{secure_suffix}"
    )
}

fn cookie_headers(set_cookie: Option<HeaderValue>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    if let Some(value) = set_cookie {
        headers.insert(header::SET_COOKIE, value);
    }
    headers
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

// ── Tests ─────────────────────────────────────────────────────────────────────

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

    #[test]
    fn guest_cart_cookie_contains_required_attributes() {
        let cookie = guest_cart_cookie_string("test-token-123", false);
        assert!(cookie.starts_with("market_guest_cart=test-token-123;"));
        assert!(cookie.contains("HttpOnly"));
        assert!(cookie.contains("SameSite=Lax"));
        assert!(cookie.contains("Path=/"));
        assert!(!cookie.contains("Secure"));
    }

    #[test]
    fn guest_cart_cookie_includes_secure_when_configured() {
        let cookie = guest_cart_cookie_string("test-token-123", true);
        assert!(cookie.contains("Secure"));
    }

    #[test]
    fn resolve_guest_cart_token_reuses_existing_cookie() {
        use core_runtime::{AppConfig, RuntimeMode};
        use secrecy::SecretString;
        use std::net::{IpAddr, Ipv4Addr};

        let state = AppState::new(AppConfig {
            runtime_mode: RuntimeMode::Vps,
            bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
            base_url: "http://127.0.0.1:8301".to_string(),
            database_url: SecretString::from("".to_string()),
            storage_path: std::env::temp_dir().to_string_lossy().to_string(),
            cors_allowed_origins: Vec::new(),
            cookie_secure: false,
        });

        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("market_guest_cart=existing-token-abc"),
        );

        let (token, set_cookie) = resolve_guest_cart_token(&state, &headers);
        assert_eq!(token, "existing-token-abc");
        assert!(set_cookie.is_none(), "should not set a new cookie when one already exists");
    }

    #[test]
    fn resolve_guest_cart_token_mints_new_token_when_missing() {
        use core_runtime::{AppConfig, RuntimeMode};
        use secrecy::SecretString;
        use std::net::{IpAddr, Ipv4Addr};

        let state = AppState::new(AppConfig {
            runtime_mode: RuntimeMode::Vps,
            bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
            base_url: "http://127.0.0.1:8301".to_string(),
            database_url: SecretString::from("".to_string()),
            storage_path: std::env::temp_dir().to_string_lossy().to_string(),
            cors_allowed_origins: Vec::new(),
            cookie_secure: false,
        });

        let (token, set_cookie) = resolve_guest_cart_token(&state, &HeaderMap::new());
        assert!(!token.is_empty());
        assert!(set_cookie.is_some(), "should set a new cookie when none exists");
    }
}
