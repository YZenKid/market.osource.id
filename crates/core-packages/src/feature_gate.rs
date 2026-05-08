use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct FeatureGate {
    enabled: HashSet<String>,
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

#[derive(Debug, thiserror::Error)]
pub enum FeatureGateError {
    #[error("feature capability is disabled: {0}")]
    Disabled(String),
}
