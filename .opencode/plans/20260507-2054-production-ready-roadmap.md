# Production Ready Implementation Plan — market.osource.id

Task ID: `20260507-2054-production-ready-roadmap`

## Goal

Membuat roadmap implementasi lengkap untuk membawa `market.osource.id` dari fase dokumentasi menjadi produk production-ready: self-hosted, installable, multi-brand marketplace dengan desktop local-hosted mode, VPS hosted mode, responsive web storefront/admin, manual transfer + upload bukti transfer, order lintas brand, local storage, bundled PostgreSQL/cloudflared, dan fondasi open-core paid package berbasis remote entitlement.

Plan ini adalah source of truth implementasi bertahap sampai production release. Implementasi harus tetap mengikuti `README.md`, `PRD.md`, `TRD.md`, `ERD.md`, `DESIGN.md`, dan `AGENTS.md`.

## Non-goals

- Tidak mengimplementasikan kode dalam plan ini.
- Tidak membuat dynamic plugin runtime atau arbitrary code execution untuk paid package MVP.
- Tidak membangun mobile native Android/iOS.
- Tidak membuat SaaS hosted marketplace multi-tenant.
- Tidak mengaktifkan S3-compatible adapter sebagai default MVP.
- Tidak memasukkan online payment, delivery integration, analytics advanced, promo engine, atau theme builder ke base MVP.
- Tidak mengklaim production-ready sebelum security, release, backup/restore, migration, dan deployment gates terpenuhi.

## Scope

### In Scope

- Struktur monorepo dan workspace backend/web/desktop.
- Rust Axum backend core dan crate boundaries.
- SvelteKit web app untuk storefront, admin, install panel.
- Tauri v2 desktop shell/process supervisor.
- Bundled PostgreSQL lifecycle untuk desktop.
- Bundled `cloudflared` sidecar opt-in.
- VPS deployment binary + systemd dan container sejak awal.
- PostgreSQL migrations core/package dengan lock.
- Auth/session/CSRF/RBAC brand-scoped.
- Brand, seller assignment, category, product, variant, image.
- Guest cart, checkout, order lintas brand, `order_brand_groups`.
- Payment proof upload dengan satu file aktif di UI MVP, schema tetap multi-proof.
- Payment proof private/protected access via RBAC.
- Local storage + `file_objects` metadata.
- Audit log untuk event sensitif.
- Package registry, feature gate, package migration boundary, remote entitlement-ready state.
- System status, health/readiness/version endpoints.
- CI/CD, tests, security scans, release artifacts, container image, desktop installers all-platform.
- Backup/restore docs and validation.
- Production release gates.

### Out of Scope for Base MVP

- Payment gateway automatic confirmation.
- Courier delivery integration.
- Advanced analytics dashboards.
- Cloud backup scheduler implementation.
- Theme builder implementation.
- Dynamic package artifact loading.
- Global marketplace categories unless docs are updated.

## Requirements

### Product Requirements

- Satu instalasi adalah satu marketplace.
- Satu domain adalah satu marketplace.
- Super Admin membuat seller; seller tidak daftar sendiri.
- Satu seller dapat mengelola banyak brand melalui `brand_members`.
- Produk berada di bawah brand dan dapat memiliki category/variant.
- Customer dapat checkout lintas brand dalam satu order.
- Order lintas brand menggunakan status global dan `order_brand_groups.fulfillment_status` per brand.
- Checkout MVP memakai manual transfer.
- Customer upload bukti transfer; proof tidak otomatis mengkonfirmasi order.
- UI MVP hanya satu bukti transfer aktif per order.
- Payment proof access dikendalikan lewat RBAC backend.
- Base open source berjalan tanpa paid packages.
- Paid packages diarahkan ke remote entitlement.

### Technical Requirements

- Axum backend sebagai authority untuk auth/RBAC/order/storage/package gate.
- Tauri hanya shell/process supervisor/control panel.
- PostgreSQL sebagai source of truth.
- Desktop PostgreSQL bind ke `127.0.0.1`.
- Tunnel opt-in, tidak aktif otomatis.
- Local storage default; S3-compatible adapter roadmap.
- Server-side session, Argon2id, CSRF untuk cookie mutation.
- Feature gate server-side untuk paid package capability.
- Migration core dan package terpisah.
- Secrets tidak masuk UI/log/diagnostics/frontend bundle.
- `DESIGN.md` wajib diikuti untuk UI.

