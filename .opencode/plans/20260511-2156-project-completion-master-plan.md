# Rencana Implementasi Lengkap — Penyelesaian MVP `market.osource.id`

Task ID: `20260511-2156-project-completion-master-plan`

## Goal

Menyusun plan implementasi lengkap, implementation-ready, dan evidence-driven untuk menyelesaikan `market.osource.id` sesuai goals dokumen repo dengan fokus **base open-source MVP**: self-hosted multi-brand marketplace yang dapat diinstal pada desktop local-hosted maupun VPS hosted, memiliki storefront responsive, admin dashboard operator, install flow aman, checkout manual transfer dengan upload bukti transfer private, order lintas brand, local storage, package awareness, serta jalur UI/UX lengkap yang nantinya mengeksekusi design generation dan image generation memakai MCP dan agent yang tersedia.

Plan ini adalah source of truth implementasi/handoff. Plan ini **tidak mengeksekusi coding, design generation, atau image generation sekarang**, tetapi menuliskan lane, gate, artifact, dan urutan kerja lengkap agar execution berikutnya bisa berjalan aman.

## Non-goals

- Tidak mengedit source implementation di luar `.opencode/` pada fase planner ini.
- Tidak menjalankan `@designer`, `@fixer`, atau `@visual-asset-generator` dari planner ini.
- Tidak menyelesaikan paid module di fase ini selain boundary/foundation planning.
- Tidak membuat mobile native Android/iOS.
- Tidak membuat SaaS hosted multi-tenant.
- Tidak mengaktifkan online payment, delivery integration, advanced analytics, theme builder, atau plugin runtime sebagai bagian base MVP.
- Tidak mengklaim UI final, generated assets final, atau visual parity final tanpa execution lane dan screenshot evidence.

## Scope

### In Scope

- Penyelesaian MVP base open-source sesuai `README.md`, `PRD.md`, `TRD.md`, `ERD.md`, dan `DESIGN.md`.
- Backend core Axum, crate boundaries, migration, auth/session/CSRF/RBAC, install lock, package gate, protected media, commerce core.
- Web SvelteKit untuk storefront, admin, install panel.
- Desktop Tauri control panel + runtime supervision plan.
- VPS binary+systemd dan container release path.
- UI/UX production polish plan untuk semua core surfaces.
- Image generation/design generation workflow plan dengan MCP/agent execution lane yang sesuai.
- TDD/Test plan dan release/ops evidence gates.

### Out of Scope

- Paid module implementation nyata.
- Public binary signing/notarization penuh sebagai syarat satu-satunya MVP code-complete; ini masuk readiness lanjutan/RC-public lane.
- Full backup scheduler/cloud sync implementation.
- S3-compatible storage activation sebagai storage default.

## Requirements

### Product Requirements

- Satu install = satu marketplace.
- Satu domain = satu marketplace.
- Super Admin membuat seller; seller tidak self-register.
- Satu seller dapat mengelola banyak brand melalui assignment.
- Produk berada di bawah brand dan dapat memiliki category/variant.
- Customer dapat checkout lintas brand dalam satu order.
- Order lintas brand dipecah menjadi `order_brand_groups`.
- Checkout MVP memakai manual transfer.
- Customer dapat upload bukti transfer.
- Bukti transfer tidak otomatis mengonfirmasi pembayaran.
- Storefront, admin, install, dan desktop control panel harus usable dan konsisten.
- Base open source harus tetap berjalan tanpa paid package.

### Technical Requirements

- Backend Axum menjadi authority untuk auth, RBAC, install lock, media access, order/payment state, dan package gate.
- Tauri hanya shell/process supervisor/control panel.
- PostgreSQL adalah source of truth.
- Desktop PostgreSQL bind ke loopback.
- Tunnel `cloudflared` opt-in, default off.
- Session server-side dengan password hashing Argon2id.
- Semua cookie-authenticated mutations memakai CSRF.
- Payment proof private dan hanya lewat protected route.
- Diagnostics/log/status tidak boleh membocorkan secret atau PII sensitif.
- UI harus mengikuti `DESIGN.md` dan reuse primitive yang sudah ada.

### UI/UX & Image Requirements

