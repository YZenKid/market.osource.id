use crate::{
    authz::{require_super_admin, AuthenticatedActor},
    routes::{auth::AuthErrorResponse, system::InstallStateSummary},
};
use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use core_app::{AppState, ReadinessReport};
use secrecy::ExposeSecret;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RedactionCategory {
    Secret,
    FilesystemPath,
    StorageObjectKey,
    CustomerData,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RedactedField {
    pub configured: bool,
    pub redacted: bool,
    pub category: RedactionCategory,
}

impl RedactedField {
    pub fn from_optional(value: Option<&str>, category: RedactionCategory) -> Self {
        let configured = value.is_some_and(|candidate| !candidate.trim().is_empty());
        Self {
            configured,
            redacted: configured,
            category,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DiagnosticsExport {
    generated_at: DateTime<Utc>,
    config: DiagnosticsConfigSummary,
    runtime: RuntimeSummary,
    readiness: ReadinessReport,
    install_state: InstallStateSummary,
    database: DatabaseSummary,
    desktop_postgres: sidecar_postgres::PostgresSidecarStatus,
    tunnel: sidecar_cloudflared::CloudflaredSidecarStatus,
    storage: StorageSummary,
    packages: PackageSummary,
    redaction_policy: RedactionPolicySummary,
}

#[derive(Debug, Clone, Serialize)]
struct DiagnosticsConfigSummary {
    runtime_mode: core_runtime::RuntimeMode,
    bind_addr: std::net::SocketAddr,
    base_url: String,
    cors_allowed_origins: Vec<String>,
    cookie_secure: bool,
    database_url: RedactedField,
    storage_path: RedactedField,
}

#[derive(Debug, Clone, Serialize)]
struct RuntimeSummary {
    started_at: DateTime<Utc>,
    uptime_seconds: u64,
    backend_ok: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DatabaseSummary {
    configured: bool,
    connected: bool,
}

#[derive(Debug, Clone, Serialize)]
struct StorageSummary {
    provider: &'static str,
    configured: bool,
    writable_check_available: bool,
}

#[derive(Debug, Clone, Serialize)]
struct PackageSummary {
    registry_count: usize,
    enabled_capabilities: usize,
    definitions: Vec<PackageDefinitionSummary>,
    database_statuses: Option<Vec<PackageDatabaseStatusSummary>>,
    database_status_message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct PackageDefinitionSummary {
    package_id: String,
    name: String,
    paid: bool,
    capability_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct PackageDatabaseStatusSummary {
    package_id: String,
    name: String,
    version: String,
    core_version_range: String,
    license_status: core_db::PackageLicenseStatus,
    enabled: bool,
    paid: bool,
    capability_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct RedactionPolicySummary {
    excluded_fields: Vec<&'static str>,
    notes: Vec<&'static str>,
}

pub(crate) async fn export(
    State(state): State<AppState>,
    actor: AuthenticatedActor,
) -> Result<Json<DiagnosticsExport>, (StatusCode, Json<AuthErrorResponse>)> {
    require_super_admin(&actor)?;
    Ok(Json(build_diagnostics_export(&state).await))
}

pub(crate) async fn build_diagnostics_export(state: &AppState) -> DiagnosticsExport {
    let readiness = state.readiness().await;
    let install_state = crate::routes::system::install_state_summary(state).await;
    let package_summary = package_summary(state).await;

    DiagnosticsExport {
        generated_at: Utc::now(),
        config: DiagnosticsConfigSummary {
            runtime_mode: state.config.runtime_mode,
            bind_addr: state.config.bind_addr,
            base_url: state.config.base_url.clone(),
            cors_allowed_origins: state.config.cors_allowed_origins.clone(),
            cookie_secure: state.config.cookie_secure,
            database_url: RedactedField::from_optional(
                Some(state.config.database_url.expose_secret()),
                RedactionCategory::Secret,
            ),
            storage_path: RedactedField::from_optional(
                Some(&state.config.storage_path),
                RedactionCategory::FilesystemPath,
            ),
        },
        runtime: RuntimeSummary {
            started_at: DateTime::<Utc>::from(state.started_at),
            uptime_seconds: state.started_at.elapsed().map(|value| value.as_secs()).unwrap_or(0),
            backend_ok: true,
        },
        readiness,
        install_state,
        database: DatabaseSummary {
            configured: !state.config.database_url.expose_secret().trim().is_empty(),
            connected: state.db_pool.is_some(),
        },
        desktop_postgres: sidecar_postgres::default_desktop_status(),
        tunnel: sidecar_cloudflared::default_tunnel_status(),
        storage: StorageSummary {
            provider: "local",
            configured: !state.config.storage_path.trim().is_empty(),
            writable_check_available: true,
        },
        packages: package_summary,
        redaction_policy: RedactionPolicySummary {
            excluded_fields: vec![
                "config.database_url.value",
                "config.storage_path.value",
                "orders.customer_name",
                "orders.customer_contact",
                "orders.shipping_address",
                "payment_proofs.file_object_key",
                "file_objects.object_key",
                "payment_proofs.original_filename",
            ],
            notes: vec![
                "diagnostics export is operator-focused and excludes customer/order payloads",
                "private storage identifiers and filesystem roots are represented only as redacted presence flags",
                "package summary includes capability counts and entitlement state, not raw secrets or proof references",
            ],
        },
    }
}

async fn package_summary(state: &AppState) -> PackageSummary {
    let definitions = state
        .package_registry
        .iter()
        .map(|definition| PackageDefinitionSummary {
            package_id: definition.package_id.clone(),
            name: definition.name.clone(),
            paid: definition.paid,
            capability_count: definition.capabilities.len(),
        })
        .collect();

    let (database_statuses, database_status_message) = match &state.db_pool {
        Some(pool) => {
            match core_db::list_package_statuses(pool, state.package_registry.as_ref()).await {
                Ok(statuses) => (
                    Some(
                        statuses
                            .into_iter()
                            .map(|status| PackageDatabaseStatusSummary {
                                package_id: status.package_id,
                                name: status.name,
                                version: status.version,
                                core_version_range: status.core_version_range,
                                license_status: status.license_status,
                                enabled: status.enabled,
                                paid: status.paid,
                                capability_count: status.capabilities.len(),
                            })
                            .collect(),
                    ),
                    None,
                ),
                Err(error) => {
                    tracing::warn!(error = %error, "diagnostics package summary unavailable");
                    (
                        None,
                        Some("package status table unavailable or not yet populated".to_string()),
                    )
                }
            }
        }
        None => (None, Some("database is not connected".to_string())),
    };

    PackageSummary {
        registry_count: state.package_registry.len(),
        enabled_capabilities: state.package_gate.enabled_count(),
        definitions,
        database_statuses,
        database_status_message,
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
            database_url: SecretString::from(
                "postgres://secret-user:top-secret@127.0.0.1:5432/market".to_string(),
            ),
            storage_path: "/srv/market/private-storage".to_string(),
            cors_allowed_origins: vec!["http://127.0.0.1:8300".to_string()],
            cookie_secure: false,
        })
    }

    #[test]
    fn redacted_field_marks_sensitive_values_without_exposing_content() {
        let secret = RedactedField::from_optional(Some("top-secret"), RedactionCategory::Secret);
        let object_key = RedactedField::from_optional(
            Some("payment-proofs/2026/05/proof-123.png"),
            RedactionCategory::StorageObjectKey,
        );
        let customer = RedactedField::from_optional(
            Some("customer@example.com"),
            RedactionCategory::CustomerData,
        );

        assert!(secret.configured && secret.redacted);
        assert_eq!(object_key.category, RedactionCategory::StorageObjectKey);
        assert_eq!(customer.category, RedactionCategory::CustomerData);
    }

    #[tokio::test]
    async fn diagnostics_export_omits_secret_and_private_path_values() {
        let diagnostics = build_diagnostics_export(&test_state()).await;
        let payload =
            serde_json::to_string(&diagnostics).expect("diagnostics payload must serialize");

        assert!(!payload.contains("top-secret"));
        assert!(!payload.contains("/srv/market/private-storage"));
        assert!(!payload.contains("payment-proofs/2026/05/proof-123.png"));
        assert!(!payload.contains("customer@example.com"));
        assert!(payload.contains("\"redacted\":true"));
    }

    #[tokio::test]
    async fn diagnostics_export_requires_session_cookie() {
        let mut state = test_state();
        state.db_pool = Some(sqlx::PgPool::connect_lazy("postgres://example.invalid/db").unwrap());

        let response = app_router(state)
            .oneshot(
                Request::builder()
                    .uri("/api/admin/diagnostics/export")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