## Acceptance Criteria

### MVP Functional Acceptance

- Fresh desktop install dapat menjalankan bundled PostgreSQL, backend, web UI, dan optional `cloudflared`.
- Fresh VPS install dapat dijalankan via binary+systemd dan container.
- Install panel mengunci setup setelah super admin dibuat.
- Super Admin dapat membuat brand, seller, dan assignment multi-brand.
- Seller hanya dapat melihat/mengelola brand yang assigned.
- Seller dapat membuat category/product/variant dan upload image valid.
- Storefront responsive dapat browse product/brand, cart, checkout, upload proof, dan track order.
- Order lintas brand tersimpan dengan `order_brand_groups` dan snapshot order items.
- Payment proof tersimpan private dan hanya bisa diakses melalui protected route dengan RBAC.
- Admin dapat verify/reject payment proof dan memicu status canonical.
- Seller fulfillment hanya untuk brand group yang assigned.
- Package registry menampilkan status package dan feature gate menolak fitur disabled/unlicensed.

### Production Acceptance

- CI green untuk format/lint/test/build/security scan/migration test.
- Negative security tests pass: seller isolation, proof private access, CSRF, install lock, upload invalid, package disabled.
- `/health`, `/ready`, `/version` valid di desktop, systemd, dan container.
- Backup dan restore docs diuji minimal satu dry-run.
- Migration lock dan idempotency diuji.
- Desktop all-platform installer smoke test pass.
- Container image scan pass tanpa critical/high unresolved.
- Binary+systemd smoke test pass.
- Docker Compose/container smoke test pass.
- Release manifest, SBOM/checksum, release notes, rollback notes tersedia.
- Tidak ada secret/PII sensitif dalam logs, diagnostics, UI, atau frontend bundle.

## Existing Patterns/Reuse

- Reuse `TRD.md` monorepo structure and crate boundaries.
- Reuse `ERD.md` schema, relations, status canonical, and index recommendations.
- Reuse `DESIGN.md` for UI visual language, layout, state, and validation guidance.
- Reuse `AGENTS.md` as contributor/agent constraints.
- No existing code utilities or implementation patterns exist yet; project is planning/documentation-only.
- Prefer `sqlx` per TRD recommendation; verify version-specific migration/query behavior before implementation.
- Prefer SvelteKit route grouping `(storefront)`, `(admin)`, `(install)` per TRD/DESIGN.

## Constraints

- Stack fixed unless user explicitly changes it.
- Production assumes public exposure through Cloudflare Tunnel or reverse proxy.
- Base open source must remain useful offline/local without paid packages.
- Remote entitlement may affect paid packages only, not core base availability.
- Desktop all-platform target increases release matrix from the start.
- Container support from the start requires explicit volume/secret/healthcheck strategy.
- Payment proof contains sensitive financial/customer data.
- Planner artifacts only; implementation occurs later through implementation agents.

## Risks

| Risk | Impact | Mitigation |
| --- | --- | --- |
| Desktop all-platform packaging too broad | Release delays, signing/notarization complexity | Separate artifact build from signing readiness; smoke-test matrix per OS; prioritize internal RC artifacts before public signing. |
| Bundled PostgreSQL corrupt/shutdown issue | Data loss | Safe process supervisor, graceful shutdown, backup docs, pre-upgrade warnings, loopback-only bind. |
| Container volumes misconfigured | Lost storage/payment proof data | Compose with named volumes, explicit storage paths, restore validation, docs warnings. |
| Payment proof privacy leak | PII/financial leak | Private `file_objects`, protected route, RBAC, audit download, no public static proof. |
| Seller cross-brand access | Tenant/brand data leak | Service-layer authorization, query scoping by `brand_members`, negative tests. |
| Remote entitlement outage | Paid package confusion | Cache entitlement with expiry/grace; base remains fully functional; status visible. |
| Package system grows into plugin runtime | MVP scope explosion/security risk | MVP limited to registry/capabilities/feature gate/migration boundary. |
| Migration/upgrade breaks production data | Data loss/outage | Migration lock, backup warning, fixture upgrade tests, no destructive auto migration. |
| Tunnel exposes internal ports | Internet attack surface | Tunnel opt-in, route only HTTP app, verify no PostgreSQL/debug/internal port exposure. |
| UI generic/inconsistent | Poor trust/usability | Follow `DESIGN.md`; require screenshot validation when runnable. |

## Decisions/Assumptions

