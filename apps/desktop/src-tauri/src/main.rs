use serde::Serialize;
use sidecar_postgres::{PostgresSidecarHandle, RuntimeLifecycleState};
use std::{
    env,
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::Mutex,
};

#[derive(Debug, Clone, Serialize)]
struct BackendStatus {
    state: RuntimeLifecycleState,
    bind: String,
    command: String,
    public_http_exposure: bool,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
struct DesktopRuntimeStatus {
    mode: &'static str,
    release_identity: serde_json::Value,
    runtime_contract: serde_json::Value,
    backend: BackendStatus,
    postgres: sidecar_postgres::PostgresSidecarStatus,
    tunnel: sidecar_cloudflared::CloudflaredSidecarStatus,
    storage: serde_json::Value,
    operator_caution: Vec<&'static str>,
}

#[derive(Debug)]
struct BackendSupervisor {
    command: String,
    args: Vec<String>,
    bind: String,
    workdir: Option<PathBuf>,
    child: Option<Child>,
    state: RuntimeLifecycleState,
    note: String,
}

impl BackendSupervisor {
    fn from_env() -> Self {
        let command = env::var("DESKTOP_BACKEND_COMMAND").unwrap_or_else(|_| "market-backend".to_string());
        let args = env::var("DESKTOP_BACKEND_ARGS")
            .map(|raw| raw.split_whitespace().map(ToString::to_string).collect())
            .unwrap_or_default();
        let bind = env::var("DESKTOP_BACKEND_BIND").unwrap_or_else(|_| "127.0.0.1:7301".to_string());
        let workdir = env::var_os("DESKTOP_BACKEND_WORKDIR").map(PathBuf::from);
        let available = sidecar_postgres::binary_available(&command);
        let state = if available {
            RuntimeLifecycleState::Stopped
        } else {
            RuntimeLifecycleState::Unavailable
        };
        let note = if !is_local_bind(&bind) {
            "Backend bind is not loopback; start commands are blocked for operator safety".to_string()
        } else if available {
            "Backend supervisor is configured and stopped".to_string()
        } else {
            "Backend command is not available".to_string()
        };

        Self {
            command,
            args,
            bind,
            workdir,
            child: None,
            state,
            note,
        }
    }

    fn status(&mut self) -> BackendStatus {
        if let Some(child) = self.child.as_mut() {
            match child.try_wait() {
                Ok(Some(exit)) => {
                    self.child = None;
                    self.state = RuntimeLifecycleState::Failed;
                    self.note = format!("Backend process exited with status {exit}");
                }
                Ok(None) => {
                    self.state = RuntimeLifecycleState::Running;
                    self.note = "Backend process is running".to_string();
                }
                Err(err) => {
                    self.state = RuntimeLifecycleState::Failed;
                    self.note = format!("Failed to check backend process state: {err}");
                }
            }
        }
        self.snapshot()
    }

