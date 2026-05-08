# Complete Release Gate Plan — market.osource.id

Task ID: `20260508-0548-release-gate-plan`

## Goal

Membuat plan implementasi lengkap dari scaffold Phase 0–1 saat ini sampai **MVP Release Candidate** dan **public v1.0 release gate** untuk `market.osource.id`: self-hosted, installable, open-core, multi-brand marketplace dengan desktop all-platform RC, VPS binary/systemd, VPS container, separate SvelteKit service, Axum API authority, manual transfer, private payment proof, brand-scoped RBAC, variant-only pricing, dan release evidence yang bisa diaudit.

Plan ini adalah source of truth operasional untuk membawa repository dari scaffold ke release gate. Plan ini melengkapi roadmap besar `.opencode/plans/20260507-2054-production-ready-roadmap.md` dengan gate, evidence, blockers, dan sequencing yang lebih konkret.

## Non-goals

- Tidak mengimplementasikan source code dalam task planning ini.
- Tidak mengganti stack yang sudah dikunci.
- Tidak membuat SaaS hosted multi-tenant mode.
- Tidak membuat dynamic plugin runtime/arbitrary code loading untuk MVP.
- Tidak memasukkan online payment, delivery integration, promo engine, analytics advanced, cloud backup scheduler, theme builder, customer accounts, seller self-registration, atau native mobile ke base MVP.
- Tidak mengklaim production-ready tanpa evidence release gate.

## Scope

### In Scope

- Blueprint freeze dan docs synchronization.
- Backend foundation sampai business services siap diuji.
- Separate VPS topology: SvelteKit service + Axum API service.
- Desktop all-platform RC: Windows, macOS, Linux.
- Auth/session/CSRF/CORS/RBAC/rate limit.
- Install lock dan first Super Admin setup.
- Catalog/variant-only pricing/storefront/cart/checkout/order lintas brand.
- Payment proof upload, private media, explicit seller proof permission, audit.
- Local storage hardening.
- Package registry/capability/feature gate/remote entitlement abstraction.
- UI completion mengikuti `DESIGN.md`.
- CI/CD, security scans, migration tests, backup/restore dry-run.
- Systemd/container/desktop smoke tests.
- SBOM, checksums, release manifest, release notes, rollback docs.

### Out of Scope for MVP Release Gate

- Payment gateway automatic confirmation.
- Shipping/courier integration.
- Advanced analytics and chart-heavy dashboard.
- Cloud backup scheduler implementation.
- Theme builder/custom storefront sections.
- S3-compatible adapter as active default.
- App auto-update production system, except update/migration safety docs and warnings.

## Requirements

### Locked Product/Technical Decisions

- One installation/domain equals one marketplace.
- Seller is created by Super Admin; no seller self-registration.
- One seller may manage multiple brands through `brand_members`.
- Product price is **variant-only**; simple product UX creates a default/internal variant.
- One order may contain products from multiple brands and must use `order_brand_groups`.
- Checkout MVP uses manual transfer.
- Payment proof UI shows one active proof per order while schema can retain proof history.
- Seller proof-file access requires explicit permission; default seller sees status/order information only.
- VPS production uses separate SvelteKit service from Axum API service.
- Desktop RC targets Windows, macOS, and Linux from the beginning.
- Base open source must work without paid package entitlement.
- Paid packages use registry/capability/feature gate/remote entitlement abstraction, not dynamic plugin runtime.

### Security Requirements

- Backend Axum is authority for auth, RBAC, package gates, protected media, storage auth, order transitions, and install lock.
- Cookie/session auth across separate SvelteKit/Axum must have explicit CORS/CSRF contract.
- Payment proof is private by storage location, metadata visibility, route authorization, audit, logs, diagnostics, and screenshots.
- No wildcard CORS with credentials.
- Tunnel opt-in only and must not expose PostgreSQL/debug/internal ports.
- Secrets must not appear in UI, logs, diagnostics, crash reports, frontend bundle, or release artifacts.