### Locked Decisions

- UI MVP payment proof allows one active file per order.
- Schema remains multi-proof per order for future extensibility.
- Payment proof access uses RBAC policy; exact permission matrix must be implemented and audited.
- VPS supports both binary+systemd and container from the beginning.
- Desktop installer targets Windows, macOS, and Linux from the beginning.
- Paid package entitlement direction is remote entitlement.
- SvelteKit and Axum deployment shape must be finalized during foundation; default recommendation is one SvelteKit app with route groups and Axum-owned API.

### Open Questions to Resolve Before Coding Specific Features

- Payment proof RBAC matrix detail: default Super Admin only, or seller permission toggle for assigned order groups?
- Desktop public release signing readiness: are unsigned/internal test artifacts acceptable before public production?
- Exact desktop package formats per OS: Windows `.msi`/`.exe`, macOS `.dmg`, Linux `.deb`/AppImage/RPM.
- Remote entitlement data-sharing policy and grace period duration.
- Whether product price exists only on variants or product may have default price for no-variant products.
- Whether category remains brand-scoped only for MVP or global categories are roadmap.

### Assumption-first Defaults if User Does Not Decide Further

- Payment proof file content visible to Super Admin by default; seller sees payment status only unless explicit permission is added.
- Desktop RC may build all-platform artifacts before full code signing/notarization is production-grade; public v1 requires signing plan.
- VPS container uses external PostgreSQL service/container with persistent volume; app container does not embed PostgreSQL.
- Remote entitlement sends minimal pseudonymous instance/package/version data, not customer/order/payment proof data.

## Production Blueprint Summary

### MVP Slice

MVP is a production-hardened base open-source marketplace: installable desktop/VPS, Super Admin + seller RBAC, multi-brand catalog, responsive storefront, cart/checkout manual transfer, private payment proof, order brand grouping, local storage, system status, tunnel config, package awareness, CI/release/deployment hardening.

### Epics / User Flows

1. Platform foundation: workspace, config, migrations, health, CI.
2. Install/runtime: desktop setup, VPS install panel, PostgreSQL/cloudflared sidecars, container/systemd.
3. Auth/RBAC: sessions, CSRF, roles, seller assignments.
4. Catalog/storefront: brand/category/product/variant/images, browse/search.
5. Checkout/order: guest cart, order snapshots, order brand groups.
6. Payment proof/order ops: single-active upload UI, protected proof access, verify/reject, fulfillment.
7. Package foundation: registry, feature gate, remote entitlement-ready state.
8. Release/security: tests, scans, backup/restore, packaging, docs.

### Data/API Outline

- Data follows `ERD.md`: `users`, `sessions`, `brands`, `brand_members`, `products`, `product_variants`, `file_objects`, `orders`, `order_brand_groups`, `payment_proofs`, `packages`, `audit_events`.
- API follows `TRD.md`: `/api/install`, `/api/auth`, `/api/storefront`, `/api/admin`, `/api/system`, `/api/packages`, protected `/media`.
- Status canonical follows PRD/TRD/ERD; no new status without docs update.

### SaaS/RBAC Considerations

- Not SaaS hosted; one installation = one marketplace.
- RBAC is still critical: Super Admin vs Seller vs Guest, brand-scoped seller access, payment proof permission.
- Remote entitlement is only for paid package activation, not tenant hosting.

### UI/Design Readiness

- `DESIGN.md` exists and must be first design authority.
- Implementation must plan storefront/admin/install state matrices and screenshot validation.
- Substantial UI must include desktop/tablet/mobile evidence before done.

### AI Boundaries

- No AI/LLM/RAG system in MVP; no AI-specific architecture needed.

### Mobile Constraints

- Native mobile out of scope.
- Mobile web responsive required for storefront and checkout; admin at least desktop/tablet usable.

### Security/Privacy Checklist

- Auth/session/CSRF/rate limit.
- Brand-scoped RBAC.
- Payment proof private + RBAC + audit.
- Upload hardening.
- Install lock.
- Secret storage/redaction.
- Tunnel opt-in.
- CORS/security headers.
- Remote entitlement privacy minimization.

### Release/Ops Checklist

- CI/CD full test/build/security gates.
- Desktop all-platform packaging smoke.
- VPS binary+systemd and container deployment docs/tests.
- Migration lock + backup/restore validation.
- Release manifest/SBOM/checksum/signing plan.
- Health/readiness/version endpoints.

