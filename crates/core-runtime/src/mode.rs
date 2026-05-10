use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeMode {
    Desktop,
    #[default]
    Vps,
}

impl fmt::Display for RuntimeMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Desktop => formatter.write_str("desktop"),
            Self::Vps => formatter.write_str("vps"),
        }
    }
}