## Acceptance Criteria

### MVP Functional Acceptance

- Fresh desktop install can start bundled PostgreSQL, Axum backend, SvelteKit UI, and detect bundled `cloudflared` off by default.
- Fresh VPS install works through binary/systemd and container with separate SvelteKit/Axum services behind a documented reverse proxy topology.
- Install setup creates one Super Admin and locks server-side.
- Super Admin can create brands, sellers, assignments, categories, products, variants, and product images.
- Seller can manage only assigned brands.
- Storefront can browse products, select variants, cart, checkout, upload proof, and track order.
- Multi-brand checkout creates one order and one `order_brand_group` per brand.
- Payment proof upload stores private file and changes payment status to `waiting_payment_verification` without auto-confirming.
- Super Admin can view/verify/reject proof; seller cannot view proof file unless explicit permission and brand/order scope pass.
- Package disabled/unlicensed features are denied server-side.

### Release Acceptance

- CI green for Rust, web, migrations, API/security tests, E2E, scans, and packaging jobs relevant to the release branch/tag.
- No unresolved critical/high security vulnerabilities or secret leaks.
- Backup/restore dry-run passes for VPS systemd, container, and desktop data directory at least at RC/internal evidence level.
- Desktop Windows/macOS/Linux RC smoke tests pass.
- VPS systemd and container smoke tests pass.
- Release artifacts have SBOM, checksums, manifest, release notes, known limitations, upgrade notes, and rollback notes.

## Existing Patterns/Reuse

- Reuse Phase 0–1 scaffold committed in `4d8e40d feat: scaffold marketplace foundation`.
- Reuse Rust workspace/crate boundaries from `TRD.md` and existing `crates/*`.
- Reuse SvelteKit route grouping `(storefront)`, `(admin)`, `(install)` from current scaffold.
- Reuse `DESIGN.md` for UI tone, semantic colors, layout, accessibility, and responsive evidence.
- Reuse `ERD.md` for canonical entities/statuses/index direction.
- Reuse prior production roadmap `.opencode/plans/20260507-2054-production-ready-roadmap.md` as high-level phase map.
- No KiloCode/project utilities beyond current scaffold solve the remaining implementation; extend current modules rather than creating a separate architecture.

## Constraints

- Do not change stack without explicit user approval.
- Do not serve payment proof as static public file.
- Do not introduce product-level default price unless docs and migrations are intentionally changed.
- Do not let SvelteKit become authorization authority.
- Do not let Tauri contain marketplace domain logic.
- Do not make package MVP a plugin runtime.
- Do not claim close UI quality without browser evidence and `DESIGN.md` conformance.
- Public production release should not be treated the same as unsigned/internal RC.

## Risks

| Risk | Impact | Mitigation / Gate |
| --- | --- | --- |
| Separate SvelteKit/Axum auth breaks cookies/CSRF | Login or mutation failures, CSRF risk | Same external origin reverse proxy where possible; explicit CORS allowlist; CSRF and Origin tests before RC. |
| Seller proof-file permission leaks sensitive data | Privacy breach | Explicit permission `payment_proof.view_assigned`, brand/order scope, audit, negative tests. |
| Variant-only pricing confuses simple product UX | Seller friction | Simple product UI manages one default/internal variant. |
| Desktop all-platform scope delays release | Packaging/signing complexity | Split internal all-platform RC from public v1 signing/notarization readiness. |
| Bundled PostgreSQL data corruption | Data loss | Graceful process supervisor, backup warning, migration lock, restore dry-run. |
| Container volume misconfiguration | Lost DB/media/proofs | Named volumes, restart persistence tests, backup/restore evidence. |
| Entitlement privacy overreach | Trust/privacy risk | Minimal pseudonymous payload, no customer/order/proof PII, documented grace/offline behavior. |
| Open-core scope creep | MVP delay/security risk | Registry/capability/feature gate only; no dynamic loader. |
| README/docs drift | Agents implement stale assumptions | Gate A docs sync must fix stale `README.md` section and new locked decisions. |