- Semua core surfaces harus punya visual spec yang jelas.
- Semua image-heavy atau illustration-heavy section harus punya **Image Generation Decision** eksplisit.
- Asset generation yang dibutuhkan harus legal style-equivalent, bukan copy aset reference terlarang.
- Visual completion baru sah setelah reference/current/final capture evidence tersedia.

## Acceptance Criteria

### MVP Functional Acceptance

- Desktop fresh install dapat menyelesaikan setup marketplace pertama dan menampilkan runtime status yang berarti.
- VPS fresh install dapat berjalan via binary+systemd maupun container.
- Install panel terkunci server-side setelah setup berhasil.
- Super Admin dapat membuat brand, seller, assignment, dan mengelola konfigurasi marketplace.
- Seller hanya dapat melihat dan mengelola brand yang assigned.
- Brand/category/product/variant/inventory basic berjalan.
- Storefront responsive dapat browse produk, cart, checkout, menerima order number/tracking, dan upload proof.
- Order lintas brand tersimpan dengan snapshot item dan `order_brand_groups`.
- Payment proof private, terproteksi backend, dan dapat verify/reject oleh role berwenang.
- Admin/seller dapat memproses order sesuai boundary RBAC.
- Package awareness tampil di settings/admin dan backend gate menolak capability yang tidak aktif.

### UI/UX Acceptance

- Storefront, admin, install, dan desktop control panel mengikuti hierarchy dan token di `DESIGN.md`.
- Semua loading/error/empty/degraded/auth-required states ada dan jelas.
- Visual density sesuai surface purpose.
- Responsive behavior stabil di desktop/tablet/mobile yang relevan.
- Semua CTA utama tetap terlihat dan reachable pada mobile untuk install/checkout.
- Tidak ada decorative imagery yang merusak clarity operator-trust.

### Image/Design Acceptance

- Setiap section visual memiliki keputusan asset yang terdokumentasi.
- Generated illustration/hero assets, bila dipakai, dicatat dengan prompt/legal note dan target path.
- Product catalog production tetap memakai uploaded product assets, bukan generated replacements.

### Release Acceptance

- Build/check/test utama hijau.
- Security negative tests penting lulus.
- `/health`, `/ready`, `/version` tervalidasi di runtime yang dideklarasikan.
- Backup/restore dry-run tervalidasi minimal sekali per lane penting.
- Bukti visual (current/final comparison) tersedia untuk surface UI utama.
- Tidak ada secret/PII sensitif bocor di UI/log/diagnostics/artifacts.

## Existing Patterns/Reuse

### Reuse yang Wajib Diprioritaskan

- `DESIGN.md` sebagai authority utama visual.
- `apps/web/src/lib/styles/app.css` untuk token semantic, `.panel`, focus ring, reduced motion.
- `AdminShell.svelte` untuk shell admin/operator.
- `StoreShell.svelte` untuk storefront shell.
- `StateNotice.svelte` dan `StatusBadge.svelte` untuk state messaging.
- Route grouping `(storefront)`, `(admin)`, `(install)` yang sudah ada.
- `core-api` route topology existing daripada mendesain ulang API surface.
- `core-db`, `core-auth`, `core-app`, `core-runtime` sebagai fondasi domain/runtime.

### Reuse > Extend > Create

- Reuse primitive dan shell yang sudah ada terlebih dahulu.
- Extend komponen yang ada untuk order/admin/install complexity berikutnya.
- Buat primitive baru hanya jika reuse tidak cukup.

### Yang Tidak Perlu Dibangun Ulang

- Design token dasar.
- Install panel visual language.
- Package awareness boundary conceptual model.
- Protected media/private proof boundary yang sudah terdokumentasi kuat.

## Constraints

- Stack tetap: Tauri v2, Rust Axum, PostgreSQL, SvelteKit, Tailwind CSS + TweakCN, local storage, `cloudflared`.
- User memilih scope: **MVP base only**.
- User memilih UI priority: **all core surfaces**.
- User memilih output: **implementation-ready plan**.
- Planner ini tidak boleh menjalankan agent implementation/generation.
- Browser evidence penting untuk UI, tetapi environment saat ini belum membuktikan runtime browser siap; existing repo evidence menunjukkan blocker Chromium/Chrome.
- Base open source harus tetap bernilai tanpa paid package.

## Risks

