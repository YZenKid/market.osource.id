# Rencana Penyelesaian Lanjutan MVP Dasar — `market.osource.id`

Task ID: `20260512-0746-unfinished-mvp-completion-plan`

## Goal

Menyelesaikan seluruh pekerjaan lanjutan yang masih tersisa agar **fitur dasar MVP `market.osource.id` benar-benar siap digunakan** pada jalur operasional minimum: desktop local-hosted, VPS binary+systemd, dan container/Compose, dengan flow install, runtime, commerce, backup/restore, dan release evidence yang cukup untuk dipakai operator nyata.

## Non-goals

- Tidak menyelesaikan fitur paid module.
- Tidak menambahkan AI/product features baru.
- Tidak mengubah scope menjadi SaaS multi-tenant.
- Tidak mengklaim visual completion/final parity tanpa browser evidence.
- Tidak menjadikan desktop public-signing/notarization lintas OS sebagai syarat blocking MVP dasar internal/operator.

## Scope

### In Scope

- Menutup gap implementasi dasar yang masih tersisa setelah master plan dan execution sebelumnya.
- Desktop runtime supervision minimal yang benar-benar operasional.
- Install/runtime end-to-end verification.
- systemd/container smoke lane yang executable dan tervalidasi.
- Backup/restore rehearsal staging yang nyata.
- Release artifact lane minimum: manifest, checksum, release note, smoke evidence.
- Browser evidence untuk usability dasar core surfaces bila runtime browser tersedia.
- Hardening validation dan negative tests tambahan bila dibutuhkan untuk menutup gap MVP dasar.

### Out of Scope

- Theming lanjutan, animation polish lanjutan, dan visual parity premium.
- Public marketplace scale/performance tuning tingkat lanjut.
- Signed installer public distribution per OS.
- Cloud backup scheduler otomatis penuh.

## Requirements

### Functional Requirements

- Operator dapat menjalankan marketplace desktop dengan runtime yang benar-benar bisa start/stop/status.
- Operator VPS dapat menjalankan backend+web via systemd maupun container dengan smoke verification yang jelas.
- Install flow pertama terkunci setelah setup sukses dan tidak bisa dibuka ulang secara ilegal.
- Flow commerce inti tetap berfungsi setelah hardening/release work:
  - auth,
  - brand/seller,
  - catalog,
  - cart/checkout,
  - tracking,
  - payment proof,
  - fulfillment.
- Backup database + storage dapat diuji minimal pada rehearsal staging/isolated target.

### Technical Requirements

- Backend tetap authority untuk readiness, install lock, auth/RBAC, proof protection, dan order state.
- Desktop shell tidak boleh memulai proses secara liar; lifecycle harus eksplisit, observable, dan terbatas.
- Release lane harus membedakan liveness (`/health`) dan readiness (`/ready`).
- Restore rehearsal tidak boleh menulis ke target berisiko tinggi/production path aktif.
- Docs dan helper scripts harus konsisten dengan perilaku sistem aktual.

### UI/UX Requirements

- Core surfaces tetap usable dan tidak regress akibat work operasional.
- Admin/system/install/desktop control surfaces harus memberi status operator yang jelas.
- Visual completion tetap bergantung pada browser evidence, tetapi usability dasar tidak boleh rusak.

## Acceptance Criteria

### MVP Ready-to-Use Acceptance

- Desktop runtime dapat menampilkan status proses nyata dan minimal start/stop/retry flow untuk backend yang relevan.
- Fresh install dapat diverifikasi end-to-end setidaknya pada satu runtime lane aktif.
- systemd lane memiliki smoke procedure yang berjalan dan tervalidasi.
- container lane memiliki smoke procedure yang berjalan dan tervalidasi.
- Backup + restore rehearsal staging minimal sekali tervalidasi dengan hasil `/health`, `/ready`, `/version` tercatat.
- Release evidence minimum tersedia: artifact notes, checksum, runtime contract verification, backup/restore rehearsal notes.
- Tidak ada regresi pada flow commerce inti setelah seluruh slice ini selesai.

