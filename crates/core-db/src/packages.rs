use core_packages::PackageDefinition;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageLicenseStatus {
    Base,
    Active,
    Expired,
    Grace,
    Unreachable,
    Invalid,
    Unlicensed,
}

impl PackageLicenseStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Active => "active",
            Self::Expired => "expired",
            Self::Grace => "grace",
            Self::Unreachable => "unreachable",
            Self::Invalid => "invalid",
            Self::Unlicensed => "unlicensed",
        }
    }

    pub fn permits_capabilities(self) -> bool {
        matches!(self, Self::Base | Self::Active | Self::Grace)
    }
}

impl From<&str> for PackageLicenseStatus {
    fn from(value: &str) -> Self {
        match value {
            "base" => Self::Base,
            "active" => Self::Active,
            "expired" => Self::Expired,
            "grace" => Self::Grace,
            "unreachable" => Self::Unreachable,
            "invalid" => Self::Invalid,
            "unlicensed" => Self::Unlicensed,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PackageStatusRecord {
    pub package_id: String,
    pub name: String,
    pub version: String,
    pub core_version_range: String,
    pub license_status: PackageLicenseStatus,
    pub enabled: bool,
    pub paid: bool,
    pub capabilities: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum PackageRepositoryError {
    #[error("package repository operation failed: {0}")]
    Sqlx(#[from] sqlx::Error),
}

pub async fn sync_static_package_registry(
    pool: &PgPool,
    registry: &[PackageDefinition],
) -> Result<(), PackageRepositoryError> {
    let mut tx = pool.begin().await?;
    for package in registry {
        sqlx::query(
            r#"
            INSERT INTO packages (
                package_id, name, version, core_version_range, license_status, enabled, installed_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, CASE WHEN $6 THEN now() ELSE NULL END)
            ON CONFLICT (package_id) DO UPDATE SET
                name = EXCLUDED.name,
                version = EXCLUDED.version,
                core_version_range = EXCLUDED.core_version_range,
                updated_at = now()
            "#,
        )
        .bind(&package.package_id)
        .bind(&package.name)
        .bind(package.default_version())
        .bind(package.default_core_version_range())
        .bind(package.default_license_status())
        .bind(package.default_enabled())
        .execute(tx.as_mut())
        .await?;
    }
    tx.commit().await?;
    Ok(())
}

pub async fn list_package_statuses(
    pool: &PgPool,
    registry: &[PackageDefinition],
) -> Result<Vec<PackageStatusRecord>, PackageRepositoryError> {
    let rows = sqlx::query_as::<_, PackageRow>(
        r#"
        SELECT package_id, name, version, core_version_range, license_status, enabled
        FROM packages
        ORDER BY package_id ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let definition = registry
                .iter()
                .find(|package| package.package_id == row.package_id);
            PackageStatusRecord {
                package_id: row.package_id,
                name: row.name,
                version: row.version,
                core_version_range: row.core_version_range,
                license_status: PackageLicenseStatus::from(row.license_status.as_str()),
                enabled: row.enabled,
                paid: definition.is_none_or(|package| package.paid),
                capabilities: definition
                    .map(|package| package.capabilities.clone())
                    .unwrap_or_default(),
            }
        })
        .collect())
}

pub fn capability_enabled_in_statuses(statuses: &[PackageStatusRecord], capability: &str) -> bool {
    statuses.iter().any(|package| {
        package.enabled
            && package.license_status.permits_capabilities()
            && package
                .capabilities
                .iter()
                .any(|candidate| candidate == capability)
    })
}

#[derive(Debug, sqlx::FromRow)]
struct PackageRow {
    package_id: String,
    name: String,
    version: String,
    core_version_range: String,
    license_status: String,
    enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn license_status_supports_entitlement_cache_states() {
        for status in ["active", "expired", "grace", "unreachable", "invalid"] {
            assert_eq!(PackageLicenseStatus::from(status).as_str(), status);
        }
    }

    #[test]
    fn disabled_or_unlicensed_paid_capability_is_denied() {
        let statuses = vec![PackageStatusRecord {
            package_id: "promo".to_string(),
            name: "Promo Package".to_string(),
            version: "0.1.0".to_string(),
            core_version_range: ">=0.1.0 <0.2.0".to_string(),
            license_status: PackageLicenseStatus::Unlicensed,
            enabled: false,
            paid: true,
            capabilities: vec!["promo.voucher".to_string()],
        }];

        assert!(!capability_enabled_in_statuses(&statuses, "promo.voucher"));
    }

    #[test]
    fn base_capability_is_allowed_without_paid_entitlement() {
        let statuses = vec![PackageStatusRecord {
            package_id: "base".to_string(),
            name: "Base Open Source".to_string(),
            version: "0.1.0".to_string(),
            core_version_range: ">=0.1.0 <0.2.0".to_string(),
            license_status: PackageLicenseStatus::Base,
            enabled: true,
            paid: false,
            capabilities: vec!["core.catalog.basic".to_string()],
        }];

        assert!(capability_enabled_in_statuses(
            &statuses,
            "core.catalog.basic"
        ));
    }
}