| Risk | Dampak | Mitigasi |
| --- | --- | --- |
| Runtime desktop supervision terlambat matang | MVP desktop terasa palsu/read-only | Jadikan runtime/process lane sebagai epic inti, bukan polish akhir. |
| Seller cross-brand leakage | Kebocoran data brand/order/proof | Enforce RBAC di backend query/service + negative tests wajib. |
| Payment proof leak | Kebocoran data finansial/PII | Private media, protected route, explicit permission, audit log. |
| Install panel tidak terkunci benar | Account takeover / re-bootstrap | Server-side install lock, race-safe setup, abuse protection. |
| UI selesai visual lebih dulu daripada commerce core | Rework besar | Urutkan implementation: foundation → commerce → proof/order ops → production polish. |
| Browser evidence lane terblokir | Tidak bisa claim visual completion | Selesaikan browser runtime lebih awal di designer/browser lane. |
| Asset generation liar/tidak legal | Risiko legal/brand mismatch | Semua generated asset harus style-equivalent, section-aware, dan dicatat legal note. |
| VPS/container/systemd divergence | Release gap dan drift | Validasi kedua lane secara eksplisit dalam release gate. |

## Decisions/Assumptions

### Locked Decisions

- MVP yang direncanakan adalah **base open-source only**.
- Semua core surfaces mendapat planning UI mendalam.
- Output planning bersifat implementation-ready, bukan execution-generated assets hari ini.
- Payment proof tetap private dan seller proof access deny-by-default kecuali permission eksplisit.
- Tunnel default off.
- Product catalog production memakai uploaded assets operator; generated assets hanya untuk illustration/support/demo placeholders bila perlu.

### Assumptions

- Desktop RC dapat dipisahkan dari public-signing readiness penuh.
- Category tetap brand-scoped untuk MVP kecuali docs diubah.
- Harga utama tetap berada di variant-backed model agar snapshot order konsisten.
- Browser capture workflow akan dieksekusi kemudian setelah runtime browser siap.

### Open Questions

- Format final desktop public release per OS masih perlu diputuskan saat release lane.
- Apakah seller tertentu akan diberi permission melihat file proof atau hanya status proof pada MVP awal; default plan: status-only untuk seller, file view eksplisit permission.

## Production Blueprint Summary

### MVP Slice

MVP adalah marketplace self-hosted tunggal yang installable di desktop/VPS, dengan operator dapat membuat marketplace, brand, seller, produk, menerima order lintas brand, memverifikasi bukti transfer, dan mengelola sistem dasar tanpa paid package.

### Epics / User Flows

1. Install/runtime foundation.
2. Auth/session/CSRF/RBAC.
3. Brand/seller/catalog foundation.
4. Storefront browsing + cart + checkout.
5. Order lifecycle + payment proof + fulfillment.
6. Desktop control panel + system status + tunnel.
7. Package awareness foundation.
8. UI/UX production polish + image generation lane.
9. Release, security, backup/restore, and deployment gates.

### Data/API Outline

- Data model mengikuti `ERD.md`: `users`, `sessions`, `brands`, `brand_members`, `products`, `product_variants`, `file_objects`, `orders`, `order_brand_groups`, `payment_proofs`, `packages`, `audit_events`.
- API mengikuti route skeleton saat ini: `/api/install`, `/api/auth`, `/api/storefront`, `/api/admin`, `/api/system`, `/api/packages`, `/media` protected route.

### SaaS/RBAC Considerations

- Bukan multi-tenant SaaS, tetapi isolation antar-brand tetap kritikal.
- Seller scoping, proof access, dan admin-only operations wajib backend-enforced.

### UI/Design Readiness

- `DESIGN.md` adalah authority visual pertama.
- Existing shells/tokens sudah cukup untuk baseline; plan fokus pada extension dan completeness.
- Semua UI completion claim membutuhkan capture evidence yang adil dan konsisten.

### AI Boundaries

- Tidak ada AI feature dalam MVP product.
- AI/MCP di sini dipakai untuk design generation, asset generation, browser validation, research, dan review workflow.

### Security/Privacy Checklist

- session secure,
- CSRF pada mutation,
- CORS allowlist explicit,
- install lock,
- proof private,
- audit sensitive events,
- diagnostics redaction,
- no wildcard credentialed CORS,
- no public PostgreSQL/debug exposure.

### Release/Ops Checklist

- systemd lane,
- container lane,
- desktop lane,
- health/ready/version,
- migration lock,
- backup/restore dry-run,
- checksums/manifest/release notes,
- smoke tests per environment.

