# Discovery Evidence — Project Completion Master Plan

Task ID: `20260511-2156-project-completion-master-plan`
Tanggal: 2026-05-11 21:56 ICT

## Files Inspected

- `README.md`
- `PRD.md`
- `TRD.md`
- `ERD.md`
- `DESIGN.md`
- `AGENTS.md`
- `.opencode/docs/AGENT_ROUTING.md`
- `.opencode/docs/SKILLS.md`
- `.opencode/plans/20260507-2054-production-ready-roadmap.md`
- `apps/web/src/lib/styles/app.css`
- `apps/web/src/lib/ui/AdminShell.svelte`
- `apps/web/src/lib/ui/StoreShell.svelte`
- `apps/web/src/routes/(install)/install/+page.svelte`
- `apps/desktop/src-tauri/src/main.rs`
- `.opencode/evidence/gate-h-ui-foundation.md`

## Project Patterns Found

### Arsitektur & Reuse

- Monorepo sudah mengikuti boundary yang dideskripsikan di `TRD.md`: `apps/backend`, `apps/web`, `apps/desktop`, dan `crates/core-*`.
- `core-api` sudah memiliki route topology inti: `auth`, `install`, `storefront`, `admin`, `system`, `packages`, `media`.
- `core-db` sudah punya fondasi untuk install state, auth/session, commerce, media, packages, dan migration locking.
- `core-auth` sudah menyiapkan primitive penting: password hashing, session hashing, CSRF, permission helper.
- `core-app` sudah memegang application state dan readiness orchestration.

### UI System & Reuse

- Design authority utama sudah ada di `DESIGN.md`.
- Token dan visual primitive sudah tersedia di `apps/web/src/lib/styles/app.css`:
  - semantic colors,
  - `.panel`,
  - `.status-chip`,
  - `:focus-visible`,
  - reduced-motion baseline.
- Reusable UI shells sudah ada:
  - `AdminShell.svelte`
  - `StoreShell.svelte`
  - `StateNotice.svelte`
  - `StatusBadge.svelte`
- Install surface sudah punya status-forward pattern yang sesuai `DESIGN.md`.

### Existing Validation/Evidence

- `.opencode/evidence/gate-h-ui-foundation.md` menunjukkan:
  - `npm run check` dan `npm run build` untuk web pernah pass,
  - screenshot evidence sebelumnya terblokir karena runtime browser/Chromium belum tersedia.
- Existing release/planning artifacts sudah ada di `.opencode/plans/20260507-2054-production-ready-roadmap.md` dan `.opencode/plans/20260508-0548-release-gate-plan.md`.

## Reuse Candidates

- Reuse route grouping SvelteKit: `(storefront)`, `(admin)`, `(install)`.
- Reuse `AdminShell` untuk semua admin/operator workspace tambahan.
- Reuse `StoreShell` untuk storefront shell consistency.
- Reuse `StateNotice` dan `StatusBadge` untuk semua state/error/permission/install/runtime messages.
- Reuse semantic tokens dari `app.css` dan Tailwind aliases; jangan menambah token ad-hoc tanpa alasan kuat.
- Reuse install panel structure sebagai baseline untuk wizard surfaces lain.
- Reuse `core-db` + `core-api` skeleton daripada membangun flow baru dari nol.

## Commands / Docs / Sources Checked

- Repo-local docs: `README.md`, `PRD.md`, `TRD.md`, `ERD.md`, `DESIGN.md`, `AGENTS.md`
- Repo-local routing/ownership docs: `.opencode/docs/AGENT_ROUTING.md`, `.opencode/docs/SKILLS.md`
- Existing evidence: `.opencode/evidence/gate-h-ui-foundation.md`
- Specialist subagent outputs used:
  - `@explorer` for current scaffold maturity and reuse map
  - `@product-architect` for MVP blueprint and epic slicing
  - `@security-privacy-reviewer` for security/privacy release boundaries
  - `@release-engineer` for operational/release gates

## Constraints

- Planner hanya boleh menulis artifact di `.opencode/`.
- User memilih scope: **MVP base only**.
- User memilih UI priority: **all core surfaces**.
- User memilih output: **implementation-ready plan**.
- Browser screenshot evidence relevan untuk UI, tetapi environment sebelumnya menunjukkan blocker Chromium/Chrome runtime.
- Planner tidak boleh memanggil agent implementation/generation; design/image generation harus diroute dalam plan ke agent yang sesuai.

## Risks Observed

- Desktop runtime supervision masih sangat awal; `apps/desktop/src-tauri/src/main.rs` masih read-only status stub.
- UI shell lebih maju dibanding domain flows end-to-end; banyak surface masih placeholder/fallback.
- Bukti visual parity/reference capture belum tersedia untuk semua core surfaces.
- Karena user meminta UI/UX lengkap dengan generated images/design, tanpa design execution lane yang disiplin proyek berisiko over-design sebelum commerce core benar-benar siap.
- Browser evidence environment saat ini belum terbukti siap, sehingga plan harus menuliskan blocker dan fallback explicitly.

## Planning Impact

- Plan harus **reuse-first**, bukan membangun design system baru.
- Plan harus memisahkan:
  1. MVP commerce foundation,
  2. UI/UX production polish,
  3. image generation and designer evidence lane.
- Implementasi UI/image generation harus diblok oleh **Design Readiness Gate** dan screenshot evidence workflow yang konsisten.
