use axum::{extract::State, routing::get, Json, Router};
use core_app::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/runtime", get(runtime))
        .route("/tunnel", get(tunnel))
}

async fn runtime(State(state): State<AppState>) -> Json<serde_json::Value> {
    let storage_ok = std::path::Path::new(&state.config.storage_path).exists();
    Json(serde_json::json!({
        "config": state.config.safe_summary(),
        "backend": {"ok": true},
        "database": {"ok": false, "message": "not connected in skeleton"},
        "storage": {"ok": storage_ok, "path": state.config.storage_path},
        "install_state": "unconfigured",
        "note": "skeleton response; DB/install services are wired in later phases"
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
