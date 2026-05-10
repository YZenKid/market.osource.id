use serde::Serialize;
use std::{env, path::Path};

#[derive(Debug, Clone, Serialize)]
pub struct PostgresSidecarStatus {
    pub available: bool,
    pub running: bool,
    pub bind_addr: String,
    pub note: String,
}

pub fn default_desktop_status() -> PostgresSidecarStatus {
    desktop_status("postgres", "127.0.0.1")
}

pub fn desktop_status(binary_name: &str, bind_addr: &str) -> PostgresSidecarStatus {
    let available = binary_available(binary_name);
    PostgresSidecarStatus {
        available,
        running: false,
        bind_addr: bind_addr.to_string(),
        note: if available {
            "PostgreSQL binary detected; lifecycle supervision is not started yet".to_string()
        } else {
            "PostgreSQL binary not detected; bundled lifecycle supervision is pending".to_string()
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
    fn desktop_status_keeps_loopback_bind_addr() {
        let status = desktop_status("definitely-missing-postgres", "127.0.0.1");

        assert!(!status.available);
        assert!(!status.running);
        assert_eq!(status.bind_addr, "127.0.0.1");
    }
}
