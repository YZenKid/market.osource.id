use axum::{extract::State, routing::get, Json, Router};
use core_app::AppState;
use serde::Serialize;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/runtime", get(runtime))
        .route("/tunnel", get(tunnel))
}

async fn runtime(State(state): State<AppState>) -> Json<serde_json::Value> {
    let readiness = state.readiness().await;
    let install_state = install_state_summary(&state).await;
    let postgres = sidecar_postgres::default_desktop_status();
    let tunnel = sidecar_cloudflared::default_tunnel_status();

    Json(serde_json::json!({
        "config": state.config.safe_summary(),
        "backend": {"ok": true},
        "readiness": readiness,
        "database": {"ok": state.db_pool.is_some()},
        "desktop_postgres": postgres,
        "storage": {"path": state.config.storage_path},
        "packages": {"registered": state.package_registry.len(), "enabled_capabilities": state.package_gate.enabled_count()},
        "install_state": install_state,
        "tunnel": tunnel,
        "note": "runtime status is read-only; process start/stop supervision remains a later Gate C slice"
    }))
}

async fn tunnel() -> Json<serde_json::Value> {
    Json(serde_json::json!(
        sidecar_cloudflared::default_tunnel_status()
    ))
}

#[derive(Debug, Serialize)]
struct InstallStateSummary {
    ok: bool,
    state: String,
    locked: bool,
    installed: bool,
    runtime_mode: Option<String>,
    core_version: Option<String>,
    database_connected: bool,
    message: Option<String>,
}

async fn install_state_summary(state: &AppState) -> InstallStateSummary {
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
