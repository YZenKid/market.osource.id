# Gate A+B Validation Evidence

Task ID: `20260508-0548-release-gate-plan`

## Scope Implemented

- Gate A docs sync for locked decisions:
  - VPS uses separate SvelteKit service from Axum API service.
  - Desktop target is all-platform RC.
  - Seller proof-file access requires explicit, brand/order-scoped permission and audit.
  - Product price source is `product_variants`; simple products use default/internal variant.
  - README stale “implementation has not started” section updated.
- Gate B foundation additions:
  - GitHub Actions CI baseline for Rust, web, docs, and secret scan.
  - `AppState` now carries optional DB pool, local storage provider, package gate, and package registry.
  - `/ready` now reports database, migration state, storage, install state, package registry, and package compatibility checks.
  - `/api/system/runtime` now includes safe config summary, readiness, DB/storage/package information.
  - Migration lock IDs and migration path helpers now have unit tests.
  - Live PostgreSQL migration smoke tests were added for fresh core migration apply, runner-level advisory-lock release, and advisory-lock behavior; they are ignored by default and run with `TEST_DATABASE_URL` in validation.
  - Readiness behavior without DB has a unit test.
  - VPS docs and Compose now model separate `backend` and `web` services.
  - Added reverse proxy baseline for one public origin routing to separate services, with prefix-preserving `/api/*` and `/media/*` proxy examples.
  - Authoritative docs define canonical payment proof permissions: `payment_proof.view_assigned`, `payment_proof.verify`, and `payment_proof.reject`.

## Validation Commands

| Command | Result | Notes |
| --- | --- | --- |
| Docker `rust:1.88` `cargo fmt --check` | Passed | Host Rust still unavailable; Docker fallback used. |
| Docker `rust:1.88` `cargo clippy --workspace --exclude market-desktop --all-targets -- -D warnings` | Passed | Desktop excluded until Linux Tauri system deps are added. |
| Docker `rust:1.88` `cargo test --workspace --exclude market-desktop` | Passed | Includes new readiness and migration helper tests. |
| Docker PostgreSQL 16 + Docker `rust:1.88` `TEST_DATABASE_URL=... cargo test -p core-db -- --ignored` | Passed | Validates fresh core migration apply, package migration boundary record, runner-level advisory-lock release, and advisory lock behavior against disposable PostgreSQL container. |
| Local `rustc --version` / `cargo --version` | Passed | User installed local Rust toolchain: `rustc 1.95.0`, `cargo 1.95.0`. |
| Local `cargo tauri --version` | Passed | Installed user-level `tauri-cli 2.11.1` via `cargo install tauri-cli --version 2.11.1 --locked`. |
| Local `cargo fmt --check` | Passed | Runs with host Rust toolchain. |
| Local `cargo clippy --workspace --exclude market-desktop --all-targets -- -D warnings` | Passed | Updated `RuntimeMode` default derive to satisfy newer Rust 1.95 Clippy. |
| Local `cargo test --workspace --exclude market-desktop` | Passed | Includes readiness and migration helper tests. |
| Local PostgreSQL container + `TEST_DATABASE_URL=... cargo test -p core-db -- --ignored` | Passed | Validates live DB migration tests through host Cargo against PostgreSQL 16 exposed on localhost. |
| Fedora 43 container with Tauri Linux deps + `cargo check -p market-desktop` | Passed | Validates `market-desktop` with `dbus-devel`, `gtk3-devel`, `webkit2gtk4.1-devel`, `librsvg2-devel`, and related deps installed in container. |
| Local `npx tauri info` | Partial | Rust/Cargo/Tauri CLI are installed, but host OS still lacks `webkit2gtk-4.1` and `rsvg2`; `sudo dnf install ...` requires interactive password, so host-native desktop validation remains blocked by system deps. |
| `npm run check` in `apps/web` | Passed | `svelte-check found 0 errors and 0 warnings`. |
| `npm run build` in `apps/web` | Passed | SvelteKit production build completed. |
| `npm audit --audit-level=moderate` in `apps/web` | Non-blocking warning | After updating compatible SvelteKit/adapter/node/vite/typescript packages, npm still reports 3 low-severity transitive SvelteKit `cookie@0.6.0` findings. Latest SvelteKit still depends on `cookie@0.6.0`; `npm audit fix --force` proposes breaking downgrade and was not run. |

## Known Gaps

- Gate B is not fully production-complete: DB pool is optional and install-state service remains Gate C work. `/ready` now reports missing install state as not ready instead of masking it.
- Host-native full `market-desktop` validation still requires Linux Tauri/WebKit/rsvg system dependencies (`webkit2gtk4.1-devel`, `librsvg2-devel`, etc.) installed with sudo. Containerized Fedora validation for `cargo check -p market-desktop` now passes.
- CI has not run remotely yet; local validation is the current evidence.
- `npm audit` low-severity transitive findings remain tracked until SvelteKit upgrades its internal `cookie` dependency or npm advisory metadata changes.
- The Compose `web` service uses a local bind mount for smoke/dev modeling; release image packaging remains later Gate J work.
- Migration advisory locking now uses one acquired PostgreSQL session for lock, migration execution, and unlock for core migrations; package boundary migration also executes on the locked session.

## Security Notes

- CORS example now includes separate web origin instead of assuming only backend origin.
- Reverse proxy docs route protected media to Axum, not SvelteKit static assets, and preserve `/api/*`/`/media/*` path prefixes.
- Payment proof seller access remains explicit-permission only in docs; permission names are canonicalized, while implementation of actual permission routes is Gate D/F work.
- `.env.example` remains placeholder-only and does not contain real secrets.
