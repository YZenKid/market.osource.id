use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PostgresSidecarStatus {
    pub available: bool,
    pub running: bool,
    pub bind_addr: String,
}

pub fn default_desktop_status() -> PostgresSidecarStatus {
    PostgresSidecarStatus {
        available: false,
        running: false,
        bind_addr: "127.0.0.1".to_string(),
    }
}
