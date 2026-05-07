use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CloudflaredSidecarStatus {
    pub available: bool,
    pub enabled: bool,
    pub running: bool,
    pub public_url: Option<String>,
}

pub fn default_tunnel_status() -> CloudflaredSidecarStatus {
    CloudflaredSidecarStatus {
        available: false,
        enabled: false,
        running: false,
        public_url: None,
    }
}
