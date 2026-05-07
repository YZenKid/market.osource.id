use axum::{extract::State, Json};
use core_app::AppState;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

#[derive(Debug, Serialize)]
pub struct ReadyResponse {
    pub status: &'static str,
    pub checks: Vec<ReadyCheck>,
}

#[derive(Debug, Serialize)]
pub struct ReadyCheck {
    pub name: &'static str,
    pub ok: bool,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VersionResponse {
    pub app: &'static str,
    pub version: &'static str,
    pub git_sha: Option<&'static str>,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "healthy" })
}

pub async fn ready(State(state): State<AppState>) -> Json<ReadyResponse> {
    let storage_ok = std::path::Path::new(&state.config.storage_path).exists()
        || std::fs::create_dir_all(&state.config.storage_path).is_ok();
    Json(ReadyResponse {
        status: if storage_ok { "ready" } else { "not_ready" },
        checks: vec![ReadyCheck {
            name: "storage_path",
            ok: storage_ok,
            message: (!storage_ok).then(|| "storage path is not writable".to_string()),
        }],
    })
}

pub async fn version() -> Json<VersionResponse> {
    Json(VersionResponse {
        app: "market.osource.id",
        version: env!("CARGO_PKG_VERSION"),
        git_sha: option_env!("GIT_SHA"),
    })
}