## Visual Spec Summary

### Surface Matrix

| Surface | Purpose | Visual Character | Priority |
| --- | --- | --- | --- |
| Storefront home/catalog | Browse & trust building | content-first, product-forward, image-supported, mobile-friendly | High |
| Product detail | Conversion | media + variant clarity + stock confidence | High |
| Cart | Review before checkout | compact, clear totals, low distraction | High |
| Checkout | Complete order | guided, low-friction, high instruction clarity | High |
| Order tracking/success | Post-order confidence | status-first, explicit next step, payment guidance | High |
| Admin dashboard | Operational overview | calm control center, status-forward | High |
| Admin brands/products/orders/system/packages | CRUD + operations | panel-based, compact, explicit state | High |
| Install panel | Provisioning | trustable wizard, deterministic states | High |
| Desktop control panel | Local runtime ops | runtime-monitoring centric, operator-first | High |

### Visual Density Rubric

- Storefront: medium density.
- Checkout/order success: medium clarity, sticky summary desktop.
- Admin/system: compact but breathable.
- Install/desktop control: procedural, instruction-rich, high state visibility.

### Motion Summary

- Entrance motion subtle only.
- No large-shadow hover theatrics.
- Respect `prefers-reduced-motion` everywhere.
- Loading transitions should clarify state, not entertain.

### Icon Summary

- Gunakan licensed/open icon library untuk navigasi, status, commerce, dan runtime icons.
- Hindari icon dekoratif yang menjadi satu-satunya carrier makna status.

## Asset Manifest Summary

| Section | Decision | Reason |
| --- | --- | --- |
| Storefront hero / empty-state illustration | `generate` | Membutuhkan visual trust/support tanpa bergantung asset user. |
| Product thumbnails/gallery | `use-provided-assets` | Harus merepresentasikan produk nyata operator. |
| Brand logos | `use-provided-assets` | Identitas brand milik operator. |
| Admin empty states | `generate` bila perlu | Untuk clarity/support tanpa marketing-heavy feel. |
| Runtime/system icons | `licensed-existing-assets` | Lebih tepat memakai icon library yang konsisten. |
| Install guidance illustration | `generate` optional | Hanya jika membantu onboarding visual. |
| Desktop control panel visuals | `licensed-existing-assets` / `no-generation-needed` | Fokus utama operational clarity, bukan imagery. |

## Image Generation Decision

### Storefront

- Hero/support illustration: `generate`
- Product imagery: `use-provided-assets`
- Empty-state/support cards: `generate`

### Checkout / Order Tracking

- Success/empty/error illustration: `generate`
- Core product/order visuals: `use-provided-assets` atau `no-generation-needed` jika text-first lebih jelas

### Admin

- Primary operational views: `no-generation-needed`
- Empty-state illustrations: `generate` bila memperjelas guidance
- Icons: `licensed-existing-assets`

### Install Panel

- Setup helper illustration: `generate` optional
- Operational status visuals: `no-generation-needed`

### Desktop Control Panel

- Runtime/service visuals: `licensed-existing-assets`
- Decorative imagery: `no-generation-needed`

## Design Readiness Gate

Implementasi UI/image generation **tidak boleh dimulai** sampai item berikut terpenuhi:

1. Visual spec matrix lengkap untuk semua core surfaces.
2. Motion storyboard ringkas tersedia.
3. Icon matrix tersedia.
4. Visual density rubric terdokumentasi.
5. Asset manifest summary disepakati.
6. Image generation decision per section tersedia.
7. `reference-captures.md` dan `current-captures.md` siap sebagai baseline artifact.
8. Browser capture blocker diselesaikan atau limitation dicatat jelas.
9. Final comparison dan designer review artifacts sudah menjadi bagian dari execution checklist.

Jika salah satu item di atas belum siap, lane implementasi visual dianggap **blocked**.

## TDD/Test Plan

### Apakah TDD Wajib?

Ya, untuk production logic dan interaction-critical UI. Alasan: proyek ini menyentuh install lock, auth, RBAC, payment proof privacy, checkout correctness, order grouping, dan runtime readiness; semua area ini berisiko tinggi dan tidak boleh hanya diverifikasi manual.

### Existing Test Patterns

