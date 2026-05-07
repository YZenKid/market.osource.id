use axum::{routing::get, Json, Router};
use core_app::AppState;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct InstallStateResponse {
    pub state: &'static str,
    pub locked: bool,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/state", get(state))
        .route("/preflight", get(preflight))
}

async fn state() -> Json<InstallStateResponse> {
    Json(InstallStateResponse {
        state: "unconfigured",
        locked: false,
    })
}

async fn preflight() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "ok": true,
        "checks": [
            {"name": "backend", "ok": true},
            {"name": "storage", "ok": true}
        ]
    }))
}
