# Phase 0–1 Validation Evidence

Task ID: `20260507-2054-production-ready-roadmap`

## Scope Implemented

- Rust workspace and crate skeletons for backend/domain/runtime/auth/db/storage/package/sidecar boundaries.
- Axum backend skeleton with health/readiness/version and install/system/package route stubs.
- Core SQL migration skeleton for installation, auth, catalog, orders/payment proofs, packages/audit, plus package migration boundary.
- SvelteKit + Tailwind web scaffold with route groups for storefront, admin, and install panel.
- Tauri v2 desktop shell scaffold for local process-supervisor/control-panel direction.
- VPS/container baseline files: Dockerfile, Docker Compose, systemd unit, deployment notes.
- Environment and git hygiene baseline: `.env.example`, `.gitignore`.

## Validation Commands

| Command | Result | Notes |
| --- | --- | --- |
| `rustc --version` | Failed on host | `zsh:1: command not found: rustc`; host Rust toolchain unavailable. |
| `cargo --version` | Failed on host | `zsh:1: command not found: cargo`; host cargo unavailable. |
| Docker `rust:1.82` `cargo build --workspace` | Failed | Transitive crate `base64ct 1.8.3` requires Cargo edition 2024 support. |
| Docker `rust:1.85` `cargo generate-lockfile` | Partial/insufficient | Dependency resolution selected transitive crates requiring Rust 1.88; raised workspace `rust-version` and Docker base to Rust 1.88. |
| Docker `rust:1.88` `cargo generate-lockfile` | Passed | Generated `Cargo.lock`. |
| Docker `rust:1.88` `cargo build --workspace --exclude market-desktop` | Passed | Non-desktop backend/crates compile. Full workspace initially failed on Linux desktop system dependency `dbus-1` missing in base container. |
| Docker `rust:1.88` `cargo test --workspace --exclude market-desktop` | Passed | No behavior tests yet, but all non-desktop crate test harnesses and doctests pass. |
| Docker `rust:1.88` `cargo fmt --check` | Passed after formatting | `rustfmt` component installed in container; code formatted. |
| Docker `rust:1.88` `cargo clippy --workspace --exclude market-desktop --all-targets -- -D warnings` | Passed | Non-desktop backend/crates pass clippy. |
| `npm install` in `apps/web` | Passed with audit warning | Installed web dependencies and generated `package-lock.json`. |
| `npm run check` in `apps/web` | Passed | `svelte-check found 0 errors and 0 warnings`. |
| `npm run build` in `apps/web` | Passed | SvelteKit production build completed with `@sveltejs/adapter-node`. |
| `npm audit --audit-level=moderate` in `apps/web` | Failed/Warning | 3 low severity vulnerabilities from SvelteKit transitive `cookie <0.7.0`; no moderate/high/critical findings, but command exits non-zero because `npm audit fix --force` proposes breaking downgrade. |

## Known Gaps

- Rust/Tauri compile validation on host is blocked until `rustc`/`cargo` are installed. Container validation is used as fallback.
- `market-desktop` compile validation needs Linux Tauri/WebKit/DBus dev packages in the validation image/host; non-desktop workspace passes.
- Backend routes are skeleton responses; DB pool, install lock, package gate, CSRF/session, and RBAC enforcement are later phases.
- SvelteKit UI is a scaffold only; it does not yet call backend APIs or implement real catalog/checkout/payment proof flows.
- Tauri shell is a scaffold only; PostgreSQL/backend/cloudflared process supervision is not wired yet.
- Dockerfile uses `rust:1.88-bookworm` because dependency resolution currently selects transitive crates requiring Rust 1.88.
- `npm audit` reports low severity transitive vulnerability in current SvelteKit dependency chain; do not use `npm audit fix --force` because it proposes breaking dependency changes.

## Security Notes

- `.env` and secret-like files are ignored; `.env.example` contains only placeholder values.
- Runtime config exposes a safe summary and redacts `DATABASE_URL` from debug output.
- Payment proof privacy is not production-complete yet; storage and route scaffolds must be followed by protected media/RBAC implementation.
- Tunnel remains opt-in in docs and desktop/API skeleton responses.