## Decisions/Assumptions

### User-Answered Decisions

- Desktop release target: **all-platform RC**.
- VPS serving model: **separate SvelteKit service** from Axum.
- Payment proof file: seller may view only with **explicit permission**.
- Product pricing: **variant-only price**.

### Assumptions

- Same external origin through reverse proxy is preferred for VPS even when services are separate: app routes to SvelteKit, `/api/*` and protected media to Axum.
- Public v1.0 requires an acceptable signing/notarization/distribution policy; internal RC may be unsigned if clearly labeled.
- Remote entitlement receives only pseudonymous instance/package/version/license fingerprint metadata, not customer/order/payment proof data.
- Seller proof file permissions are disabled by default and must be assigned by Super Admin.

### Open Questions Remaining

- Exact desktop artifact formats per OS: Windows `.msi`/`.exe`, macOS `.dmg`, Linux `.deb`/AppImage/RPM.
- Exact signing/notarization bar for public v1.0.
- Entitlement grace-period duration and payload contract details.
- Whether category remains brand-scoped only for MVP or later global categories are planned.

## Production Blueprint Summary

### MVP Slice

Production-hardened base marketplace: installable desktop/VPS, Super Admin + seller RBAC, brand-scoped catalog, variant-only products, responsive storefront, multi-brand checkout, manual transfer, private proof review, local storage, package awareness, deployment/recovery/security release evidence.

### Epics/User Flows

1. Blueprint freeze and docs sync.
2. Platform foundation and CI baseline.
3. Install/runtime for desktop and VPS.
4. Auth/session/RBAC.
5. Catalog/storefront with variant-only price.
6. Cart/checkout/multi-brand orders.
7. Payment proof and order operations.
8. Package/open-core foundation.
9. UI completion and responsive validation.
10. Security/privacy/backup/ops hardening.
11. Packaging and release candidate.
12. Public v1.0 release gate.

### Data/API Outline

- Data follows `ERD.md` with `users`, `roles`, `sessions`, `brands`, `brand_members`, `products`, `product_variants`, `file_objects`, `orders`, `order_brand_groups`, `payment_proofs`, `packages`, `package_migrations`, `audit_events`.
- Price source of truth is `product_variants.price`; cart/order snapshots must use variant data.
- API groups: `/api/install`, `/api/auth`, `/api/storefront`, `/api/admin`, `/api/system`, `/api/packages`, protected `/media` or equivalent.
- SvelteKit consumes APIs but does not decide authorization.

### SaaS/RBAC Considerations

- Not SaaS hosted; one marketplace per install.
- RBAC is still central: Super Admin, Seller, Guest/tracking token.
- Seller proof-file access is explicit permission plus brand/order scope.

### UI/Design Readiness

- `DESIGN.md` is the first UI authority.
- Substantial UI work must include desktop/tablet/mobile screenshot evidence and accessibility notes.
- UI must expose status and next actions clearly without relying on color only.

### AI Boundaries

- No AI/LLM/RAG/embedding system in MVP.

### Mobile Constraints

- Native mobile out of scope.
- Storefront and checkout must work on mobile web; admin at least desktop/tablet usable.

### Security/Privacy Checklist

- Auth/session/CSRF/CORS contract for separate services.
- Brand-scoped RBAC and explicit proof permission.
- Protected payment proof route with audit.
- Upload validation and private storage.
- Install lock and rate limits.
- Secret/log/diagnostics/frontend bundle redaction.
- Tunnel opt-in and safe routing.
- Entitlement privacy contract.

### Release/Ops Checklist

- CI/CD full gates.
- Migration lock and fixture upgrade tests.
- Backup/restore dry-run.
- Systemd/container/desktop smoke tests.
- SBOM/checksums/release manifest.
- Release/rollback/known limitation docs.

