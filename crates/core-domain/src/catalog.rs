use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductStatus {
    Draft,
    Published,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VariantStatus {
    Active,
    Inactive,
}

pub fn validate_non_negative_stock(stock: i32) -> Result<(), CatalogError> {
    if stock < 0 {
        return Err(CatalogError::NegativeStock);
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("stock cannot be negative")]
    NegativeStock,
}
