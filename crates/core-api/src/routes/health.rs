use axum::{extract::State, Json};
use core_app::AppState;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub probe: &'static str,
    pub app: &'static str,
    pub version: &'static str,
    pub git_sha: Option<&'static str>,
    pub runtime_mode: String,
    pub uptime_seconds: u64,
    pub note: &'static str,
}

#[derive(Debug, Serialize)]
pub struct ReadyResponse {
    pub status: &'static str,
    pub probe: &'static str,
    pub blocking_checks: usize,
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
    pub runtime_mode: String,
    pub build_profile: &'static str,
}

pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy",
        probe: "liveness",
        app: "market.osource.id",
        version: env!("CARGO_PKG_VERSION"),
        git_sha: option_env!("GIT_SHA"),
        runtime_mode: state.config.runtime_mode.to_string(),
        uptime_seconds: state
            .started_at
            .elapsed()
            .map(|value| value.as_secs())
            .unwrap_or(0),
        note: "liveness confirms the process is serving HTTP; use /ready for dependency and install-state readiness",
    })
}

pub async fn ready(State(state): State<AppState>) -> Json<ReadyResponse> {
    let report = state.readiness().await;
    let blocking_checks = report.checks.iter().filter(|check| !check.ok).count();
    Json(ReadyResponse {
        status: report.status,
        probe: "readiness",
        blocking_checks,
        checks: report
            .checks
            .into_iter()
            .map(|check| ReadyCheck {
                name: check.name,
                ok: check.ok,
                message: check.message,
            })
            .collect(),
    })
}

pub async fn version(State(state): State<AppState>) -> Json<VersionResponse> {
    Json(VersionResponse {
        app: "market.osource.id",
        version: env!("CARGO_PKG_VERSION"),
        git_sha: option_env!("GIT_SHA"),
        runtime_mode: state.config.runtime_mode.to_string(),
        build_profile: option_env!("PROFILE").unwrap_or("unknown"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_runtime::{AppConfig, RuntimeMode};
    use secrecy::SecretString;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    fn test_state() -> AppState {
        AppState::new(AppConfig {
            runtime_mode: RuntimeMode::Vps,
            bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
            base_url: "http://127.0.0.1:8301".to_string(),
            database_url: SecretString::from("".to_string()),
            storage_path: std::env::temp_dir().to_string_lossy().to_string(),
            cors_allowed_origins: Vec::new(),
            cookie_secure: false,
        })
    }

    #[tokio::test]
    async fn health_reports_liveness_metadata() {
        let response = health(State(test_state())).await.0;

        assert_eq!(response.status, "healthy");
        assert_eq!(response.probe, "liveness");
        assert_eq!(response.app, "market.osource.id");
        assert_eq!(response.runtime_mode, "vps");
    }

    #[tokio::test]
    async fn ready_reports_blocking_checks_when_not_ready() {
        let response = ready(State(test_state())).await.0;

        assert_eq!(response.probe, "readiness");
        assert!(response.blocking_checks >= 1);
    }

    #[tokio::test]
    async fn version_reports_runtime_identity() {
        let response = version(State(test_state())).await.0;

        assert_eq!(response.app, "market.osource.id");
        assert_eq!(response.runtime_mode, "vps");
    }
}