### Quality Acceptance

- `cargo test` relevant workspace hijau.
- `npm run check` dan `npm run build` di `apps/web` hijau.
- Negative test penting untuk install lock / RBAC / proof access / runtime contract tetap hijau.
- `@quality-gate` memberi signoff minimal `PASS_WITH_RISKS`, dan target akhir plan ini adalah `PASS` untuk MVP dasar internal/operator.

## Existing Patterns/Reuse

### Reuse Wajib

- `.opencode/plans/20260511-2156-project-completion-master-plan.md` sebagai baseline besar.
- `.opencode/evidence/20260511-2156-project-completion-master-plan/gate-i-j-ops-release-foundation.md` untuk runtime/release contract yang sudah selesai.
- `crates/core-api/src/routes/health.rs` untuk probe contract.
- `crates/core-api/src/routes/system.rs` untuk runtime summary dan operator caution.
- `apps/desktop/src-tauri/src/main.rs` untuk desktop status baseline.
- `scripts/backup/*` dan `scripts/release/*` yang sudah dibangun.
- `docs/deployment/*` dan `docs/release/*` yang sudah ada, extend bukan rewrite.

### Reuse > Extend > Create

- Extend release/runtime contract yang sudah hijau sebelum menambah control plane baru.
- Reuse shell/status UI yang sudah ada untuk desktop/admin system.
- Tambah script/lane baru hanya bila helper saat ini belum cukup membuktikan acceptance.

## Constraints

- Browser evidence masih bergantung pada ketersediaan runtime Chromium/Chrome di environment execution.
- Desktop supervision harus dibatasi agar tidak menciptakan control plane setengah jadi yang berbahaya.
- MVP dasar harus tetap operasional tanpa paid package dan tanpa tunnel default-on.
- Release lane harus aman terhadap secret dan path sensitif.

## Risks

| Risk | Dampak | Mitigasi |
| --- | --- | --- |
| Desktop supervision terlalu ambisius | Runtime desktop jadi rapuh | Batasi ke lifecycle minimum backend/sidecar yang paling penting, tambah observable state + retry rules sederhana. |
| Smoke lane systemd/container hanya “docs” | Deploy nyata tetap tidak terbukti | Wajib ada command/evidence executable, bukan prose saja. |
| Restore rehearsal tidak realistis | Recovery palsu | Wajib restore ke target staging/isolated lalu cek `/health` `/ready` `/version`. |
| Browser blocker berlanjut | Claim usable UI melemah | Pisahkan visual completion dari MVP operasional, tetapi tetap catat blocker dan workaround. |
| Regression commerce flow | MVP rusak saat release hardening | Selalu rerun core Rust + web validation setelah tiap slice. |

## Decisions/Assumptions

### Locked Decisions

- Prioritas berikutnya adalah **MVP operasional**, bukan visual parity.
- Browser evidence tetap penting, tetapi bukan blocker untuk seluruh lane operasional non-visual.
- Desktop public distribution final per OS bukan syarat penyelesaian MVP dasar internal/operator.
- Desktop runtime supervision pada milestone ini **mencakup PostgreSQL sidecar control juga**, bukan backend-only minimum.
- Prioritas execution berjalan **langsung paralel untuk desktop dan VPS lane**, bukan memilih salah satu lebih dahulu.

### Assumptions

- Marketplace dapat dianggap “siap digunakan” secara MVP bila lane desktop atau VPS dapat dioperasikan jelas oleh operator, walau visual browser evidence final belum lengkap.
- systemd dan container dapat dibuktikan dengan smoke lane yang cukup, tidak harus full production infra automation.

### Open Questions

- Tidak ada open question material yang tersisa untuk memulai Phase A–E.

## Production Blueprint Summary

### MVP Slice yang Belum Selesai