- Reuse tests yang sudah ada di Rust crates untuk router/auth/csrf/packages/system readiness/migrations sebagai baseline.
- Reuse `npm run check` dan `npm run build` lane yang sudah pernah pass pada web foundation.

### First Failing / Regression Tests

1. Install setup hanya sukses sekali; second setup request ditolak server-side.
2. Seller tidak bisa mengakses data/order/proof brand lain.
3. Checkout empty cart ditolak dan quantity melebihi stock ditolak.
4. Cross-brand order menghasilkan `order_brand_groups` yang benar.
5. Payment proof tidak bisa diakses via static/public path dan unauthorized media route.
6. UI baseline visual regression untuk `/install`, `/store`, `/admin`, `/store/checkout` setelah browser lane siap.

### Green Step

- Implement domain/service/route minimum agar test di atas lulus satu per satu sesuai epic order.
- Pastikan UI tests/evidence menunjukkan state yang benar pada loading/error/empty/success/auth cases.

### Refactor Step

- Rapikan duplication di service/repository/UI shells.
- Konsolidasikan shared form/status components.
- Jaga route/service boundaries tetap sesuai TRD.

### Edge Cases

- Concurrent setup attempt.
- Tunnel off tetapi marketplace local tetap usable.
- Payment proof upload invalid MIME/size/extension.
- Order dengan banyak brand dan seller assignment overlap.
- Session revoked / expired saat mutation.
- Build/deploy pada container vs systemd dengan env berbeda.

### Commands

- Web:
  - `npm run check`
  - `npm run build`
- Rust/native bila host siap:
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --workspace`
- Rust fallback via Docker bila host belum siap:
  - `docker run --rm -v "$PWD":/workspace -w /workspace rust:1.88 cargo test --workspace`
- Browser/visual evidence lane:
  - gunakan Playwright MCP workflow, bukan one-shot screenshot CLI

## Implementation Steps

### Phase 0 — Stabilize Foundation & Truth Boundaries

1. Finalisasi runtime/app config, state model, migration lock, health/readiness semantics.
2. Rapikan install state source of truth dan server-side lock contract.
3. Finalisasi auth/session/CSRF/CORS contract antara SvelteKit ↔ Axum.
4. Konfirmasi route ownership dan DTO contract inti.

### Phase 1 — Complete Install & Runtime Flows

1. Selesaikan VPS install panel end-to-end.
2. Selesaikan desktop setup/control-panel runtime lifecycle.
3. Tambahkan preflight yang benar-benar bermakna.
4. Tambahkan runtime/system status yang actionable dan sanitized.

### Phase 2 — Identity, RBAC, and Brand Isolation

1. Super Admin auth flows.
2. Seller auth + brand assignment.
3. Permission matrix untuk sensitive operations termasuk proof access.
4. Audit events untuk auth/assignment/permission changes.

### Phase 3 — Catalog & Admin CRUD

1. Brand CRUD.
2. Seller create/assign UI.
3. Category/product/variant/inventory CRUD.
4. Product image upload via local storage.
5. Admin/order list foundation.

### Phase 4 — Storefront & Commerce Core

1. Product listing/detail/brand browse/search basics.
2. Persistent cart API/UI.
3. Guest checkout form and validation.
4. Cross-brand order creation with snapshots.
5. Order tracking flow.

### Phase 5 — Payment Proof & Fulfillment Ops

1. Payment proof upload flow.
2. Protected media route enforcement.
3. Verify/reject proof workflow.
4. Seller/admin order view separation.
5. Per-brand fulfillment updates and status history.

### Phase 6 — UI/UX Production Polish Lane

1. Jalankan designer lane untuk visual spec final dan responsive polish.
2. Jalankan browser evidence baseline capture.
3. Implement missing empty/loading/error/auth states.
4. Rapikan interaction hierarchy storefront/admin/install/desktop.
5. Jalankan accessibility + motion review.

### Phase 7 — Design/Image Generation Lane

1. Jalankan `@designer` untuk final design direction per surface berdasar `DESIGN.md`.
2. Gunakan browser/reference workflow untuk capture baseline dan parity checks.
3. Jalankan `@visual-asset-generator` hanya untuk section yang decision-nya `generate`.
4. Simpan metadata asset generation di `generated-assets.md`.
5. Integrasikan asset legal style-equivalent ke UI implementation.

### Phase 8 — Ops/Release Hardening

1. systemd smoke lane.
2. container smoke lane.
3. desktop RC smoke lane.
4. backup/restore dry-run.
5. release manifest/checksum/notes.
6. quality gate final.

## Expected Files to Change

### Backend / Domain

- `apps/backend/src/**/*`
- `crates/core-api/src/**/*`
- `crates/core-app/src/**/*`
- `crates/core-auth/src/**/*`
- `crates/core-db/src/**/*`
- `crates/core-domain/src/**/*`
- `crates/core-installer/src/**/*`
- `crates/core-runtime/src/**/*`
- `crates/core-storage/src/**/*`
- `crates/core-packages/src/**/*`
- `crates/sidecar-postgres/src/**/*`
- `crates/sidecar-cloudflared/src/**/*`
- `migrations/core/*.sql`
- `migrations/packages/*.sql`

### Web UI

- `apps/web/src/routes/**/*`
- `apps/web/src/lib/ui/**/*`
- `apps/web/src/lib/api/**/*`
- `apps/web/src/lib/stores/**/*`
- `apps/web/src/lib/styles/**/*`

### Desktop UI / Shell

- `apps/desktop/src-tauri/src/**/*`
- `apps/desktop/src/**/*`
- `apps/desktop/index.html`
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`