### Validation Plan

- Red/Green/Refactor per phase.
- Unit, integration, API, E2E, security negative, browser visual, packaging smoke, restore dry-run, release candidate checklist.

## TDD/Test Plan

TDD is required because this project includes production logic, auth/RBAC, uploads, checkout/order behavior, migrations, package gates, runtime setup, and security-sensitive flows.

### Existing Test Patterns

- No implementation or test suite exists yet.
- Define test patterns during Phase 0:
  - Rust unit tests inside crates.
  - Integration tests using test PostgreSQL container/service.
  - API tests against Axum app state.
  - SvelteKit unit/component tests.
  - Playwright E2E/browser tests after UI exists.
  - Packaging smoke tests in CI/release runners.

### First Failing/Regression Tests by Phase

- Phase 0: `/ready` returns not ready when DB/storage unavailable; migration lock prevents concurrent migrations.
- Phase 1: install setup creates one Super Admin then rejects second setup.
- Phase 2: Seller A cannot read/update Brand B through direct API.
- Phase 3: product draft not visible in storefront; invalid image upload rejected.
- Phase 4: checkout with multi-brand cart creates one order and multiple `order_brand_groups`.
- Phase 5: unauthenticated payment proof file access is denied; seller proof access follows RBAC.
- Phase 6: disabled/unlicensed package route is denied server-side.
- Phase 7: diagnostics export redacts DB URL, tunnel token, license key, proof path, customer full address.
- Release: restore dry-run preserves private proof access and package compatibility state.

### Green Step

Implement minimum behavior to pass the failing tests for each phase while respecting module boundaries and docs.

### Refactor Step

Refactor duplicated auth/storage/status/package/runtime code into `core-*` crates only after tests pass; keep domain independent from Axum/SQLx/Tauri/filesystem.

### Edge Cases

- Double-submit install setup.
- Concurrent checkout stock validation.
- Cart item from inactive/draft product.
- Seller assigned to multiple brands and removed from one brand.
- Order with partially shipped brand groups.
- Payment proof upload replacing active proof while preserving audit/history.
- Tunnel enabled but cloudflared fails.
- Remote entitlement server unreachable.
- Container restart after migration partially fails.
- Desktop app killed during PostgreSQL operation.

### Commands to Establish During Implementation

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `sqlx migrate run` or embedded migration test command, depending final workflow
- package manager install with frozen lockfile
- web lint/typecheck/test/build commands
- Playwright E2E command
- container build and smoke command
- Tauri build/smoke commands per platform

## Implementation Steps

### Phase 0 — Blueprint Freeze and Project Bootstrap

1. Update PRD/TRD/ERD/AGENTS with newest decisions: proof one active UI, proof RBAC matrix, container from start, all-platform desktop, remote entitlement.
2. Pin initial versions for Rust, Axum, sqlx, Tauri v2, SvelteKit, Tailwind, TweakCN after official docs verification.
3. Create monorepo structure from TRD.
4. Add root workspace files: `Cargo.toml`, package workspace config, `.gitignore`, `.env.example` without secrets.
5. Create CI skeleton for docs, Rust, web, and security scans.
6. Add architecture decision records if needed under `docs/`.

### Phase 1 — Backend Foundation

1. Implement `apps/backend` Axum bootstrap.
2. Implement `core-runtime` config/mode/path/secrets interfaces.
3. Implement `/health`, `/ready`, `/version`.
4. Implement structured logging and request id.
5. Implement `core-db` pool and migration runner with lock.
6. Create initial core migrations: installation, users/auth, marketplace/catalog, orders/proofs, packages/audit.
7. Add migration tests and readiness tests.

### Phase 2 — Install and Runtime

1. Implement `core-installer` setup flow and install lock.
2. Implement Super Admin creation with Argon2id.
3. Implement VPS install API and install panel shell.
4. Implement desktop Tauri shell skeleton and process supervisor.
5. Implement `sidecar-postgres` lifecycle for bundled PostgreSQL.
6. Implement `sidecar-cloudflared` detection/status/start/stop with opt-in.
7. Implement container image and Docker Compose baseline.
8. Implement systemd unit example and deployment docs.

### Phase 3 — Auth and RBAC

1. Implement roles, users, sessions, CSRF, rate limit.
2. Implement login/logout/session revoke.
3. Implement `brand_members` and seller assignment service.
4. Implement authorization helpers in `core-app`.
5. Add negative API tests for seller isolation.
6. Add audit events for auth and assignment.

