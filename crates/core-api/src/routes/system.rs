use axum::{extract::State, routing::get, Json, Router};
use core_app::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/runtime", get(runtime))
        .route("/tunnel", get(tunnel))
}

async fn runtime(State(state): State<AppState>) -> Json<serde_json::Value> {
    let readiness = state.readiness().await;
    Json(serde_json::json!({
        "config": state.config.safe_summary(),
        "backend": {"ok": true},
        "readiness": readiness,
        "database": {"ok": state.db_pool.is_some()},
        "storage": {"path": state.config.storage_path},
        "packages": {"registered": state.package_registry.len(), "enabled_capabilities": state.package_gate.enabled_count()},
        "install_state": {"ok": false, "state": "service_not_wired_until_gate_c"},
        "note": "install state remains skeleton until Gate C"
    }))
}

async fn tunnel() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "available": false,
        "enabled": false,
        "running": false,
        "public_url": null
    }))
}
