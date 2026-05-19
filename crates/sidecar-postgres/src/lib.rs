use serde::Serialize;
use std::{
    env,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeLifecycleState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
    Unavailable,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostgresSidecarStatus {
    pub state: RuntimeLifecycleState,
    pub bind_addr: String,
    pub command: String,
    pub note: String,
}

#[derive(Debug, Clone)]
pub struct PostgresSidecarConfig {
    pub command: String,
    pub args: Vec<String>,
    pub bind_addr: String,
    pub workdir: Option<PathBuf>,
}

struct PostgresSupervisor {
    config: PostgresSidecarConfig,
    child: Option<Child>,
    state: RuntimeLifecycleState,
    note: String,
}

#[derive(Clone)]
pub struct PostgresSidecarHandle {
    inner: Arc<Mutex<PostgresSupervisor>>,
}

pub fn default_desktop_status() -> PostgresSidecarStatus {
    PostgresSidecarHandle::from_env().status()
}

pub fn desktop_status(binary_name: &str, bind_addr: &str) -> PostgresSidecarStatus {
    let available = binary_available(binary_name);
    PostgresSidecarStatus {
        state: if available {
            RuntimeLifecycleState::Stopped
        } else {
            RuntimeLifecycleState::Unavailable
        },
        bind_addr: bind_addr.to_string(),
        command: binary_name.to_string(),
        note: if available {
            "PostgreSQL binary detected; supervisor is initialized but not started".to_string()
        } else {
            "PostgreSQL command is unavailable on this machine or PATH".to_string()
        },
    }
}

impl PostgresSidecarHandle {
    pub fn from_env() -> Self {
        let command = env::var("DESKTOP_POSTGRES_COMMAND").unwrap_or_else(|_| "postgres".to_string());
        let args = env::var("DESKTOP_POSTGRES_ARGS")
            .map(|raw| raw.split_whitespace().map(ToString::to_string).collect())
            .unwrap_or_default();
        let bind_addr = env::var("DESKTOP_POSTGRES_BIND")
            .unwrap_or_else(|_| "127.0.0.1:5432".to_string());
        let workdir = env::var_os("DESKTOP_POSTGRES_WORKDIR").map(PathBuf::from);

        Self::new(PostgresSidecarConfig {
            command,
            args,
            bind_addr,
            workdir,
        })
    }

    pub fn new(config: PostgresSidecarConfig) -> Self {
        let available = binary_available(&config.command);
        let state = if available {
            RuntimeLifecycleState::Stopped
        } else {
            RuntimeLifecycleState::Unavailable
        };
        let note = if available {
            if is_local_bind(&config.bind_addr) {
                "PostgreSQL sidecar is configured and stopped".to_string()
            } else {
                "PostgreSQL sidecar bind is not loopback; start commands are blocked for operator safety".to_string()
            }
        } else {
            "PostgreSQL sidecar command is not available".to_string()
        };

        Self {
            inner: Arc::new(Mutex::new(PostgresSupervisor {
                config,
                child: None,
                state,
                note,
            })),
        }
    }

    pub fn status(&self) -> PostgresSidecarStatus {
        let mut guard = self.inner.lock().expect("postgres supervisor mutex poisoned");
        if let Some(child) = guard.child.as_mut() {
            match child.try_wait() {
                Ok(Some(exit)) => {
                    guard.child = None;
                    guard.state = RuntimeLifecycleState::Failed;
                    guard.note = format!("PostgreSQL process exited with status {exit}");
                }
                Ok(None) => {
                    guard.state = RuntimeLifecycleState::Running;
                    guard.note = "PostgreSQL sidecar is running".to_string();
                }
                Err(err) => {
                    guard.state = RuntimeLifecycleState::Failed;
                    guard.note = format!("Failed to check PostgreSQL process state: {err}");
                }
            }
        }

        PostgresSidecarStatus {
            state: guard.state.clone(),
            bind_addr: guard.config.bind_addr.clone(),
            command: guard.config.command.clone(),
            note: guard.note.clone(),
        }
    }

    pub fn start(&self) -> PostgresSidecarStatus {
        let mut guard = self.inner.lock().expect("postgres supervisor mutex poisoned");
        if matches!(guard.state, RuntimeLifecycleState::Running | RuntimeLifecycleState::Starting) {
            return PostgresSidecarStatus {
                state: guard.state.clone(),
                bind_addr: guard.config.bind_addr.clone(),
                command: guard.config.command.clone(),
                note: "PostgreSQL sidecar is already active".to_string(),
            };
        }

        if !is_local_bind(&guard.config.bind_addr) {
            guard.state = RuntimeLifecycleState::Failed;
            guard.note = "Refusing to start PostgreSQL sidecar: bind must be loopback (127.0.0.1 or localhost)".to_string();
            return PostgresSidecarStatus {
                state: guard.state.clone(),
                bind_addr: guard.config.bind_addr.clone(),
                command: guard.config.command.clone(),
                note: guard.note.clone(),
            };
        }

        if !binary_available(&guard.config.command) {
            guard.state = RuntimeLifecycleState::Unavailable;
            guard.note = "PostgreSQL sidecar command is not available".to_string();
            return PostgresSidecarStatus {
                state: guard.state.clone(),
                bind_addr: guard.config.bind_addr.clone(),
                command: guard.config.command.clone(),
                note: guard.note.clone(),
            };
        }

        guard.state = RuntimeLifecycleState::Starting;
        let mut cmd = Command::new(&guard.config.command);
        cmd.args(&guard.config.args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        if let Some(wd) = &guard.config.workdir {
            cmd.current_dir(wd);
        }

        match cmd.spawn() {
            Ok(child) => {
                guard.child = Some(child);
                guard.state = RuntimeLifecycleState::Running;
                guard.note = "PostgreSQL sidecar started".to_string();
            }
            Err(err) => {
                guard.child = None;
                guard.state = RuntimeLifecycleState::Failed;
                guard.note = format!("Failed to start PostgreSQL sidecar: {err}");
            }
        }

        PostgresSidecarStatus {
            state: guard.state.clone(),
            bind_addr: guard.config.bind_addr.clone(),
            command: guard.config.command.clone(),
            note: guard.note.clone(),
        }
    }

    pub fn stop(&self) -> PostgresSidecarStatus {
        let mut guard = self.inner.lock().expect("postgres supervisor mutex poisoned");
        guard.state = RuntimeLifecycleState::Stopping;

        if let Some(mut child) = guard.child.take() {
            let note = match child.kill() {
                Ok(()) => {
                    let _ = child.wait();
                    guard.state = RuntimeLifecycleState::Stopped;
                    "PostgreSQL sidecar stopped".to_string()
                }
                Err(err) => {
                    guard.state = RuntimeLifecycleState::Failed;
                    format!("Failed to stop PostgreSQL sidecar: {err}")
                }
            };
            guard.note = note;
        } else if binary_available(&guard.config.command) {
            guard.state = RuntimeLifecycleState::Stopped;
            guard.note = "PostgreSQL sidecar already stopped".to_string();
        } else {
            guard.state = RuntimeLifecycleState::Unavailable;
            guard.note = "PostgreSQL sidecar command is not available".to_string();
        }

        PostgresSidecarStatus {
            state: guard.state.clone(),
            bind_addr: guard.config.bind_addr.clone(),
            command: guard.config.command.clone(),
            note: guard.note.clone(),
        }
    }

    pub fn restart(&self) -> PostgresSidecarStatus {
        let _ = self.stop();
        self.start()
    }
}

fn is_local_bind(bind: &str) -> bool {
    bind == "127.0.0.1"
        || bind == "localhost"
        || bind.starts_with("127.0.0.1:")
        || bind.starts_with("localhost:")
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

        assert_eq!(status.state, RuntimeLifecycleState::Unavailable);
        assert_eq!(status.bind_addr, "127.0.0.1");
    }

    #[test]
    fn supervisor_reports_unavailable_for_missing_binary() {
        let handle = PostgresSidecarHandle::new(PostgresSidecarConfig {
            command: "definitely-missing-postgres-binary".to_string(),
            args: vec![],
            bind_addr: "127.0.0.1".to_string(),
            workdir: None,
        });

        let started = handle.start();
        assert_eq!(started.state, RuntimeLifecycleState::Unavailable);
    }

    #[test]
    fn supervisor_rejects_non_loopback_bind() {
        let handle = PostgresSidecarHandle::new(PostgresSidecarConfig {
            command: "postgres".to_string(),
            args: vec![],
            bind_addr: "0.0.0.0:5432".to_string(),
            workdir: None,
        });

        let started = handle.start();
        assert_eq!(started.state, RuntimeLifecycleState::Failed);
        assert!(started.note.contains("bind must be loopback"));
    }
}