### Phase 4 — Catalog and Storefront

1. Implement brand/category/product/product_variant/product_image repositories and services.
2. Implement `core-storage` local provider and upload validation.
3. Implement `file_objects` metadata.
4. Implement admin brand/seller/product UI following `DESIGN.md`.
5. Implement storefront brand/product listing/detail/search/filter.
6. Add responsive validation and image upload tests.

### Phase 5 — Cart, Checkout, Orders

1. Implement guest cart with anonymous token.
2. Implement checkout transaction with stock validation and snapshots.
3. Implement `orders`, `order_brand_groups`, `order_items` creation.
4. Implement tracking token order lookup.
5. Implement admin/seller order views with brand-scoped filtering.
6. Implement status aggregation rules and `order_status_history`.
7. Add multi-brand order integration/E2E tests.

### Phase 6 — Payment Proof and Operations

1. Implement customer proof upload route with one-active UI behavior.
2. Store proof as private local file with `file_objects` metadata.
3. Implement protected media route with RBAC.
4. Implement Super Admin verify/reject workflow.
5. Implement seller proof access according to final RBAC matrix.
6. Implement proof access/download audit.
7. Add upload hardening and unauthorized access tests.

### Phase 7 — Package and Entitlement Foundation

1. Implement `core-packages` registry and feature gate.
2. Implement package migration state table usage.
3. Implement package status admin UI.
4. Implement remote entitlement client abstraction.
5. Implement entitlement cache states: active, expired, grace, unreachable, invalid.
6. Ensure base flow does not require entitlement connectivity.
7. Add tests for disabled/unlicensed/unreachable scenarios.

### Phase 8 — UI Completion and Browser Validation

1. Complete install wizard, storefront, admin dashboard, system status, package settings.
2. Apply `DESIGN.md` semantic palette, typography, states, and responsive behavior.
3. Add Playwright E2E for setup, catalog, checkout, proof upload, verification, fulfillment.
4. Capture desktop/tablet/mobile screenshots for substantial UI surfaces.
5. Fix accessibility issues: labels, focus, keyboard, contrast, reduced motion.

### Phase 9 — Production Hardening

1. Add security headers and CORS allowlist.
2. Add request body limits and rate limits.
3. Add diagnostics export with redaction.
4. Add backup/restore docs and scripts where safe.
5. Add restore validation checklist.
6. Add container image scan, secret scan, dependency audit, Semgrep/static scan.
7. Add threat model docs for tunnel, proof, entitlement, desktop DB.

### Phase 10 — Packaging and Release Candidate

1. Build desktop artifacts for Windows/macOS/Linux.
2. Add signing/notarization plan; public v1 requires signing readiness.
3. Build VPS binary artifacts.
4. Build container image with non-root user and healthcheck.
5. Generate SBOM/checksums/release manifest.
6. Run artifact smoke tests.
7. Run migration upgrade tests from fixture DB.
8. Run backup/restore dry-run.
9. Produce release notes and known limitations.

### Phase 11 — Production v1.0 Gate

1. Verify all acceptance criteria.
2. Verify security gates with no critical/high blockers.
3. Verify release gates across desktop/systemd/container.
4. Tag release.
5. Publish artifacts and docs.
6. Monitor pilot installs and collect production bug reports.

## Expected Files to Change

### Documentation

- `README.md`
- `PRD.md`
- `TRD.md`
- `ERD.md`
- `DESIGN.md`
- `AGENTS.md`
- `docs/deployment/*`
- `docs/security/*`
- `docs/packages/*`

### Future Implementation Structure

- `Cargo.toml`, `Cargo.lock`
- `package.json`, JS workspace lockfile/config
- `apps/backend/**`
- `apps/web/**`
- `apps/desktop/**`
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
- `.github/workflows/**` or chosen CI equivalent
- `docker/**`, `Dockerfile`, `docker-compose.yml`
- release/signing/packaging configs

## Agent/Tool Routing

- `@artifact-planner`: planning artifact updates before major implementation shifts.
- `@product-architect`: PRD/MVP/user-flow slicing changes.
- `@oracle`: architecture review for runtime, package, migration, or complex tradeoffs.
- `@security-privacy-reviewer`: auth, RBAC, uploads, payment proof, tunnel, entitlement, secrets.
- `@release-engineer`: CI/CD, packaging, deployment, migration, backup/restore, release gates.
- `@librarian`: official docs/version-sensitive stack behavior.
- `@designer`: substantial UI/UX design, `DESIGN.md` updates, visual review.
- `@fixer`: bounded implementation and TDD execution after plan handoff.
- `@quality-gate`: final conformance/risk review after non-trivial implementation.

