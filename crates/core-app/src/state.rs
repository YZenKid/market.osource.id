use core_runtime::AppConfig;
use core_storage::LocalStorageProvider;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_pool: Option<PgPool>,
    pub storage: Arc<LocalStorageProvider>,
    pub package_gate: Arc<core_packages::FeatureGate>,
    pub package_registry: Arc<Vec<core_packages::PackageDefinition>>,
    pub started_at: std::time::SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_runtime::AppConfig;
    use secrecy::SecretString;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    fn test_config(storage_path: String) -> AppConfig {
        AppConfig {
            runtime_mode: core_runtime::RuntimeMode::Vps,
            bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
            base_url: "http://127.0.0.1:8080".to_string(),
            database_url: SecretString::from("".to_string()),
            storage_path,
            cors_allowed_origins: vec!["http://127.0.0.1:5173".to_string()],
            cookie_secure: false,
        }
    }

    #[tokio::test]
    async fn readiness_reports_not_ready_without_database() {
        let storage_path =
            std::env::temp_dir().join(format!("market-osource-readiness-{}", uuid::Uuid::new_v4()));
        let state = AppState::new(test_config(storage_path.to_string_lossy().to_string()));
        let report = state.readiness().await;

        assert_eq!(report.status, "not_ready");
        assert!(report
            .checks
            .iter()
            .any(|check| check.name == "database" && !check.ok));
        assert!(report
            .checks
            .iter()
            .any(|check| check.name == "migrations" && !check.ok));
        assert!(report
            .checks
            .iter()
            .any(|check| check.name == "storage" && check.ok));
        assert!(report
            .checks
            .iter()
            .any(|check| check.name == "install_state" && !check.ok));
        assert!(report
            .checks
            .iter()
            .any(|check| check.name == "package_registry" && check.ok));
        assert!(report
            .checks
            .iter()
            .any(|check| check.name == "package_compatibility" && !check.ok));

        let _ = tokio::fs::remove_dir_all(storage_path).await;
    }
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        let storage = Arc::new(LocalStorageProvider::new(config.storage_path.clone()));
        let package_registry = Arc::new(core_packages::base_registry());
        Self {
            config: Arc::new(config),
            db_pool: None,
            storage,
            package_gate: Arc::new(core_packages::FeatureGate::default()),
            package_registry,
            started_at: std::time::SystemTime::now(),
        }
    }

    pub async fn try_connect_db(mut self) -> Self {
        if !self.config.database_url.expose_secret().is_empty() {
            match core_db::connect(&self.config.database_url).await {
                Ok(pool) => self.db_pool = Some(pool),
                Err(error) => {
                    tracing::warn!(error = %error, "database connection unavailable during startup")
                }
            }
        }
        self
    }

    pub fn with_package_gate(mut self, package_gate: core_packages::FeatureGate) -> Self {
        self.package_gate = Arc::new(package_gate);
        self
    }

    pub async fn readiness(&self) -> ReadinessReport {
        let db_ok = if let Some(pool) = &self.db_pool {
            core_db::database_ready(pool).await
        } else {
            false
        };
        let migrations_ok = if let Some(pool) = &self.db_pool {
            core_db::migration_table_ready(pool).await
        } else {
            false
        };
        let install_state_ok = if let Some(pool) = &self.db_pool {
            core_db::install_state_ready(pool).await
        } else {
            false
        };
        let package_compatibility_ok = if let Some(pool) = &self.db_pool {
            core_db::package_compatibility_ready(pool).await
        } else {
            false
        };
        let storage_ok = self.storage.ensure_root().await.is_ok();
        let package_registry_ok = !self.package_registry.is_empty();

        let checks = vec![
            ReadinessCheck {
                name: "database",
                ok: db_ok,
                message: (!db_ok).then(|| "database is not connected or not reachable".to_string()),
            },
            ReadinessCheck {
                name: "migrations",
                ok: migrations_ok,
                message: (!migrations_ok).then(|| {
                    "migration state table is unavailable or migrations have not run".to_string()
                }),
            },
            ReadinessCheck {
                name: "storage",
                ok: storage_ok,
                message: (!storage_ok).then(|| "storage path is not writable".to_string()),
            },
            ReadinessCheck {
                name: "install_state",
                ok: install_state_ok,
                message: (!install_state_ok).then(|| {
                    "installation is not locked or install state table is unavailable".to_string()
                }),
            },
            ReadinessCheck {
                name: "package_registry",
                ok: package_registry_ok,
                message: (!package_registry_ok).then(|| "package registry is empty".to_string()),
            },
            ReadinessCheck {
                name: "package_compatibility",
                ok: package_compatibility_ok,
                message: (!package_compatibility_ok)
                    .then(|| "package compatibility table is unavailable".to_string()),
            },
        ];

        ReadinessReport {
            status: if checks.iter().all(|check| check.ok) {
                "ready"
            } else {
                "not_ready"
            },
            checks,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ReadinessReport {
    pub status: &'static str,
    pub checks: Vec<ReadinessCheck>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ReadinessCheck {
    pub name: &'static str,
    pub ok: bool,
    pub message: Option<String>,
}
