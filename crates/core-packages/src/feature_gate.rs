use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct FeatureGate {
    enabled: HashSet<String>,
}

impl Default for FeatureGate {
    fn default() -> Self {
        Self::new(crate::base_capabilities())
    }
}

impl FeatureGate {
    pub fn new(enabled: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            enabled: enabled.into_iter().map(Into::into).collect(),
        }
    }

    pub fn is_enabled(&self, capability: &str) -> bool {
        self.enabled.contains(capability)
    }

    pub fn enabled_count(&self) -> usize {
        self.enabled.len()
    }

    pub fn require(&self, capability: &str) -> Result<(), FeatureGateError> {
        self.is_enabled(capability)
            .then_some(())
            .ok_or_else(|| FeatureGateError::Disabled(capability.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_gate_allows_base_capabilities_only() {
        let gate = FeatureGate::default();

        assert!(gate.require("core.catalog.basic").is_ok());
        assert!(gate.require("promo.voucher").is_err());
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FeatureGateError {
    #[error("feature capability is disabled: {0}")]
    Disabled(String),
}
