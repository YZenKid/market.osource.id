# Desktop Runtime Supervision Evidence

Task ID: `20260512-0746-unfinished-mvp-completion-plan`

Status: **Implemented with targeted Rust validation; host desktop crate integration remains environment-blocked**

## Scope

- backend desktop lifecycle commands
- PostgreSQL sidecar lifecycle commands
- loopback-only safety guard
- typed runtime lifecycle states

## Changed files

- `apps/desktop/src-tauri/src/main.rs`
- `crates/sidecar-postgres/src/lib.rs`

## What changed

- Added Tauri commands:
  - `desktop_runtime_status`
  - `desktop_runtime_start`
  - `desktop_runtime_stop`
  - `desktop_runtime_restart`
- Added typed lifecycle state usage:
  - `stopped`
  - `starting`
  - `running`
  - `stopping`
  - `failed`
  - `unavailable`
- Added backend supervisor with env-configurable command/bind/workdir.
- Added PostgreSQL sidecar supervisor with env-configurable command/bind/workdir.
- Added loopback-only safety checks for backend and PostgreSQL sidecar start commands.
- Ordering now starts PostgreSQL sidecar before backend, and stops backend before PostgreSQL sidecar.

## Validation executed

- `cargo test -p sidecar-postgres`
  - Result: **passed** (`3 passed`)
- `cargo test -p market-desktop`
  - Result: **blocked by host system GTK/GLib pkg-config dependencies**
  - Missing libraries included:
    - `gio-2.0`
    - `gdk-3.0`
    - `glib-2.0`
    - `gobject-2.0`

## Residual risks

- Command surface is implemented, but end-to-end desktop crate compilation cannot be proven on this host without GTK/GLib development packages.
- No frontend desktop UI wiring was added in this slice; only Tauri backend command surface is ready.
