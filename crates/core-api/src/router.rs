use crate::routes::{admin, auth, health, install, media, packages, storefront, system};
use axum::{
    http::{header, HeaderValue, Method},
    routing::get,
    Router,
};
use core_app::AppState;
use tower_http::cors::{AllowCredentials, AllowOrigin, CorsLayer};
use tower_http::{
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

// Keep a permissive global cap for multipart routes, then tighten sensitive endpoints with
// route-specific RequestBodyLimit layers.
const GLOBAL_MAX_BODY_BYTES: usize = 8 * 1024 * 1024;

pub fn app_router(state: AppState) -> Router {
    let cors = cors_layer(&state);

    Router::new()
        .route("/health", get(health::health))
        .route("/ready", get(health::ready))
        .route("/version", get(health::version))
        .nest("/api/admin", admin::router())
        .nest("/api/auth", auth::router())
        .nest("/api/install", install::router())
        .nest("/api/storefront", storefront::router())
        .nest("/api/system", system::router())
        .nest("/api/packages", packages::router())
        .merge(media::router())
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::REFERRER_POLICY,
            HeaderValue::from_static("same-origin"),
        ))
        .layer(RequestBodyLimitLayer::new(GLOBAL_MAX_BODY_BYTES))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .with_state(state)
}

fn cors_layer(state: &AppState) -> CorsLayer {
    let allowed_origins = sanitized_cors_origins(&state.config.cors_allowed_origins);

    // Credentialed CORS must never use wildcard origins. An empty allowlist denies browser CORS.
    let allow_origin = if allowed_origins.is_empty() {
        AllowOrigin::list(Vec::<HeaderValue>::new())
    } else {
        AllowOrigin::list(allowed_origins)
    };

    CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers([
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::AUTHORIZATION,
            http::HeaderName::from_static("x-csrf-token"),
        ])
        .allow_credentials(AllowCredentials::yes())
}

fn sanitized_cors_origins(origins: &[String]) -> Vec<HeaderValue> {
    origins
        .iter()
        .filter_map(|origin| {
            let origin = origin.trim();
            if origin == "*" {
                tracing::warn!("wildcard CORS origin ignored because credentials are enabled");
                return None;
            }
            origin.parse::<HeaderValue>().ok()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_runtime::{AppConfig, RuntimeMode};
    use secrecy::SecretString;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    fn test_state(cors_allowed_origins: Vec<String>, cookie_secure: bool) -> AppState {
        AppState::new(AppConfig {
            runtime_mode: RuntimeMode::Vps,
            bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
            base_url: "http://127.0.0.1:7301".to_string(),
            database_url: SecretString::from("".to_string()),
            storage_path: std::env::temp_dir().to_string_lossy().to_string(),
            cors_allowed_origins,
            cookie_secure,
        })
    }

    #[test]
    fn cors_layer_uses_allowlist_not_wildcard_with_credentials() {
        let state = test_state(vec!["https://market.example".to_string()], true);
        let _router = app_router(state);
    }

    #[test]
    fn cors_layer_builds_with_empty_allowlist_to_deny_browser_cors() {
        let state = test_state(Vec::new(), false);
        let _router = app_router(state);
    }

    #[test]
    fn cors_sanitizer_rejects_wildcard_with_credentials() {
        let origins =
            sanitized_cors_origins(&["*".to_string(), "https://market.example".to_string()]);
        assert_eq!(
            origins,
            vec![HeaderValue::from_static("https://market.example")]
        );
    }
}
