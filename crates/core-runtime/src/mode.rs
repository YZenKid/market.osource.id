use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeMode {
    Desktop,
    Vps,
}

impl Default for RuntimeMode {
    fn default() -> Self {
        Self::Vps
    }
}
