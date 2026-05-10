use serde::Serialize;
use std::{env, path::Path};

#[derive(Debug, Clone, Serialize)]
pub struct CloudflaredSidecarStatus {
    pub available: bool,
    pub enabled: bool,
    pub running: bool,
    pub public_url: Option<String>,
    pub note: String,
}

pub fn default_tunnel_status() -> CloudflaredSidecarStatus {
    tunnel_status("cloudflared", false, false, None)
}

pub fn tunnel_status(
    binary_name: &str,
    enabled: bool,
    running: bool,
    public_url: Option<String>,
) -> CloudflaredSidecarStatus {
    let available = binary_available(binary_name);
    CloudflaredSidecarStatus {
        available,
        enabled,
        running: running && enabled,
        public_url: if enabled { public_url } else { None },
        note: if enabled {
            "cloudflared is enabled in settings; process supervision is pending".to_string()
        } else {
            "cloudflared is opt-in and disabled by default".to_string()
        },
    }
}

pub fn binary_available(binary_name: &str) -> bool {
    if binary_name.contains(std::path::MAIN_SEPARATOR) {
        return Path::new(binary_name).is_file();
    }

    env::var_os("PATH")
        .map(|paths| env::split_paths(&paths).any(|path| path.join(binary_name).is_file()))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_tunnel_never_reports_running_or_url() {
        let status = tunnel_status(
            "definitely-missing-cloudflared",
            false,
            true,
            Some("https://example.com".to_string()),
        );

        assert!(!status.enabled);
        assert!(!status.running);
        assert!(status.public_url.is_none());
    }
}
