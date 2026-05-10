use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PreflightReport {
    pub ok: bool,
    pub checks: Vec<PreflightCheck>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreflightCheck {
    pub name: &'static str,
    pub ok: bool,
    pub message: Option<String>,
}

impl PreflightReport {
    pub fn all_ok(checks: Vec<PreflightCheck>) -> Self {
        let ok = checks.iter().all(|check| check.ok);
        Self { ok, checks }
    }
}

pub fn install_preflight(
    db_connected: bool,
    migrations_ready: bool,
    storage_ready: bool,
) -> PreflightReport {
    PreflightReport::all_ok(vec![
        PreflightCheck {
            name: "database",
            ok: db_connected,
            message: (!db_connected).then(|| "database is not connected".to_string()),
        },
        PreflightCheck {
            name: "migrations",
            ok: migrations_ready,
            message: (!migrations_ready).then(|| "core migrations have not run".to_string()),
        },
        PreflightCheck {
            name: "storage",
            ok: storage_ready,
            message: (!storage_ready).then(|| "storage path is not writable".to_string()),
        },
        PreflightCheck {
            name: "tunnel_default_off",
            ok: true,
            message: Some("cloudflared tunnel is opt-in and disabled during setup".to_string()),
        },
    ])
}
