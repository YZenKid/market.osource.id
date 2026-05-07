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
