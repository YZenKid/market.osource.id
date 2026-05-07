use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDefinition {
    pub package_id: String,
    pub name: String,
    pub capabilities: Vec<String>,
    pub paid: bool,
}

pub fn base_registry() -> Vec<PackageDefinition> {
    vec![
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
