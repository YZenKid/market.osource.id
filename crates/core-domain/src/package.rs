use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageLicenseStatus {
    Base,
    Active,
    Grace,
    Expired,
    Invalid,
    Unreachable,
    Unlicensed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Capability(pub String);