## TDD/Test Plan

TDD is required because remaining work touches production logic, auth/RBAC, checkout/order state, uploads, private media, migrations, package gates, runtime setup, deployment, and security-sensitive behavior.

### Existing Test Patterns

- Current scaffold has no meaningful behavior tests yet; Docker `rust:1.88` non-desktop test harnesses pass.
- Establish tests incrementally inside Rust crates, API integration tests with PostgreSQL, Svelte/component tests, and Playwright E2E.
- Reuse `.opencode/evidence/.../phase-0-1-validation.md` evidence style for gate outputs.

### First Failing/Regression Test by Gate

- Gate A: docs consistency check fails while README/PRD/TRD/ERD disagree with locked decisions.
- Gate B: `/ready` returns not ready when DB/storage/migrations unavailable; concurrent migration lock test fails before implementation.
- Gate C: second install setup request must fail after install lock.
- Gate D: Seller A direct API request to Brand B and proof file must fail.
- Gate E: multi-brand cart checkout must create one order and multiple `order_brand_groups`.
- Gate F: Playwright baseline captures missing required UI states/viewports.
- Gate G: restore dry-run must preserve private proof protection and seller isolation.
- Gate H: release artifact manifest must fail if any artifact lacks checksum/SBOM entry.
- Gate I: public release blocked if RC smoke/security docs/signing status are incomplete.

### Green Step

Implement the minimum behavior to pass each gate test while respecting crate/module boundaries and authority rules.

### Refactor Step

After tests pass, consolidate duplicated status/RBAC/storage/runtime logic into `core-*` crates and simplify API/UI composition without moving authority to frontend/Tauri.

### Edge Cases

- Double-submit install setup.
- Separate frontend/backend credentialed CORS failure.
- Seller removed from brand while session active.
- Seller with proof permission accessing unassigned brand proof.
- Simple product default variant hidden from customer UX but canonical in DB.
- Concurrent checkout stock validation.
- Payment proof replacement preserving history/audit.
- Tunnel enabled but `cloudflared` fails.
- Entitlement server unreachable.
- Migration interrupted then retried.
- Container restart with persisted DB/media volumes.
- Desktop app killed during PostgreSQL operation.

### Commands