## Validation Commands

Commands are provisional until implementation chooses exact package manager and versions.

### Rust

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

### Database

```bash
sqlx migrate run
sqlx migrate info
```

If using embedded migrations and offline query checks, add `sqlx prepare --workspace --check` or the final chosen equivalent.

### Web

```bash
<package-manager> install --frozen-lockfile
<package-manager> run lint
<package-manager> run check
<package-manager> run test
<package-manager> run build
```

### E2E / Browser

```bash
<package-manager> run test:e2e
```

Visual evidence must include desktop/tablet/mobile screenshots for substantial UI.

### Container / VPS

```bash
docker build -t market-osource:local .
docker compose up --build
curl -f http://localhost:<port>/health
curl -f http://localhost:<port>/ready
```

### Desktop

```bash
cargo tauri build
```

Run per-platform smoke tests for setup/start/stop/upload/tunnel opt-in/log redaction.

## Evidence Requirements

### Planning Evidence Kept

- `.opencode/evidence/20260507-2054-production-ready-roadmap/discovery.md` — local discovery and advisor summary.

### Implementation Evidence to Produce Later

- CI run links/logs for each phase.
- Migration test output.
- API negative test output.
- Playwright screenshots: desktop `1440x1200`, tablet `768x1024`, mobile `390x844` for storefront, checkout, admin, install wizard.
- Accessibility notes for key UI states.
- Container smoke logs and healthcheck output.
- Desktop per-OS smoke test notes.
- Backup/restore dry-run evidence.
- Release manifest, checksums, SBOM, vulnerability scan reports.
- Security/privacy review signoff before production.

## Done Criteria

The project is production-ready when all of the following are true:

- Docs reflect actual implementation and no blocking open questions remain for MVP.
- All MVP product acceptance criteria in `PRD.md` pass.
- All technical acceptance criteria in `TRD.md` pass.
- Data model and statuses match `ERD.md` migrations.
- UI follows `DESIGN.md` and has screenshot validation for key states/viewports.
- Base open source works without paid packages.
- Remote entitlement is isolated to paid package state and does not break base mode.
- Security gates pass with no critical/high blockers.
- Backup/restore is documented and dry-run validated.
- Desktop all-platform artifacts smoke-tested.
- VPS binary+systemd and container deployment smoke-tested.
- Release artifacts include version, manifest, checksums/SBOM, and release notes.

## Final Planning Summary

### Artifacts Created / Kept

- Primary plan: `.opencode/plans/20260507-2054-production-ready-roadmap.md`
- Kept evidence: `.opencode/evidence/20260507-2054-production-ready-roadmap/discovery.md`

Evidence is kept because it records inspected docs, user decisions, advisor routing, and research gate decisions useful for implementation agents.

### Key Decisions

- Production roadmap is phased from blueprint freeze through production v1.0.
- UI MVP has one active payment proof per order, while schema remains multi-proof.
- Payment proof access is RBAC-based and must be audited.
- VPS supports container deployment from the beginning in addition to binary/systemd.
- Desktop targets Windows/macOS/Linux from the beginning.
- Paid packages use remote entitlement direction without dynamic plugin runtime in MVP.

### Assumptions

- Super Admin is default role allowed to view payment proof content until RBAC matrix grants seller permission explicitly.
- Container deployment uses external PostgreSQL service/container with persistent volumes.
- Public production desktop release will eventually require signing/notarization; internal RC artifacts may precede full signing if user approves.
- Entitlement service receives no customer/order/payment proof PII.

### Remaining Open Questions

- Exact RBAC matrix for seller payment proof visibility.
- Exact desktop package formats and signing policy per OS.
- Entitlement grace period and data-sharing contract.
- Product default price vs variant-only price.
- Brand-scoped categories only vs future global categories.

### Readiness for Implementation

Ready for implementation planning handoff with caution: before coding payment proof review, packaging release workflows, or remote entitlement, the remaining open questions above should be resolved or implemented with the assumption-first defaults.

### Cleanup Performed

No stale drafts were created. Discovery evidence is intentionally retained. No source implementation files were edited by this planner.
