use axum::{routing::get, Json, Router};
use core_app::AppState;
use core_packages::base_registry;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list))
}

async fn list() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "packages": base_registry(),
        "model": "remote_entitlement_ready"
    }))
}
