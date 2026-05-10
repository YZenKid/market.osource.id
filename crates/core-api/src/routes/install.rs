use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use core_app::AppState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct InstallStateResponse {
    pub state: String,
    pub locked: bool,
    pub installed: bool,
    pub runtime_mode: Option<String>,
    pub core_version: Option<String>,
    pub database_connected: bool,
}

#[derive(Debug, Deserialize)]
pub struct SetupRequestBody {
    pub marketplace_name: String,
    pub admin_name: String,
    pub admin_email: String,
    pub admin_password: secrecy::SecretString,
}

#[derive(Debug, Serialize)]
pub struct SetupResponse {
    pub state: String,
    pub locked: bool,
}

#[derive(Debug, Serialize)]
pub struct InstallErrorResponse {
    pub error: &'static str,
    pub message: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/state", get(state))
        .route("/preflight", get(preflight))
        .route("/setup", post(setup))
}

async fn state(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Json<InstallStateResponse> {
    let Some(pool) = &state.db_pool else {
        return Json(InstallStateResponse {
            state: "database_unavailable".to_string(),
            locked: false,
            installed: false,
            runtime_mode: None,
            core_version: None,
            database_connected: false,
        });
    };

    match core_db::get_installation_state(pool).await {
        Ok(Some(record)) => Json(InstallStateResponse {
            state: record.state,
            locked: record.locked,
            installed: record.installed,
            runtime_mode: Some(record.runtime_mode),
            core_version: Some(record.core_version),
            database_connected: true,
        }),
        Ok(None) => Json(InstallStateResponse {
            state: "unconfigured".to_string(),
            locked: false,
            installed: false,
            runtime_mode: None,
            core_version: None,
            database_connected: true,
        }),
        Err(error) => {
            tracing::warn!(error = %error, "failed to read installation state");
            Json(InstallStateResponse {
                state: "unavailable".to_string(),
                locked: false,
                installed: false,
                runtime_mode: None,
                core_version: None,
                database_connected: true,
            })
        }
    }
}

async fn preflight(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Json<core_installer::PreflightReport> {
    let db_connected = state.db_pool.is_some();
    let migrations_ready = if let Some(pool) = &state.db_pool {
        core_db::migration_table_ready(pool).await
    } else {
        false
    };
    let storage_ready = state.storage.ensure_root().await.is_ok();

    Json(core_installer::install_preflight(
        db_connected,
        migrations_ready,
        storage_ready,
    ))
}

async fn setup(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(body): Json<SetupRequestBody>,
) -> Result<Json<SetupResponse>, (StatusCode, Json<InstallErrorResponse>)> {
    let Some(pool) = &state.db_pool else {
        return Err(install_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "database_unavailable",
            "database is required before setup".to_string(),
        ));
    };

    let request = core_installer::SetupRequest {
        marketplace_name: body.marketplace_name,
        admin_name: body.admin_name,
        admin_email: body.admin_email,
        admin_password: body.admin_password,
    };
    let validated = core_installer::validate_setup_request(&request).map_err(|error| {
        install_error(
            StatusCode::BAD_REQUEST,
            "invalid_setup_request",
            error.to_string(),
        )
    })?;

    let password_hash = core_auth::hash_password(&request.admin_password).map_err(|error| {
        install_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "password_hash_failed",
            error.to_string(),
        )
    })?;

    let record = core_db::bootstrap_installation(
        pool,
        core_db::BootstrapInstallInput {
            marketplace_name: validated.marketplace_name,
            base_url: state.config.base_url.clone(),
            admin_name: validated.admin_name,
            admin_email: validated.admin_email,
            admin_password_hash: password_hash,
            runtime_mode: state.config.runtime_mode.to_string(),
            core_version: env!("CARGO_PKG_VERSION").to_string(),
            storage_path: state.config.storage_path.clone(),
        },
    )
    .await
    .map_err(|error| match error {
        core_db::InstallRepositoryError::AlreadyLocked => install_error(
            StatusCode::CONFLICT,
            "already_locked",
            "installation is already locked".to_string(),
        ),
        core_db::InstallRepositoryError::Database(error) => {
            install_error(StatusCode::INTERNAL_SERVER_ERROR, "setup_failed", {
                tracing::warn!(error = %error, "install setup failed");
                "setup failed; check backend logs for details".to_string()
            })
        }
    })?;

    Ok(Json(SetupResponse {
        state: record.state,
        locked: record.locked,
    }))
}

fn install_error(
    status: StatusCode,
    error: &'static str,
    message: String,
) -> (StatusCode, Json<InstallErrorResponse>) {
    (status, Json(InstallErrorResponse { error, message }))
}