### Docs / Release / Ops

- `docs/deployment/**/*`
- `docs/security/**/*`
- `docs/release/**/*`
- `.github/workflows/**/*`

### Generated Assets (saat execution lane)

- target paths di app root, misalnya `apps/web/static/...` atau lokasi asset final yang diputuskan implementasi

## Agent/Tool Routing

### Primary Routing

- `@orchestrator`: koordinasi urutan lane dan integrasi hasil.
- `@artifact-planner`: source-of-truth plan ini.
- `@fixer`: implementasi bounded per phase/epic dengan TDD.
- `@quality-gate`: signoff akhir untuk material changes.

### Research / Review Lanes

- `@explorer`: discovery reuse/files/tests.
- `@librarian`: official docs/version-sensitive behavior (SvelteKit, Axum, Tauri, sqlx, Playwright, Tailwind/TweakCN).
- `@oracle`: review architecture dan simplification checks.
- `@security-privacy-reviewer`: auth/RBAC/protected media/tunnel/privacy gate.
- `@release-engineer`: systemd/container/desktop release readiness.

### UI/UX & Visual Lanes

- `@designer`: visual direction, responsive/UI polish, reference replication decisions.
- `@visual-parity-auditor`: screenshot comparison claims.
- `@accessibility-reviewer`: semantics/focus/contrast/motion checks.
- `@motion-specialist`: animation/reduced-motion review bila diperlukan.
- `@ui-system-architect`: bila perlu menormalkan token/component anatomy tambahan.

### Image / Browser / MCP Lanes

- `agent-browser` / Playwright MCP: baseline, current, final captures; real user-flow validation.
- `@visual-asset-generator`: legal style-equivalent image generation untuk section yang diset `generate`.
- `image-asset-generator` tool: output asset files saat execution implementation/design lane sudah siap.

## Validation Commands

### Local / CI Core