1. Desktop supervision nyata.
2. Install/runtime end-to-end proof.
3. Deployment smoke proof (systemd + container).
4. Backup/restore rehearsal nyata.
5. Release artifact closeout.
6. Browser evidence/usability proof bila runtime memungkinkan.

### Data/API/Runtime Outline

- Tidak ada perubahan besar domain model baru.
- Fokus pada runtime orchestration, verification, dan operator evidence.
- API contract yang paling relevan:
  - `/health`
  - `/ready`
  - `/version`
  - `/api/system/runtime`
  - existing install/auth/admin/storefront/media routes.

### Security/Privacy Checklist

- control plane desktop harus local/operator scoped,
- install lock tetap server-side,
- proof access tetap protected,
- backup artifacts tidak membocorkan secret,
- release evidence tidak boleh menyertakan database URL/raw path sensitif.

### Release/Ops Checklist

- desktop runtime lifecycle minimal,
- systemd smoke,
- container smoke,
- restore rehearsal,
- checksum/manifest/release notes,
- final runtime contract evidence,
- quality gate final.

## TDD/Test Plan

### TDD Requirement

TDD **wajib** untuk logic baru pada runtime lifecycle, restore/preflight safety, deployment verification scripts yang punya branching penting, dan API/runtime contract yang berubah.

### Existing Test Patterns to Reuse

- Rust route tests di `crates/core-api/src/routes/*`.
- Rust state tests di `crates/core-app/src/state.rs`.
- Existing command/script dry-run validation pattern dari evidence sebelumnya.

### First Failing / Regression Tests to Write

1. Desktop supervision state transition tests untuk command/status baru.
2. Restore preflight negative tests untuk target berisiko tinggi / checksum mismatch.
3. Runtime probe tests untuk payload fields yang jadi contract release.
4. Smoke lane command verification tests atau scripted assertions bila feasible.

### Green Step

- Implement command/runtime lifecycle minimum.
- Implement restore rehearsal support sampai bukti live/staging tersedia.
- Implement smoke helpers untuk systemd/container.

### Refactor Step

- Rapikan DTO/status strings agar konsisten antar desktop/system/runtime docs.
- Kurangi duplikasi wording probe/runtime metadata di desktop/backend/docs.

### Edge Cases

- backend start gagal,
- DB belum siap,
- install belum selesai,
- checksum mismatch,
- restore diarahkan ke target berbahaya,
- smoke lane berhasil liveness tapi gagal readiness.

### Commands

- `cargo test -p core-api -p core-app -p core-db`
- `cargo test --workspace`
- `npm run check` di `apps/web`
- `npm run build` di `apps/web`
- helper runtime verification + backup/restore rehearsal commands sesuai lane.

## Implementation Steps

### Phase A — Desktop Runtime Supervision Minimum

1. Audit `apps/desktop/src-tauri/src/main.rs` dan sidecar crates untuk capability start/stop/status yang sudah ada.
2. Tambahkan command desktop minimum:
   - read status nyata,
   - start backend/service lokal,
   - stop backend/service lokal,
   - retry/reload status.
3. Tambahkan lifecycle minimum untuk PostgreSQL sidecar juga:
   - start sidecar,
   - stop sidecar,
   - status sidecar,
   - urutan start/stop yang aman antara PostgreSQL sidecar dan backend.
4. Tambahkan operator-safe error surface dan caution notes.
5. Tambahkan tests untuk state transitions yang paling penting.

### Phase B — Install & Runtime End-to-End Proof

1. Verifikasi fresh runtime tanpa DB/install state: expected `not_ready` semantics.
2. Verifikasi setelah setup/install state valid: readiness menjadi `ready` pada lane staging/test yang sesuai.
3. Pastikan install lock proof tercatat dalam evidence.
4. Rapikan admin/system/install status wording jika masih membingungkan operator.

### Phase C — systemd / Container Smoke Lane

