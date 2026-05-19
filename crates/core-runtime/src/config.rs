use crate::RuntimeMode;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Clone, Deserialize)]
pub struct AppConfig {
    pub runtime_mode: RuntimeMode,
    pub bind_addr: SocketAddr,
    pub base_url: String,
    pub database_url: SecretString,
    pub storage_path: String,
    pub cors_allowed_origins: Vec<String>,
    pub cookie_secure: bool,
}

impl std::fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppConfig")
            .field("runtime_mode", &self.runtime_mode)
            .field("bind_addr", &self.bind_addr)
            .field("base_url", &self.base_url)
            .field("database_url", &"<redacted>")
            .field("storage_path", &self.storage_path)
            .field("cors_allowed_origins", &self.cors_allowed_origins)
            .field("cookie_secure", &self.cookie_secure)
            .finish()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SafeAppConfig {
    pub runtime_mode: RuntimeMode,
    pub bind_addr: SocketAddr,
    pub base_url: String,
    pub cors_allowed_origins: Vec<String>,
    pub cookie_secure: bool,
    pub database_url_configured: bool,
    pub storage_configured: bool,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let runtime_mode = match std::env::var("RUNTIME_MODE")
            .unwrap_or_else(|_| "vps".to_string())
            .as_str()
        {
            "desktop" => RuntimeMode::Desktop,
            _ => RuntimeMode::Vps,
        };
        let bind_addr = std::env::var("BIND_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:7301".to_string())
            .parse()?;
        let base_url =
            std::env::var("BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:7301".to_string());
        let database_url = SecretString::from(std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://market:market@127.0.0.1:5432/market_osource".to_string()
        }));
        let storage_path =
            std::env::var("STORAGE_PATH").unwrap_or_else(|_| "./storage".to_string());
        let cors_allowed_origins = std::env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| base_url.clone())
            .split(',')
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect();
        let cookie_secure = std::env::var("COOKIE_SECURE")
            .map(|value| value == "true" || value == "1")
            .unwrap_or_else(|_| base_url.starts_with("https://"));

        Ok(Self {
            runtime_mode,
            bind_addr,
            base_url,
            database_url,
            storage_path,
            cors_allowed_origins,
            cookie_secure,
        })
    }

    pub fn safe_summary(&self) -> SafeAppConfig {
        SafeAppConfig {
            runtime_mode: self.runtime_mode,
            bind_addr: self.bind_addr,
            base_url: self.base_url.clone(),
            cors_allowed_origins: self.cors_allowed_origins.clone(),
            cookie_secure: self.cookie_secure,
            database_url_configured: !self.database_url.expose_secret().is_empty(),
            storage_configured: !self.storage_path.is_empty(),
        }
    }
}