- `npm run check` di `apps/web`
- `npm run build` di `apps/web`
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`

### Docker Fallback / Release Validation

- Rust workspace validation via Docker jika host belum siap.
- container smoke validation via `docker compose up` lane dengan health checks.

### Browser / Visual Validation

- Gunakan Playwright/browser MCP dengan viewport tetap:
  - `1440x1200`
  - `768x1024`
  - `390x844`
- Terapkan wait-stabilize-scroll-settle workflow untuk reference/current/final capture.

### Security / Ops Validation

- negative tests untuk RBAC/proof access/install lock/CSRF/CORS.
- backup/restore dry-run commands sesuai docs deployment/release lane.

## Evidence Requirements

### Required During Implementation

- `.opencode/evidence/20260511-2156-project-completion-master-plan/discovery.md`
- `.opencode/evidence/20260511-2156-project-completion-master-plan/reference-captures.md`
- `.opencode/evidence/20260511-2156-project-completion-master-plan/current-captures.md`
- `.opencode/evidence/20260511-2156-project-completion-master-plan/generated-assets.md`
- `.opencode/evidence/20260511-2156-project-completion-master-plan/visual-comparison.md`
- `.opencode/evidence/20260511-2156-project-completion-master-plan/final-designer-review.md`

### Required Notes for Browser Captures

- viewport
- wait strategy
- scroll pass detail
- screenshot paths
- rendering-affecting console errors
- rendering-affecting network errors
- known limitations

### Required Notes for Image Generation

- prompt ringkas
- legal note
- target path
- size
- alasan section-aware
- final usage status

### Why Some Evidence Is Still Pending

- Current environment belum membuktikan browser runtime siap; repo evidence sebelumnya menunjukkan blocker Chromium/Chrome. Karena itu artifact reference/current/final visual di plan ini masih berupa requirement placeholders dan menjadi blocker execution visual jika belum diselesaikan.

## Reference Capture Requirements

- Pakai workflow wait-stabilize-scroll-settle yang sama untuk current dan final.
- Capture minimal untuk storefront, checkout, admin utama, install, dan desktop control panel.
- Jangan memakai single immediate screenshot untuk halaman yang punya lazy content atau entrance animation.

## Visual Comparison Requirements

- Bandingkan current vs final per surface utama.
- Nilai hierarchy, clarity, responsive stability, token consistency, image quality, icon consistency, accessibility-visible states, dan motion restraint.
- Jangan claim visual completion tanpa comparison evidence dan designer review.

## Done Criteria

Plan ini dianggap selesai bila:

1. Seluruh section source-of-truth plan lengkap dan actionable.
2. Scope user tercermin jelas: MVP base only + all core surfaces + implementation-ready planning.
3. Reuse candidates dan existing repo patterns tercatat.
4. UI/UX lane dan image generation lane sudah terdokumentasi lengkap.
5. Design Readiness Gate tertulis jelas.
6. TDD/test/release/security evidence gates tertulis jelas.
7. Artifact plan dan evidence minimal tersimpan di `.opencode/`.

Project implementation nanti baru boleh disebut selesai jika:

1. Functional MVP acceptance tercapai.
2. UI/UX acceptance tercapai dengan screenshot evidence.
3. Image generation decisions yang relevan sudah dieksekusi dan dicatat.
4. Security, release, backup/restore, dan deployment gates lulus.
5. `@quality-gate` memberikan signoff final.

## Final Planning Summary

### Artifacts Created

- Primary plan:
  - `.opencode/plans/20260511-2156-project-completion-master-plan.md`
- Kept draft artifacts:
  - `.opencode/draft/20260511-2156-project-completion-master-plan/visual-notes.md`
  - `.opencode/draft/20260511-2156-project-completion-master-plan/asset-manifest.md`
- Kept evidence artifacts:
  - `.opencode/evidence/20260511-2156-project-completion-master-plan/discovery.md`
  - `.opencode/evidence/20260511-2156-project-completion-master-plan/reference-captures.md`
  - `.opencode/evidence/20260511-2156-project-completion-master-plan/current-captures.md`
  - `.opencode/evidence/20260511-2156-project-completion-master-plan/generated-assets.md`
  - `.opencode/evidence/20260511-2156-project-completion-master-plan/visual-comparison.md`
  - `.opencode/evidence/20260511-2156-project-completion-master-plan/final-designer-review.md`

### Key Decisions

- Plan difokuskan ke **base MVP**, bukan paid modules.
- Semua core surfaces masuk planning UI mendalam.
- Design/image generation dimasukkan sebagai execution lane formal memakai MCP/agent, bukan dilakukan diam-diam di fase planner.
- Reuse-first terhadap design system dan UI shells existing.

### Assumptions

- Browser evidence lane akan dapat dijalankan setelah runtime browser disiapkan.
- Desktop public-signing penuh dapat dipisahkan dari MVP implementation-complete internal RC.

### Open Questions

- Final desktop public artifact format/signing per OS.
- Seller proof file-view policy final; default plan: explicit permission only.

### Readiness for Implementation

- Siap untuk dipecah menjadi beberapa execution lane oleh `@orchestrator`.
- Urutan yang direkomendasikan: foundation → install/runtime → auth/RBAC → catalog → storefront/checkout → proof/order ops → UI/UX polish → image generation → release gates.

### Cleanup Performed

- Draft/evidence tidak dibersihkan karena masih operasionally useful untuk lane UI/UX, browser capture, dan image generation berikutnya.
- Tidak ada source file di luar `.opencode/` yang diubah.