1. Tambahkan helper/commands praktis untuk systemd smoke verification.
2. Tambahkan helper/commands praktis untuk container/Compose smoke verification.
3. Jalankan `verify_runtime_contract.sh` pada kedua lane atau documented equivalent.
4. Simpan evidence output command dan expected caveats.
5. Jalankan lane ini paralel dengan desktop lane bila tidak ada dependency keras yang saling memblokir.

### Phase D — Backup/Restore Rehearsal Nyata

1. Siapkan target staging/isolated rehearsal untuk DB + storage restore.
2. Jalankan backup artifacts dengan metadata dan checksum.
3. Jalankan restore preflight.
4. Restore ke target staging.
5. Boot app terhadap target itu.
6. Catat `/health`, `/ready`, `/version`, dan residual risk.

### Phase E — Release Artifact Closeout

1. Generate/update checksum artifacts.
2. Generate SBOM bila tool tersedia; bila tidak, catat alasan.
3. Lengkapi release note template untuk known limitations + upgrade + rollback.
4. Tautkan evidence path ke manifest/release notes.

### Phase F — Browser Evidence & Usability Proof

1. Jika Chromium/Chrome tersedia, jalankan browser capture workflow untuk core surfaces.
2. Verifikasi loading/error/empty/auth-required states.
3. Catat blocker jika browser runtime tetap unavailable.
4. Jika blocker tetap ada, MVP operasional dapat ditandai siap dengan caveat visual evidence pending, bukan claim visual final.

## Expected Files to Change

### Desktop / Runtime

- `apps/desktop/src-tauri/src/**/*`
- `crates/sidecar-postgres/src/**/*`
- `crates/sidecar-cloudflared/src/**/*`
- `crates/core-runtime/src/**/*`

### Backend / App State

- `apps/backend/src/**/*`
- `crates/core-api/src/**/*`
- `crates/core-app/src/**/*`
- `crates/core-db/src/**/*`

### Web / Admin System Surface

- `apps/web/src/routes/(admin)/admin/system/**/*`
- `apps/web/src/routes/(install)/**/*`
- `apps/web/src/lib/ui/**/*`

### Ops / Docs / Release

- `scripts/backup/**/*`
- `scripts/release/**/*`
- `docs/deployment/**/*`
- `docs/release/**/*`
- `.github/workflows/**/*`

## Agent/Tool Routing

### Primary

- `@fixer`: implementasi bounded per phase.
- `@release-engineer`: smoke lane, release readiness, restore rehearsal strategy.
- `@quality-gate`: review final tiap slice material.

### Discovery / Review

- `@explorer`: cek reuse code/tests/scripts.
- `@oracle`: review desktop supervision boundary agar tidak overengineer.
- `@security-privacy-reviewer`: review backup/release/privacy exposure bila ada perubahan sensitif.

### Browser / UI

- `agent-browser`: capture & usability proof bila runtime browser siap.
- `@designer`: hanya untuk final usability/visual review jika browser lane sudah bisa jalan.

## Validation Commands

### Core

- `cargo test -p core-api -p core-app -p core-db`
- `cargo test --workspace`
- `npm run check` di `apps/web`
- `npm run build` di `apps/web`

### Runtime / Ops

- `BASE_URL=... ./scripts/release/verify_runtime_contract.sh`
- backup helper commands dengan `DRY_RUN=1` lalu rehearsal nyata
- systemd/container smoke commands sesuai docs lane

### Browser

- Playwright/browser MCP workflow jika runtime tersedia

## Evidence Requirements

### Required

- `.opencode/evidence/20260512-0746-unfinished-mvp-completion-plan/discovery.md`
- `.opencode/evidence/20260512-0746-unfinished-mvp-completion-plan/desktop-runtime.md`
- `.opencode/evidence/20260512-0746-unfinished-mvp-completion-plan/install-runtime-proof.md`
- `.opencode/evidence/20260512-0746-unfinished-mvp-completion-plan/systemd-smoke.md`
- `.opencode/evidence/20260512-0746-unfinished-mvp-completion-plan/container-smoke.md`
- `.opencode/evidence/20260512-0746-unfinished-mvp-completion-plan/backup-restore-rehearsal.md`
- `.opencode/evidence/20260512-0746-unfinished-mvp-completion-plan/release-closeout.md`
- `.opencode/evidence/20260512-0746-unfinished-mvp-completion-plan/browser-usability.md`