    fn start(&mut self) -> BackendStatus {
        if matches!(self.state, RuntimeLifecycleState::Running | RuntimeLifecycleState::Starting) {
            return self.snapshot();
        }
        if !is_local_bind(&self.bind) {
            self.state = RuntimeLifecycleState::Failed;
            self.note = "Refusing to start backend: bind must be loopback (127.0.0.1 or localhost)".to_string();
            return self.snapshot();
        }
        if !sidecar_postgres::binary_available(&self.command) {
            self.state = RuntimeLifecycleState::Unavailable;
            self.note = "Backend command is not available".to_string();
            return self.snapshot();
        }

        self.state = RuntimeLifecycleState::Starting;
        let mut cmd = Command::new(&self.command);
        cmd.args(&self.args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        if let Some(wd) = &self.workdir {
            cmd.current_dir(wd);
        }

        match cmd.spawn() {
            Ok(child) => {
                self.child = Some(child);
                self.state = RuntimeLifecycleState::Running;
                self.note = "Backend process started".to_string();
            }
            Err(err) => {
                self.child = None;
                self.state = RuntimeLifecycleState::Failed;
                self.note = format!("Failed to start backend: {err}");
            }
        }
        self.snapshot()
    }

    fn stop(&mut self) -> BackendStatus {
        self.state = RuntimeLifecycleState::Stopping;
        if let Some(mut child) = self.child.take() {
            match child.kill() {
                Ok(()) => {
                    let _ = child.wait();
                    self.state = RuntimeLifecycleState::Stopped;
                    self.note = "Backend process stopped".to_string();
                }
                Err(err) => {
                    self.state = RuntimeLifecycleState::Failed;
                    self.note = format!("Failed to stop backend: {err}");
                }
            }
        } else if sidecar_postgres::binary_available(&self.command) {
            self.state = RuntimeLifecycleState::Stopped;
            self.note = "Backend process already stopped".to_string();
        } else {
            self.state = RuntimeLifecycleState::Unavailable;
            self.note = "Backend command is not available".to_string();
        }
        self.snapshot()
    }

    fn restart(&mut self) -> BackendStatus {
        let _ = self.stop();
        self.start()
    }

    fn snapshot(&self) -> BackendStatus {
        BackendStatus {
            state: self.state.clone(),
            bind: self.bind.clone(),
            command: self.command.clone(),
            public_http_exposure: !is_local_bind(&self.bind),
            note: self.note.clone(),
        }
    }
}

struct DesktopRuntimeState {
    backend: Mutex<BackendSupervisor>,
    postgres: PostgresSidecarHandle,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum ControlAction {
    Start,
    Stop,
    Restart,
}

#[derive(Debug, Clone, Serialize)]
struct DesktopRuntimeControlResult {
    action: ControlAction,
    backend: BackendStatus,
    postgres: sidecar_postgres::PostgresSidecarStatus,
}

#[tauri::command]
fn desktop_runtime_status(state: tauri::State<'_, DesktopRuntimeState>) -> DesktopRuntimeStatus {
    let postgres = state.postgres.status();
    let tunnel = sidecar_cloudflared::default_tunnel_status();
    let backend = state
        .backend
        .lock()
        .expect("backend supervisor mutex poisoned")
        .status();

    DesktopRuntimeStatus {
        mode: "desktop-local-hosted",
        release_identity: serde_json::json!({
            "app": "market.osource.id",
            "version": env!("CARGO_PKG_VERSION"),
            "git_sha": option_env!("GIT_SHA"),
            "build_profile": option_env!("PROFILE").unwrap_or("unknown")
        }),
        runtime_contract: serde_json::json!({
            "liveness_probe": "/health",
            "readiness_probe": "/ready",
            "version_probe": "/version",
            "note": "Desktop shell supervises local backend and PostgreSQL sidecar processes"
        }),
        postgres,
        backend,
        tunnel,
        storage: serde_json::json!({
            "provider": "local",
            "configured": true,
            "note": "Storage root is managed by runtime configuration; runtime commands do not mutate it"
        }),
        operator_caution: vec![
            "Desktop mode remains operator-hosted; if the machine is off the marketplace is offline.",
            "Runtime control commands are local-only and intended for loopback-bound services.",
            "Tunnel is opt-in and should only expose the HTTP app origin.",
        ],
    }
}

#[tauri::command]
fn desktop_runtime_start(state: tauri::State<'_, DesktopRuntimeState>) -> DesktopRuntimeControlResult {
    let postgres = state.postgres.start();
    let backend = state
        .backend
        .lock()
        .expect("backend supervisor mutex poisoned")
        .start();
    DesktopRuntimeControlResult {
        action: ControlAction::Start,
        backend,
        postgres,
    }
}

#[tauri::command]
fn desktop_runtime_stop(state: tauri::State<'_, DesktopRuntimeState>) -> DesktopRuntimeControlResult {
    let backend = state
        .backend
        .lock()
        .expect("backend supervisor mutex poisoned")
        .stop();
    let postgres = state.postgres.stop();
    DesktopRuntimeControlResult {
        action: ControlAction::Stop,
        backend,
        postgres,
    }
}

#[tauri::command]
fn desktop_runtime_restart(state: tauri::State<'_, DesktopRuntimeState>) -> DesktopRuntimeControlResult {
    let postgres = state.postgres.restart();
    let backend = state
        .backend
        .lock()
        .expect("backend supervisor mutex poisoned")
        .restart();
    DesktopRuntimeControlResult {
        action: ControlAction::Restart,
        backend,
        postgres,
    }
}

fn is_local_bind(bind: &str) -> bool {
    bind == "127.0.0.1"
        || bind == "localhost"
        || bind.starts_with("127.0.0.1:")
        || bind.starts_with("localhost:")
}

fn main() {
    tauri::Builder::default()
        .manage(DesktopRuntimeState {
            backend: Mutex::new(BackendSupervisor::from_env()),
            postgres: PostgresSidecarHandle::from_env(),
        })
        .invoke_handler(tauri::generate_handler![
            desktop_runtime_status,
            desktop_runtime_start,
            desktop_runtime_stop,
            desktop_runtime_restart
        ])
        .run(tauri::generate_context!())
        .expect("failed to run market.osource.id desktop shell");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_bind_guard_accepts_loopback_only() {
        assert!(is_local_bind("127.0.0.1"));
        assert!(is_local_bind("localhost"));
        assert!(!is_local_bind("0.0.0.0"));
        assert!(!is_local_bind("192.168.1.20"));
    }
}
