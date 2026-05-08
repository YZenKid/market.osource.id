# Discovery Evidence — Release Gate Plan

Task ID: `20260508-0548-release-gate-plan`

## Files Inspected

- `README.md` — current project status, scaffold summary, deployment modes, roadmap, known gaps.
- `PRD.md` — product scope, users, MVP expectations, manual transfer/payment proof, open-core package model.
- `TRD.md` — architecture, crate boundaries, runtime/deployment/security/testing expectations.
- `ERD.md` — data model, canonical statuses, `order_brand_groups`, `payment_proofs`, `file_objects`, package state.
- `DESIGN.md` — UI authority for storefront/admin/install/control-panel visual language and accessibility expectations.
- `AGENTS.md` — repository constraints, security/storage/package rules, testing expectations.
- `.opencode/plans/20260507-2054-production-ready-roadmap.md` — prior production-ready phased roadmap and validation gates.
- `.opencode/evidence/20260507-2054-production-ready-roadmap/phase-0-1-validation.md` — Phase 0–1 scaffold validation evidence.

## Current Project State Found

- Phase 0–1 scaffold exists and has been committed as `4d8e40d feat: scaffold marketplace foundation`.
- Rust workspace and core crate boundaries exist, but production behavior is mostly skeleton.
- SvelteKit web skeleton exists with storefront/admin/install routes, but it is not API-connected.
- Tauri desktop shell skeleton exists, but bundled PostgreSQL/backend/cloudflared supervision is not implemented.
- VPS/container/systemd baseline files exist, but production smoke tests are not complete.
- Host lacks `rustc`/`cargo`; Docker `rust:1.88` validation is the current fallback.
- Non-desktop Rust build/test/fmt/clippy passed in Docker; full Tauri build needs platform/system deps.
- Web `npm run check` and `npm run build` passed.
- README still contains a stale lower section saying implementation has not started; plan should include docs consistency cleanup.

## User Decisions Captured During Question Gate

- Desktop release target: **all-platform RC** — Windows/macOS/Linux artifacts from the beginning, with public signing/notarization as a separate release readiness concern.
- VPS serving model: **separate SvelteKit service** from Axum for VPS production.
- Payment proof RBAC: **seller may view proof file only with explicit permission**; default seller sees status/order data only.
- Product pricing: **variant-only price**; simple products use a default/internal variant.

## Advisory Sources

- `release-engineer` advisory: CI/CD phases, systemd/container readiness, all-platform desktop RC matrix, migration/backup/restore, SBOM/checksums/manifest, rollback, release gate evidence.
- `security-privacy-reviewer` advisory: separate SvelteKit/Axum CORS/CSRF/session risks, RBAC matrix, payment proof private media, uploads, tunnel, secrets, entitlement privacy, audit, negative tests.
- `product-architect` advisory: MVP slice, must-have/defer list, user flows, gate sequence, product risks, release narrative.

## Research Gate Decision

- Local project discovery: required and completed.
- Official docs/context7/librarian: not required for this artifact because the request is release planning from existing project docs; version-sensitive API details should be verified during each implementation phase.
- GitHub: not required; no upstream repo issue/source behavior drives this planning artifact.
- Brave/web search: not required; current project docs and release/security/product advisors are sufficient.
- Browser/screenshot capture: not required for planning; screenshots are included as release evidence requirements once UI is runnable.

## Reuse Candidates

- Reuse existing `.opencode/plans/20260507-2054-production-ready-roadmap.md` as the broad production roadmap.
- Reuse current Phase 0–1 scaffold rather than replanning bootstrap from zero.
- Reuse `DESIGN.md` for all UI release criteria.
- Reuse `ERD.md` canonical statuses and relations.
- Reuse existing validation evidence format under `.opencode/evidence/` for future gates.

## Constraints and Risks

- Stack is fixed: Tauri v2, Rust Axum, PostgreSQL, SvelteKit, Tailwind CSS + TweakCN, local storage MVP, bundled `cloudflared`.
- Separate SvelteKit/Axum service on VPS raises CORS/CSRF/cookie/reverse-proxy complexity.
- Seller proof-file permission is security-sensitive and must be explicit, brand-scoped, and audited.
- Variant-only pricing requires default/internal variant UX for simple products.
- All-platform desktop RC expands packaging, system dependencies, signing/notarization, sidecar, and smoke-test scope.
- Payment proofs, customer contact/address, sessions, tunnel tokens, DB URLs, package license keys, and diagnostics are privacy/security-sensitive.
- Release readiness must be evidence-based; checklist-only status is insufficient.