Host commands once toolchain exists:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm --prefix apps/web ci
npm --prefix apps/web run check
npm --prefix apps/web run build
docker build -t market-osource:local .
docker compose up --build
```

Fallback until host Rust exists:

```bash
docker run --rm -v "$PWD:/workspace" -w /workspace rust:1.88-bookworm cargo test --workspace --exclude market-desktop
```

Desktop validation must add platform/system deps and run Tauri builds/smokes per OS.

## Implementation Steps

### Gate A — Blueprint Freeze and Docs Sync

1. Update `README.md` stale development status section to reflect scaffold.
2. Update `PRD.md`, `TRD.md`, `ERD.md`, and `AGENTS.md` with locked decisions: separate VPS SvelteKit service, all-platform desktop RC, explicit seller proof permission, variant-only pricing.
3. Define permission names such as `payment_proof.view_assigned`, `payment_proof.verify`, `payment_proof.reject`.
4. Define simple product UX as one default/internal variant.
5. Define release matrix and artifact naming scheme.

**Gate A pass:** docs agree and no MVP-blocking decision remains except artifact format/signing/grace details tracked as release risks.

### Gate B — Platform Foundation and CI Baseline

1. Install/standardize Rust 1.88+ toolchain in dev/CI.
2. Add CI workflows for Rust, web, migrations, security scans, and docs.
3. Wire `AppState` with DB pool, local storage provider, package registry/gate, install state service.
4. Make `/ready` check DB, storage, migration state, install state, package compatibility.
5. Add migration runner tests with advisory lock and fresh DB apply.
6. Add structured logging/request IDs and redaction baseline.

**Gate B pass:** CI baseline green, non-desktop workspace validates on host/CI, readiness reflects real dependencies.

### Gate C — Install and Runtime

1. Implement `core-installer` preflight/setup/install lock.
2. Implement Super Admin creation with Argon2id and server-side sessions.
3. Implement VPS install panel with separate SvelteKit UI and Axum install API.
4. Add reverse proxy/dev docs for SvelteKit service + Axum API.
5. Implement Tauri process supervisor skeleton into real bundled PostgreSQL/backend lifecycle.
6. Implement cloudflared detection/status/start/stop as opt-in only.
7. Add systemd units for backend and web service; update Compose with separate `backend` and `web` services.

**Gate C pass:** install works once per runtime, lock blocks repeat setup, desktop DB binds loopback, tunnel off by default.

### Gate D — Auth, Session, CSRF, CORS, RBAC

1. Implement users/roles/sessions repositories and APIs.
2. Implement session cookie flags and CSRF token issuance/verification.
3. Implement strict CORS allowlist for separate frontend/backend and local desktop origins.
4. Implement rate limits for login, install, checkout, upload, order lookup.
5. Implement RBAC middleware/helpers for Super Admin, Seller, Guest/tracking token.
6. Implement explicit proof permission support but default seller proof file access to denied.
7. Add audit events for auth, assignment, role/permission changes.

**Gate D pass:** negative auth/RBAC/CSRF/CORS/security tests green.

### Gate E — Commerce MVP

1. Implement brand/category/product/variant/image repositories/services.
2. Implement variant-only product model and simple product default variant UX.
3. Implement public storefront listing/detail/search/filter.
4. Implement guest cart and checkout transaction.
5. Implement order snapshots, `order_brand_groups`, status transitions, and order history.
6. Implement admin/seller order views with brand-scoped filtering.

**Gate E pass:** full customer browse/cart/checkout/order tracking and seller assigned fulfillment flow work with multi-brand orders.

### Gate F — Payment Proof and Protected Media

1. Implement upload validation: size, extension, declared MIME, magic bytes, image dimensions, reject SVG, traversal prevention.
2. Store payment proof in private storage root with `file_objects` metadata.
3. Implement protected media route with auth, RBAC, brand/order scope, explicit seller proof permission, and audit.
4. Implement Super Admin proof review/verify/reject.
5. Implement seller status-only default view and explicit permission file view.
6. Implement one-active proof UI while preserving historical rows.

**Gate F pass:** unauthorized proof file access denied, proof not static-public, audit events recorded, proof workflow works.

### Gate G — Package/Open-Core Foundation

1. Make package registry DB-backed or reliably seeded from static definitions.
2. Implement capability/feature gate checks server-side.
3. Implement package migration state handling.
4. Implement admin package status UI.
5. Implement remote entitlement abstraction and cache states: active, expired, grace, unreachable, invalid.
6. Document and test entitlement privacy payload and offline/base behavior.

**Gate G pass:** base works offline/no entitlement; paid endpoints denied when disabled/unlicensed; entitlement outage does not break base flows.

### Gate H — UI/UX Completion and Evidence

1. Complete install wizard, storefront, cart/checkout, order tracking, admin dashboard, catalog, orders, proof review, packages, system/tunnel/storage screens.
2. Follow `DESIGN.md` semantic palette, labels, status badges, responsive layout, and no generic UI patterns.
3. Add loading/empty/error/success/permission/offline states.
4. Add Playwright E2E for setup, catalog, checkout, proof upload, verify/reject, fulfillment.
5. Capture stable screenshots for desktop `1440x1200`, tablet `768x1024`, mobile `390x844` for key flows.
6. Run accessibility review for headings, labels, focus, keyboard, contrast, reduced motion.

**Gate H pass:** UI usable and evidenced across required surfaces/viewports.

### Gate I — Ops, Backup, Recovery, Security Hardening

1. Add security headers and body limits.
2. Add diagnostics export with redaction tests.
3. Add threat model docs for payment proof, tunnel, desktop DB, entitlement, install lock.
4. Add backup/restore docs and scripts/checklists.
5. Run restore dry-run for VPS systemd, container, and desktop data directory.
6. Add migration fixture upgrade tests and interrupted migration recovery notes.
7. Add container/dependency/static/secret scans.

**Gate I pass:** no production data recovery/security blocker remains.

### Gate J — Release Candidate Packaging

1. Build VPS backend binary artifact.
2. Build SvelteKit web service artifact.
3. Build container image with non-root user, labels, healthcheck, persistent volume docs.
4. Build desktop RC artifacts for Windows/macOS/Linux.
5. Run systemd smoke test.
6. Run container smoke and restart persistence tests.
7. Run desktop all-platform smoke matrix.
8. Generate SBOM, checksums, release manifest, release notes, known limitations, upgrade and rollback docs.

**Gate J pass:** all release artifacts and deployment-mode evidence exist; unresolved critical/high issues block promotion.

### Gate K — Public v1.0 Release Gate

1. Review RC feedback and fix blockers.
2. Confirm docs match actual behavior.
3. Confirm signing/notarization/distribution policy is acceptable for public release.
4. Confirm security/privacy/release/quality gates pass.
5. Tag release and publish artifacts/docs.
6. Monitor pilot installs and collect production bug reports.

**Gate K pass:** public v1.0 can be released; otherwise keep status as internal RC.

## Expected Files to Change

### Documentation / Planning

- `README.md`
- `PRD.md`
- `TRD.md`
- `ERD.md`
- `DESIGN.md`
- `AGENTS.md`
- `.opencode/plans/**`
- `.opencode/evidence/**`
- `docs/deployment/**`
- `docs/security/**`
- `docs/packages/**`
- release notes, rollback, upgrade, SBOM/manifest/checksum docs

### Backend / Rust

- `Cargo.toml`, `Cargo.lock`
- `apps/backend/**`
- `crates/core-domain/**`
- `crates/core-app/**`
- `crates/core-api/**`
- `crates/core-auth/**`
- `crates/core-db/**`
- `crates/core-storage/**`
- `crates/core-runtime/**`
- `crates/core-installer/**`
- `crates/core-packages/**`
- `crates/sidecar-postgres/**`
- `crates/sidecar-cloudflared/**`
- `migrations/core/**`
- `migrations/packages/**`

### Frontend / Desktop / Deployment

- `apps/web/**`
- `apps/desktop/**`
- `Dockerfile`, `docker-compose.yml`, future compose/proxy files
- `.github/workflows/**` or chosen CI equivalent
- packaging/signing/release configs

## Agent/Tool Routing

- `@artifact-planner`: update durable plans before major phase/gate shifts.
- `@product-architect`: MVP slicing, acceptance criteria, release narrative changes.
- `@security-privacy-reviewer`: auth, CSRF/CORS, RBAC, payment proof, uploads, tunnel, secrets, entitlement, diagnostics.
- `@release-engineer`: CI/CD, deployment, migration, backup/restore, packaging, SBOM/checksums, rollback, release gates.
- `@oracle`: architecture tradeoffs for serving topology, migrations, desktop supervisor, package boundary.
- `@librarian`: official docs for Axum, sqlx, Tauri v2, SvelteKit, Tailwind/TweakCN, Docker/systemd tooling.
- `@designer`: substantial UI/UX plans/reviews following `DESIGN.md`.
- `@fixer`: implementation after this plan, with TDD.
- `@quality-gate`: final conformance/risk review after each non-trivial phase and before commit/PR/release.

## Validation Commands

### Rust

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Until host Rust exists:

```bash
docker run --rm -v "$PWD:/workspace" -w /workspace rust:1.88-bookworm cargo test --workspace --exclude market-desktop
```

### Web

```bash
npm --prefix apps/web ci
npm --prefix apps/web run check
npm --prefix apps/web run test
npm --prefix apps/web run build
npm --prefix apps/web audit --audit-level=moderate
```

### Database

```bash
sqlx migrate run
sqlx migrate info
```

Add test DB commands for fresh, fixture upgrade, concurrent lock, and restore validation.

### API / E2E / Browser

```bash
npm --prefix apps/web run test:e2e
```

Playwright evidence must use wait-stabilize-scroll-settle workflow and matching viewports.

### Container / VPS

```bash
docker build -t market-osource:local .
docker compose up --build
curl -f http://localhost:8080/health
curl -f http://localhost:8080/ready
```

Update commands once separate `backend` and `web` services are finalized.

### Desktop

```bash
cargo tauri build
```

Run per-platform smoke tests for Windows/macOS/Linux RC artifacts.

### Release

```bash
sha256sum <artifacts>
syft <artifact-or-image> -o spdx-json
grype <image-or-sbom>
```

Exact SBOM/scanner commands may change based on chosen tooling.

## Evidence Requirements

### Evidence Created/Kept for This Plan

- `.opencode/evidence/20260508-0548-release-gate-plan/discovery.md` — discovery, decisions, advisory sources, research gate.

### Gate Evidence Required Later

- Docs consistency diff/summary for Gate A.
- CI run links/logs for each gate.
- Rust fmt/clippy/test output.
- Web check/test/build/audit output.
- Migration fresh/fixture/concurrent lock logs.
- API negative security test output.
- Playwright E2E reports and screenshots.
- Accessibility review notes.
- Container build/scan/smoke/restart logs.
- Systemd smoke logs.
- Desktop Windows/macOS/Linux smoke reports.
- Backup/restore dry-run reports.
- `/health`, `/ready`, `/version` outputs for desktop/systemd/container.
- Diagnostics/log redaction review.
- Entitlement privacy test output.
- SBOM files, checksums, release manifest.
- Release notes, known limitations, upgrade notes, rollback notes.
- Final `quality-gate` result.

## Done Criteria

This release-gate plan is done when:

- Primary plan exists at `.opencode/plans/20260508-0548-release-gate-plan.md`.
- Discovery evidence exists and records docs inspected, user decisions, advisors, and research gate.
- Plan includes gate-by-gate implementation sequence through public v1.0.
- Plan includes TDD/test strategy, validation commands, evidence requirements, risks, assumptions, open questions, and agent routing.

The product is release-ready only when Gate K passes with required evidence and no unresolved blocker.

## Final Planning Summary

### Artifacts Created / Kept

- Primary plan: `.opencode/plans/20260508-0548-release-gate-plan.md`
- Kept evidence: `.opencode/evidence/20260508-0548-release-gate-plan/discovery.md`

Evidence is kept because it records material user decisions and advisory inputs needed by future implementation/release agents.

### Key Decisions

- Desktop release target is all-platform RC.
- VPS production uses separate SvelteKit and Axum services.
- Seller proof-file access is explicit-permission only, brand/order scoped, and audited.
- Product price is variant-only; simple product uses default/internal variant.
- Release is split into MVP RC and public v1.0 gate.

### Assumptions

- Reverse proxy should present one public origin where possible even with separate internal services.
- Public v1.0 signing/notarization expectations are stricter than internal RC.
- Remote entitlement does not receive customer/order/payment proof PII.
- Release gates are evidence-based and can block promotion even if checklist items are nominally implemented.

### Remaining Open Questions

- Exact desktop artifact formats per OS.
- Public v1.0 signing/notarization/distribution policy.
- Remote entitlement grace-period duration and exact payload contract.
- Category scope roadmap beyond brand-scoped MVP.

### Readiness for Implementation

Ready for implementation handoff. Next recommended implementation plan/task is Gate A + Gate B: docs sync, CI baseline, real `AppState`, DB/storage readiness, migration lock tests, and separate service topology docs.

### Cleanup Performed

No draft artifacts were created. Discovery evidence is intentionally retained. No implementation/source files were edited by this planner.
