use axum::{extract::State, routing::get, Json, Router};
use core_app::AppState;
use serde::Serialize;

use crate::authz::{require_super_admin, AuthenticatedActor};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/runtime", get(runtime))
        .route("/tunnel", get(tunnel))
}

async fn runtime(
    State(state): State<AppState>,
    actor: AuthenticatedActor,
) -> Result<
    Json<serde_json::Value>,
    (
        axum::http::StatusCode,
        Json<crate::routes::auth::AuthErrorResponse>,
    ),
> {
    require_super_admin(&actor)?;
    let readiness = state.readiness().await;
    let install_state = install_state_summary(&state).await;
    let postgres = sidecar_postgres::default_desktop_status();
    let tunnel = sidecar_cloudflared::default_tunnel_status();

    Ok(Json(serde_json::json!({
        "config": state.config.safe_summary(),
        "backend": {"ok": true},
        "readiness": readiness,
        "database": {"ok": state.db_pool.is_some()},
        "desktop_postgres": postgres,
        "storage": {"configured": !state.config.storage_path.is_empty(), "provider": "local"},
        "packages": {"registered": state.package_registry.len(), "enabled_capabilities": state.package_gate.enabled_count()},
        "install_state": install_state,
        "tunnel": tunnel,
        "operator_caution": [
            "Runtime status is read-only in this slice; process start/stop remains a later control-plane milestone.",
            "Tunnel/public exposure must stay opt-in and never expose PostgreSQL or debug ports."
        ],
        "note": "runtime status is read-only; process start/stop supervision remains a later Gate C slice"
    })))
}

async fn tunnel(
    actor: AuthenticatedActor,
) -> Result<
    Json<serde_json::Value>,
    (
        axum::http::StatusCode,
        Json<crate::routes::auth::AuthErrorResponse>,
    ),
> {
    require_super_admin(&actor)?;
    Ok(Json(serde_json::json!(
        sidecar_cloudflared::default_tunnel_status()
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::app_router;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use core_runtime::{AppConfig, RuntimeMode};
    use secrecy::SecretString;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use tower::ServiceExt;

    fn test_state() -> AppState {
        AppState::new(AppConfig {
            runtime_mode: RuntimeMode::Vps,
            bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
            base_url: "http://127.0.0.1:7301".to_string(),
            database_url: SecretString::from("".to_string()),
            storage_path: std::env::temp_dir().to_string_lossy().to_string(),
            cors_allowed_origins: Vec::new(),
            cookie_secure: false,
        })
    }

    #[tokio::test]
    async fn system_runtime_requires_session_cookie() {
        let mut state = test_state();
        state.db_pool = Some(sqlx::PgPool::connect_lazy("postgres://example.invalid/db").unwrap());
        let response = app_router(state)
            .oneshot(
                Request::builder()
                    .uri("/api/system/runtime")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct InstallStateSummary {
    ok: bool,
    state: String,
    locked: bool,
    installed: bool,
    runtime_mode: Option<String>,
    core_version: Option<String>,
    database_connected: bool,
    message: Option<String>,
}

pub(crate) async fn install_state_summary(state: &AppState) -> InstallStateSummary {
    let Some(pool) = &state.db_pool else {
        return InstallStateSummary {
            ok: false,
            state: "database_unavailable".to_string(),
            locked: false,
            installed: false,
            runtime_mode: None,
            core_version: None,
            database_connected: false,
            message: Some("database is not connected".to_string()),
        };
    };

    match core_db::get_installation_state(pool).await {
        Ok(Some(record)) => InstallStateSummary {
            ok: record.locked && record.installed,
            state: record.state,
            locked: record.locked,
            installed: record.installed,
            runtime_mode: Some(record.runtime_mode),
            core_version: Some(record.core_version),
            database_connected: true,
            message: None,
        },
        Ok(None) => InstallStateSummary {
            ok: false,
            state: "unconfigured".to_string(),
            locked: false,
            installed: false,
            runtime_mode: None,
            core_version: None,
            database_connected: true,
            message: Some("installation has not been completed".to_string()),
        },
        Err(error) => {
            tracing::warn!(error = %error, "failed to read installation state for runtime status");
            InstallStateSummary {
                ok: false,
                state: "unavailable".to_string(),
                locked: false,
                installed: false,
                runtime_mode: None,
                core_version: None,
                database_connected: true,
                message: Some("installation state is unavailable".to_string()),
            }
        }
    }
}
