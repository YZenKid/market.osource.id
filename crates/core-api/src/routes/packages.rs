use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use core_app::AppState;
use serde::Serialize;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_public))
        .route("/sample/base-capability", get(sample_base_capability))
        .route("/sample/paid-capability", get(sample_paid_capability))
}

#[derive(Debug, Serialize)]
struct PublicPackageResponse {
    packages: Vec<PublicPackageSummary>,
    model: &'static str,
    degraded: bool,
}

#[derive(Debug, Serialize)]
struct PublicPackageSummary {
    package_id: String,
    name: String,
    capabilities: Vec<String>,
    paid: bool,
}

#[derive(Debug, Serialize)]
struct CapabilityResponse {
    capability: &'static str,
    allowed: bool,
}

#[derive(Debug, Serialize)]
struct PackageErrorResponse {
    error: &'static str,
    message: String,
}

async fn list_public(State(state): State<AppState>) -> Json<PublicPackageResponse> {
    let registry = state.package_registry.as_ref();
    if let Some(pool) = &state.db_pool {
        match list_public_from_db(pool, registry).await {
            Ok(packages) => {
                return Json(PublicPackageResponse {
                    packages,
                    model: "remote_entitlement_ready",
                    degraded: false,
                });
            }
            Err(error) => {
                tracing::warn!(error = %error, "package status DB read failed; falling back to static public registry")
            }
        }
    }

    Json(PublicPackageResponse {
        packages: registry
            .iter()
            .map(public_summary_from_definition)
            .collect(),
        model: "remote_entitlement_ready",
        degraded: true,
    })
}

async fn list_public_from_db(
    pool: &sqlx::PgPool,
    registry: &[core_packages::PackageDefinition],
) -> Result<Vec<PublicPackageSummary>, core_db::PackageRepositoryError> {
    core_db::sync_static_package_registry(pool, registry).await?;
    let statuses = core_db::list_package_statuses(pool, registry).await?;
    Ok(statuses
        .into_iter()
        .map(|package| PublicPackageSummary {
            package_id: package.package_id,
            name: package.name,
            capabilities: package.capabilities,
            paid: package.paid,
        })
        .collect())
}

async fn sample_base_capability(
    State(state): State<AppState>,
) -> Result<Json<CapabilityResponse>, (StatusCode, Json<PackageErrorResponse>)> {
    require_capability(&state, "core.catalog.basic").await?;
    Ok(Json(CapabilityResponse {
        capability: "core.catalog.basic",
        allowed: true,
    }))
}

async fn sample_paid_capability(
    State(state): State<AppState>,
) -> Result<Json<CapabilityResponse>, (StatusCode, Json<PackageErrorResponse>)> {
    require_capability(&state, "promo.voucher").await?;
    Ok(Json(CapabilityResponse {
        capability: "promo.voucher",
        allowed: true,
    }))
}

async fn require_capability(
    state: &AppState,
    capability: &'static str,
) -> Result<(), (StatusCode, Json<PackageErrorResponse>)> {
    let allowed = if let Some(pool) = &state.db_pool {
        match package_statuses(pool, state.package_registry.as_ref()).await {
            Ok(statuses) => core_db::capability_enabled_in_statuses(&statuses, capability),
            Err(error) => {
                tracing::warn!(error = %error, capability, "package capability DB check failed; using offline base fallback");
                core_packages::base_capabilities()
                    .iter()
                    .any(|base_capability| base_capability == capability)
            }
        }
    } else {
        state.package_gate.is_enabled(capability)
    };

    allowed.then_some(()).ok_or_else(|| {
        (
            StatusCode::FORBIDDEN,
            Json(PackageErrorResponse {
                error: "package_capability_disabled",
                message: "required package capability is disabled or unlicensed".to_string(),
            }),
        )
    })
}

async fn package_statuses(
    pool: &sqlx::PgPool,
    registry: &[core_packages::PackageDefinition],
) -> Result<Vec<core_db::PackageStatusRecord>, core_db::PackageRepositoryError> {
    core_db::sync_static_package_registry(pool, registry).await?;
    core_db::list_package_statuses(pool, registry).await
}

fn public_summary_from_definition(
    package: &core_packages::PackageDefinition,
) -> PublicPackageSummary {
    PublicPackageSummary {
        package_id: package.package_id.clone(),
        name: package.name.clone(),
        capabilities: package.capabilities.clone(),
        paid: package.paid,
    }
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
            base_url: "http://127.0.0.1:8301".to_string(),
            database_url: SecretString::from("".to_string()),
            storage_path: std::env::temp_dir().to_string_lossy().to_string(),
            cors_allowed_origins: Vec::new(),
            cookie_secure: false,
        })
    }

    #[tokio::test]
    async fn sample_paid_capability_is_denied_when_disabled_offline() {
        let response = app_router(test_state())
            .oneshot(
                Request::builder()
                    .uri("/api/packages/sample/paid-capability")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn sample_base_capability_works_offline_without_entitlement() {
        let response = app_router(test_state())
            .oneshot(
                Request::builder()
                    .uri("/api/packages/sample/base-capability")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