### Evidence Notes Minimum

- commands run,
- environment assumptions,
- pass/fail,
- blocker,
- artifact paths,
- residual risks.

## Done Criteria

Plan ini dianggap tuntas bila:

1. Semua gap MVP dasar yang belum selesai telah dipetakan konkret.
2. Tiap gap punya phase implementasi yang bounded.
3. Validation dan evidence lane jelas untuk deployment, restore, desktop, dan browser.
4. Source-of-truth plan tersimpan di `.opencode/plans/`.

Implementasi nanti baru boleh dianggap “MVP dasar siap digunakan” bila:

1. Desktop atau VPS primary lane dapat dijalankan operator nyata.
2. Release/deployment smoke terbukti.
3. Restore rehearsal staging terbukti.
4. Core commerce flow tetap hijau.
5. Browser usability proof selesai atau blocker-nya terdokumentasi jelas dan diterima.
6. `@quality-gate` final memberi signoff yang sesuai.

## Final Planning Summary

### Artifacts Created

- Primary plan:
  - `.opencode/plans/20260512-0746-unfinished-mvp-completion-plan.md`
- Kept evidence:
  - `.opencode/evidence/20260511-2156-project-completion-master-plan/gate-i-j-ops-release-foundation.md`
  - `.opencode/evidence/20260511-2156-project-completion-master-plan/current-captures.md`
  - `.opencode/evidence/20260512-0746-unfinished-mvp-completion-plan/discovery.md`

### Key Decisions

- Fokus baru adalah menyelesaikan **operasional MVP dasar**, bukan menambah scope produk.
- Prioritas pertama adalah desktop/runtime/deployment/restore proof.
- Browser evidence tetap dipertahankan sebagai lane penting, tetapi dipisahkan dari kelayakan operasional non-visual.
- Desktop supervision akan mencakup backend + PostgreSQL sidecar control minimum.
- Desktop dan VPS akan dikerjakan langsung keduanya, bukan serial desktop-first/VPS-first.

### Assumptions / Open Questions

- Tidak ada pertanyaan terbuka yang tersisa dari user decision gate untuk memulai execution plan ini.

### Readiness for Implementation

- Plan ini siap dieksekusi per phase.
- Tidak membutuhkan keputusan tambahan untuk mulai execution; Phase A dan Phase C dapat dibuka dengan strategi paralel yang terkoordinasi.

## Execution Status Snapshot

### Completed in this execution batch

- Phase A partially completed:
  - desktop backend supervision command surface implemented,
  - PostgreSQL sidecar supervision command surface implemented,
  - loopback-only safety guards added,
  - targeted Rust validation for `sidecar-postgres` passed.
- Phase C partially completed:
  - `smoke_systemd.sh` implemented,
  - `smoke_compose.sh` implemented,
  - compose smoke passed on alternate host ports,
  - `docker-compose.yml` host port/base URL parameterization added,
  - web healthcheck added.
- Phase E partially completed:
  - `release_closeout.sh` implemented,
  - dry closeout executed successfully.

### Still pending / blocked after this batch

- Phase C systemd live smoke remains pending on a host where `market-osource` and `market-osource-web` are actually installed/running.
- Desktop end-to-end crate validation is host-blocked by missing GTK/GLib dev packages.
- Phase F browser usability evidence remains blocked by missing Chromium/Chrome runtime.

### Cleanup

- Tidak ada draft terpisah yang dibuat untuk plan ini.
- Evidence lama tetap dipertahankan karena masih operasionally useful sebagai baseline/runtime blocker history.
