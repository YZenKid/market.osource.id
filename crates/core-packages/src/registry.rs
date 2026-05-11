use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDefinition {
    pub package_id: String,
    pub name: String,
    pub capabilities: Vec<String>,
    pub paid: bool,
}

impl PackageDefinition {
    pub fn default_version(&self) -> &'static str {
        "0.1.0"
    }

    pub fn default_core_version_range(&self) -> &'static str {
        ">=0.1.0 <0.2.0"
    }

    pub fn default_license_status(&self) -> &'static str {
        if self.paid {
            "unlicensed"
        } else {
            "base"
        }
    }

    pub fn default_enabled(&self) -> bool {
        !self.paid
    }
}

pub fn base_registry() -> Vec<PackageDefinition> {
    vec![
        PackageDefinition {
            package_id: "base".to_string(),
            name: "Base Open Source".to_string(),
            capabilities: vec![
                "core.catalog.basic".to_string(),
                "core.checkout.manual_transfer".to_string(),
                "core.storage.local".to_string(),
            ],
            paid: false,
        },
        PackageDefinition {
            package_id: "promo".to_string(),
            name: "Promo Package".to_string(),
            capabilities: vec!["promo.voucher".to_string()],
            paid: true,
        },
        PackageDefinition {
            package_id: "online-payment".to_string(),
            name: "Online Payment Package".to_string(),
            capabilities: vec!["payment.online".to_string()],
            paid: true,
        },
        PackageDefinition {
            package_id: "delivery".to_string(),
            name: "Delivery Package".to_string(),
            capabilities: vec!["delivery.rate_lookup".to_string()],
            paid: true,
        },
    ]
}

pub fn base_capabilities() -> Vec<String> {
    base_registry()
        .into_iter()
        .filter(|package| !package.paid)
        .flat_map(|package| package.capabilities)
        .collect()
}

pub fn paid_capabilities() -> Vec<String> {
    base_registry()
        .into_iter()
        .filter(|package| package.paid)
        .flat_map(|package| package.capabilities)
        .collect()
}
