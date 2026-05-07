# Discovery Evidence — Production Ready Roadmap

Task ID: `20260507-2054-production-ready-roadmap`

## Files Inspected

- `README.md` — project summary, open-core model, deployment modes, roadmap.
- `PRD.md` — product requirements, MVP scope, locked decisions, acceptance criteria.
- `TRD.md` — architecture, runtime, module boundaries, API, security, deployment, testing.
- `ERD.md` — data model, order grouping, payment proofs, file objects, indexes.
- `DESIGN.md` — project UI direction for storefront, admin, install/control panel.
- `AGENTS.md` — agent rules, authoritative docs, security/storage/package constraints.

## Project Patterns Found

- Repository is documentation/planning-only; no implementation code exists yet.
- Target architecture is a monorepo with `apps/backend`, `apps/web`, `apps/desktop`, and Rust `crates/core-*` boundaries.
- Single Axum backend core must serve both desktop and VPS runtimes.
- Tauri is a desktop process supervisor/control panel, not domain logic.
- PostgreSQL is source of truth; desktop bundles PostgreSQL, VPS uses service/container DB.
- `cloudflared` is bundled and opt-in; tunnel must not expose PostgreSQL/internal ports.
- Storage MVP is local; S3-compatible adapter is roadmap.
- Open-core paid package model starts with capability registry, feature gate, package migration boundary, and remote entitlement direction.

## Reuse Candidates

- Existing planning docs provide the source of truth for scope and architecture; no code utilities exist yet.
- Reuse document-defined module boundaries before creating implementation modules.
- Reuse `DESIGN.md` for all UI planning rather than inventing visual direction.
- Reuse `ERD.md` statuses and indexes as initial migration guidance.

## Commands / Docs / Advisors Checked

- Local project discovery through `README.md`, `PRD.md`, `TRD.md`, `ERD.md`, `DESIGN.md`, `AGENTS.md`.
- `product-architect` advisor for MVP slicing, epics, user flows, production rollout.
- `security-privacy-reviewer` advisor for auth, RBAC, payment proof privacy, uploads, tunnel, secrets, entitlement.
- `release-engineer` advisor for CI/CD, desktop packaging, VPS systemd/container, backups, release gates.
- `librarian` advisor for version-sensitive docs to verify during implementation: Axum, sqlx, Tauri v2 sidecars/packaging, SvelteKit deployment, Tailwind/TweakCN, Docker deployment.

## User Decisions Captured

- UI MVP upload bukti transfer: one active file per order.
- Payment proof access: use RBAC policy, not a fixed hardcoded role-only rule.
- VPS production baseline: support container deployment from the beginning in addition to binary/systemd.
- Desktop installer target: all platforms from the beginning.
- Paid package model: remote entitlement direction.

## Constraints

- Do not change stack without explicit user approval.
- Must keep docs in sync when decisions change.
- Base open source must run without paid packages.
- MVP must not implement dynamic plugin runtime/arbitrary code execution.
- Security must assume public internet exposure through tunnel/reverse proxy.
- Current task is planning only; implementation files must not be edited by planner.

## Research Gate Decision

- Local discovery: required and completed.
- Official docs/context7/librarian: required for version-sensitive stack; advisor provided docs to verify at implementation time.
- GitHub: not required for this planning artifact because no upstream issue/source dependency drives current roadmap.
- Brave/web search: not required; current docs and advisor inputs are sufficient for implementation roadmap.
- Browser/screenshot capture: not required because this is production implementation planning, not visual parity work; screenshots are planned later once UI is runnable.

## Risks

- Desktop all-platform packaging from the beginning expands CI, signing, notarization, sidecar compatibility, and smoke-test scope.
- Container support from the beginning expands ops documentation, volume, healthcheck, and secret handling requirements.
- Remote entitlement introduces external dependency risk; base must remain offline-capable.
- Payment proof contains sensitive financial/customer information; RBAC, private storage, and audit must be implemented before production.
- Bundled PostgreSQL desktop lifecycle can cause data corruption if shutdown/update/backup flows are weak.
- Open questions remain around exact payment proof RBAC matrix and paid package distribution details.
