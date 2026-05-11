# Architecture

Canonical architecture sources are `TRD.md`, `ERD.md`, and `README.md`.

Current project shape:

- `apps/backend` — Axum backend authority for auth, RBAC, install lock, package gates, protected media, and business rules.
- `apps/web` — SvelteKit storefront/admin/install UI. UI consumes APIs; it is not authorization authority.
- `apps/desktop/src-tauri` — Tauri shell/process-control lane; do not move marketplace domain logic here.
- `crates/core-*` — domain, app, auth, db, storage, runtime, installer, packages.
- `crates/sidecar-*` — bundled PostgreSQL and `cloudflared` lifecycle adapters.

Deployment posture:

- Desktop local-hosted: Tauri + bundled PostgreSQL + Axum + optional `cloudflared`.
- VPS hosted: separate SvelteKit web service and Axum API service, with PostgreSQL and local storage.

Validation reality today:

- Rust host toolchain may be unavailable locally; non-desktop Rust validation is expected via Docker `rust:1.88`.
- Web validation uses `apps/web` `npm run check` and `npm run build`.
