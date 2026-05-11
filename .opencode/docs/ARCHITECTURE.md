# Architecture Map

Canonical sources:

- `TRD.md` is the technical source of truth.
- `PRD.md` defines product scope.
- `ERD.md` defines entities, relations, indexes, and canonical statuses.

Current locked stack:

- Desktop: Tauri v2
- Backend: Rust + Axum
- Database: PostgreSQL
- Frontend: SvelteKit
- Styling: Tailwind CSS + TweakCN
- Storage MVP: local storage
- Tunnel: bundled `cloudflared` sidecar

Runtime model:

- Desktop local-hosted and VPS hosted share one backend core.
- Tauri is a shell/process supervisor, not a domain layer.
- Axum is authoritative for auth, RBAC, orders, storage access, package gates, and business rules.
- PostgreSQL is the source of truth.

Project structure and boundaries are defined in `TRD.md` and the root `Cargo.toml` workspace.
Do not change stack, runtime mode, package boundary, or canonical statuses without updating the root docs.
