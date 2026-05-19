# Plan: Revamp UI/UX, RBAC, Install Flow, Demo Data Marketplace

Task ID: `20260519-2158-revamp-uiux-rbac-install`
Tanggal: 2026-05-19 (Asia/Bangkok)
Status: ready-for-implementation, beberapa task butuh keputusan saat eksekusi (lihat worklist).

## Goal

Membenahi pengalaman ujung ke ujung marketplace `market.osource.id` agar:
1. Pengguna VPS hanya melihat panel install saat first-run; setelah locked, root domain langsung menjadi storefront clothing-friendly.
2. RBAC marketplace mendukung 4 role operasional: `super_admin`, `admin`, `karyawan`, `seller` dengan login terbedakan dan authz backend yang konsisten.
3. Dashboard admin dirombak mengikuti arah visual template dashboard TweakCN (sidebar kiri persistent, header dengan user menu/role badge, KPI cards modern, table/recent activity, secondary panels) dan disesuaikan per role.
4. Marketplace memiliki demo data clothing company yang opsional (Batik Nusantara, Urban Threads, Modest Wear ID) yang dapat di-seed lewat install panel atau script.
5. Storefront, cart, checkout, order tracking, payment proof, dan settings menjadi lebih jelas, mobile-friendly, konsisten dengan `DESIGN.md`, dan tetap menjaga boundary keamanan PRD/TRD.
6. Reset install hanya tersedia di admin settings (super admin saja, double-confirm, audit) sebagai jalur recovery resmi.

## Non-goals

- Mengubah PRD/TRD scope inti (open-core, manual transfer, satu domain = satu marketplace, seller dibuat super admin).
- Menambah analytics/chart berat di dashboard. KPI hanya operasional MVP.
- Menambah payment gateway, promo, atau delivery package (tetap di Phase 3+).
- Membangun mobile native, SSR penuh untuk storefront, atau theme builder.
- Mengubah session model, password hashing, atau struktur protected media.
- Menyalin desain dari template/brand pihak ketiga.

## Scope

In scope:
- `apps/web/src/routes/**` (root, install, admin, storefront, login).
- `apps/web/src/lib/ui/**` (shell, primitives, status notice, badge).
- `apps/web/src/lib/api/base.ts` + `apps/web/src/lib/stores/cart.ts`.
- `crates/core-api/src/routes/admin.rs`, `auth.rs`, `system.rs`, `media.rs`, `install.rs`, `authz.rs`.
- `crates/core-db/src/auth.rs`, `commerce.rs`, `media.rs`, `install.rs`.
- `crates/core-installer/src/setup.rs` (demo seed flag + reset install pathway).
- Migration baru di `migrations/core/` untuk role tambahan dan kolom demo flag.
- Script seed di `scripts/demo/` untuk clothing company.
- Dokumentasi lokal di `.opencode/evidence/...`, `DESIGN.md` jika perlu update kecil.

Out of scope:
- Tauri desktop deep changes (di luar dampak konfigurasi).
- Sidecar postgres/cloudflared lifecycle besar.
- Backup/restore otomatis, log viewer Tauri.

## Requirements

### Produk
- Setelah install locked, root `/` = storefront marketplace; `/install` redirect ke `/store` jika dibuka kembali; landing marketing dipindah ke `/about` (opsional) atau dihapus.
- Sebelum install, root `/` redirect ke `/install`; `/admin` mengarahkan ke `/install` agar tidak menampilkan dashboard kosong.
- Login terpisah: `/admin/login` untuk operator (super_admin/admin/karyawan/seller), tetap satu form karena menggunakan field email + password; UI menampilkan label "Operator login".
- Dashboard `/admin` mengikuti arah visual template dashboard TweakCN (`https://tweakcn.com/editor/theme`) sebagai referensi style-equivalent: sidebar kiri persistent (logo + section nav + footer user/role), top bar (breadcrumb + search + theme indicator + user menu), KPI cards dengan delta/sparkline ringan opsional, blok recent orders dan task queue (payment menunggu verifikasi, order belum diproses), secondary panel system health. Token visual tetap memakai `apps/web/src/lib/styles/app.css` + `tailwind.config.ts` di repo (Tailwind 3 + TweakCN-derived tokens), bukan menyalin asset/kode TweakCN secara langsung.
- Demo data clothing dapat di-seed saat install (toggle "Isi data demo clothing company") atau via `cargo run -p` / `psql` script.
- Reset install hanya di `/admin/settings/install` untuk super admin, double-confirm, mencatat audit event.

### RBAC
- Role baru `admin` dan `karyawan` ditambahkan via migration additive, idempotent.
- Authorization helper diperluas: `require_super_admin`, `require_admin_or_super`, `require_brand_scope_member`.
- Admin dapat: kelola brand, seller, karyawan assignment, produk lintas brand, order, verifikasi payment proof. Tidak bisa: rotate secret, ubah install lock, reset install, ubah package entitlement.
- Karyawan dapat: produk dan order brand assigned saja, ubah status fulfillment brand assigned, lihat payment proof brand assigned bila punya `payment_proof.view_assigned`.
- Seller tetap brand-scoped owner; karyawan dibedakan dari seller via `brand_members.member_role` atau `users.role`.
- Endpoint mutation strategis tetap super_admin-only: install reset, package enable/disable, settings sensitive (storage/tunnel/secret).

### UX
- Mobile-first; semua tombol >=44px tinggi.
- Form punya label, error inline, success state.
- Loading state tidak menutup CTA utama.
- Reduced-motion dipertahankan.
- Status tidak hanya warna, selalu pasangan label/text.
- Empty state actionable.

### Data
- Demo data idempotent; aman dijalankan dua kali; punya kolom `is_demo` agar dapat dihapus dari admin settings.
- Demo data tidak masuk migration core. Letak di `scripts/demo/seed_clothing.sql` dan helper Rust opsional di `crates/core-installer`.

### Security
- Semua authz tetap di backend; UI hanya menyembunyikan menu.
- Reset install endpoint super_admin-only + CSRF + rate limit + audit event.
- Demo seed wajib menolak dijalankan jika `installation_state.state != 'locked'` saat dipanggil dari admin pasca-install (kecuali pada install panel pre-lock).
- Tidak ada perubahan akses public ke payment proof.

## Acceptance Criteria

1. Setelah install locked, mengetik domain root menampilkan storefront, bukan landing marketing dan bukan install panel.
2. Mengakses `/install` setelah locked menampilkan notice "Marketplace sudah ter-install" dan tombol ke `/admin` atau `/store`, atau redirect ke `/store`.
3. Login operator mendukung 4 role; user `karyawan` brand A tidak dapat melihat data brand B di API maupun UI.
4. Dashboard admin mengikuti arah visual TweakCN dashboard template: sidebar kiri persistent dengan grouping nav, top bar (breadcrumb + user menu + role badge + theme/health indicator), minimal 4 KPI cards bergaya modern (judul, value tabular-nums, helper, trend opsional), blok recent orders, task queue, dan system health panel. Layout responsif untuk 1440 / 768 / 390 dengan sidebar collapsible/drawer di mobile. Implementasi memakai token internal repo (Tailwind 3 + TweakCN-derived) tanpa menyalin asset proprietary.
5. Operator yang menyalakan toggle demo data saat install mendapatkan minimal 3 brand demo, 3 seller demo, dan 12 produk clothing dengan variant size/warna; data tampil di storefront.
6. Reset install dari admin settings hanya bisa dijalankan super_admin, butuh password re-entry atau double-confirm, dan menulis `audit_events`.
7. `npm run check` dan `npm run build` di `apps/web` lulus tanpa error.
8. `cargo test -p core-api -p core-db -p core-auth` di Docker `rust:1.88` lulus tanpa regresi pada test sekarang.
9. Playwright smoke flow lulus untuk: install pre-lock → storefront post-lock, login admin, dashboard, order tracking, demo data toggle.
10. Tidak ada peningkatan akses publik untuk private media; test seller_isolation tetap hijau.

## Existing Patterns / Reuse

- Sidebar AdminShell: gunakan ulang struktur `lg:grid-cols-[260px_1fr]`, perluas slot user-menu + role badge.
- StoreShell: tambah cart count chip dan link kategori clothing.
- Token theme dan utility class di `app.css` (`panel`, `metric-card`, `list-row`, `tabular-nums`).
- `StatusBadge`, `StateNotice`.
- `apiUrl`, `cart` store.
- Backend helper `core-db::user_is_brand_member`, `seller_can_access_order_brand_group`, `user_has_brand_scoped_permission_for_order`.
- `core-installer::setup` sebagai single transaction setup → diperluas dengan demo seed step opsional.
- `pg_advisory_xact_lock` pattern → reuse untuk reset install.

Reuse > Extend > Create:
- Reuse: AdminShell, StoreShell, StateNotice, StatusBadge, panel utilities, install lock helper, brand_members, brand_member_permissions, payment proof flow.
- Extend: AdminShell (header user/role/notification slot), StoreShell (cart count + brand link), apiUrl (apiFetch helper + getCsrfToken cache), install setup (demo seed step), authz module (admin/karyawan helpers).
- Create: server `+page.server.ts` di root + `/install`, primitives `InputField`, `SubmitButton`, `FormSection`, `AsyncState`, dashboard summary endpoint, demo seed script + admin settings reset install screen.

Tidak ada KiloCode/utility eksternal yang lebih cocok dari yang sudah ada di repo untuk masalah ini.

## Constraints

- Tetap memakai Svelte 5 + Tailwind 3 + Axum + sqlx; tanpa upgrade major framework.
- Migration additive saja; tidak boleh destructive.
- Demo data hanya boleh berjalan eksplisit (toggle install atau endpoint super_admin); tidak otomatis.
- Boundary domain (`core-domain` tidak depend ke axum/sqlx) tetap dihormati.
- Tidak boleh menambah dependency front-end besar tanpa audit; jika menambah `lucide-svelte`, harus tetap menjaga bundle size dan accessibility.
- Reduced-motion dan focus-visible tetap dijaga.

## Risks

- Refactor literal `"super_admin"`/`"seller"` ke matriks 4 role berisiko membocorkan brand isolation jika audit tidak menyeluruh.
- Redirect server-side dari `/` ke `/store` butuh `+page.server.ts` baru; kalau API `/api/install/state` lambat/error, root bisa loop atau blank.
- Demo seed bisa men-trigger ulang data; harus idempotent dengan `is_demo` + `slug` unique.
- Reset install di admin settings memperbesar attack surface; harus ada audit + rate limit + double-confirm.
- Mengubah AdminShell mengubah semua halaman admin sekaligus; perlu test visual per breakpoint.
- Menggeser landing marketing ke `/about` mungkin merusak referensi eksternal (link di README atau docs internal); cek sebelum hapus.
- Test fixture Rust banyak mengasumsikan dua role saja; menambah role memperpanjang fixture dan test.

## Decisions / Assumptions

Dikunci dari Question Gate:
- D1 Role: `admin` = ops marketplace tanpa settings sistem; `karyawan` = staff brand-scoped via `brand_members`. Seller tetap. Super admin tetap.
- D2 Storefront default: setelah locked, `/` = storefront. `/install` redirect ke `/store` setelah locked.
- D3 Demo data: script terpisah di `scripts/demo/seed_clothing.sql` plus toggle di install panel.
- D4 Dashboard: revamp total mengikuti arah visual template dashboard TweakCN (sidebar kiri persistent, top bar, KPI cards modern, recent orders, task queue, secondary panels). Reuse token TweakCN-derived sudah ada di `app.css` + `tailwind.config.ts`. Tidak menyalin asset/kode proprietary; semua adopsi bersifat style-equivalent.
- D5 Reset install: tombol di admin settings, super_admin-only, double-confirm.
- D6 Plan: satu primary plan dengan worklist execution-ready.
- D7 Test: backend TDD ketat (Rust tests), frontend Playwright smoke + svelte-check.

Asumsi:
- A1 Karyawan tidak boleh membuat brand atau seller. Hanya mengelola produk dan order brand assigned. Permission file payment proof tetap eksplisit via `brand_member_permissions` jika diperlukan.
- A2 Admin dapat membuat seller dan karyawan, mengelola assignment brand, verifikasi payment proof; tidak boleh ubah install lock atau secret.
- A3 Demo seed dipanggil server-side oleh backend lewat use case di `core-installer`, bukan dari frontend langsung mengeksekusi SQL.
- A4 Landing marketing dipindah ke `/about` (opsional). Jika tidak diperlukan, cukup dihapus.
- A5 Tidak menambah library icon baru pada milestone 1; gunakan StatusBadge dan teks. `lucide-svelte` masuk pertimbangan opsional pada milestone polishing.
- A6 Tidak ada perubahan UI Tauri/desktop; perubahan setup wizard hanya di web install panel.

Open questions yang dapat dikonfirmasi saat eksekusi:
- OQ1 Apakah `karyawan` perlu permission tambahan canonical (mis. `order.update_fulfillment_assigned`) di `permissions` atau cukup default scope `brand_members`? Asumsi default scope cukup; bila perlu eskalasi → tambah permission code dan migration.
- OQ2 Apakah landing marketing `/` saat ini perlu dipertahankan sebagai `/about`? Default: pindah, bisa di-skip jika user menolak.
- OQ3 Apakah demo data perlu seed customer/order contoh atau cukup brand+produk? Asumsi: brand + seller + produk + variant + 2 order contoh (tanpa payment proof) untuk demo dashboard.

## TDD / Test Plan

TDD diperlukan: ya, untuk semua perubahan authz, install, dan demo seed. UI berikut Playwright smoke + screenshot baseline.

Existing test patterns yang dipakai:
- `core-db/src/install.rs` test `super_admin_count`, lock concurrent.
- `core-api/src/routes/admin.rs` integration test pattern.
- `core-api/src/authz.rs` unit test `super_admin_helper_rejects_non_super_admin`.

Red → Green → Refactor checklist:

1. Authz role matrix
   - Red: `core-auth::permission` test gagal saat `admin` mencoba kelola brand seharusnya allow, dan `karyawan` mencoba akses brand B harus deny.
   - Green: tambah role + helper `require_admin_or_super`, `require_brand_scope_member`.
   - Refactor: hapus literal hardcoded role di handler.

2. Install lock + demo seed
   - Red: test setup gagal saat demo flag aktif tapi seed belum jalan; test setup ulang setelah locked harus tetap 409.
   - Green: extend `core-installer::setup` untuk panggil seed clothing idempotent jika `seed_demo=true`.
   - Refactor: pisahkan demo seed runner ke modul tersendiri.

3. Reset install
   - Red: endpoint `/api/admin/install/reset` 403 untuk admin/karyawan/seller, 401 untuk anonymous, 200 untuk super_admin dengan double-confirm token.
   - Green: implementasi handler + audit event + advisory lock.
   - Refactor: rate limit + clear UI message.

4. Dashboard summary endpoint
   - Red: test scope per role (super_admin/admin lihat semua, seller/karyawan brand assigned saja).
   - Green: implementasi `/api/admin/dashboard/summary`.
   - Refactor: query gabungan tanpa N+1.

5. Frontend
   - svelte-check on every change.
   - Playwright smoke: install pre-lock → setup → root storefront → admin login → dashboard → orders.
   - Visual snapshot untuk 1440/768/390 di dashboard, install, storefront, login.

Edge cases:
- Concurrent setup dengan/ tanpa demo flag.
- Reset install saat sesi admin lain aktif.
- Karyawan tanpa assignment brand sama sekali (UI harus empty state instruktif).
- Storefront dengan zero brand aktif (root harus tetap valid, tampilkan "Belum ada brand aktif").

Validation commands:
- `npm --prefix apps/web run check`
- `npm --prefix apps/web run build`
- Playwright: `npx playwright test` (setelah konfigurasi awal)
- Backend: `docker run --rm -v "$PWD":/app -w /app rust:1.88 cargo test --workspace --exclude market-desktop` (sesuai pola README)

## Implementation Steps

Fase 1 — Foundation routing + RBAC backend:
1. Tambah migration `migrations/core/0007_roles_admin_karyawan.sql` (idempotent insert role baru, optional kolom `is_demo` di `brands`/`products`/`orders`).
2. Update `core-db::auth` agar mengakomodasi role baru tanpa breakage (helper `is_admin_or_super`, `is_seller_or_karyawan`).
3. Refactor `core-api::authz` menambah helper baru.
4. Audit dan refactor `routes/admin.rs`, `system.rs`, `media.rs`, `install.rs` agar memakai helper baru.
5. Tambah test unit/integration untuk role matrix.
6. Tambah endpoint `GET /api/admin/dashboard/summary` scoped per role.

Fase 2 — Install gating + storefront default:
1. Buat `apps/web/src/routes/+layout.server.ts` (atau `hooks.server.ts`) dengan helper `getInstallState`.
2. Tambah `apps/web/src/routes/+page.server.ts`: pre-install → redirect `/install`; locked → redirect `/store`.
3. Tambah `apps/web/src/routes/(install)/install/+page.server.ts`: locked → redirect `/store`.
4. Pindahkan landing marketing existing menjadi `/about` (opsional) atau hapus.
5. Update `+layout.svelte` jika perlu shared shell/announcement.

Fase 3 — UI shell + primitives:
1. Tambah primitives `lib/ui/forms/InputField.svelte`, `SubmitButton.svelte`, `FormSection.svelte`, `AsyncState.svelte`.
2. Tambah helper `lib/api/base.ts` → `apiFetch()`, `getCsrfToken()` (memo cache per session).
3. Refactor AdminShell: tambah header user-menu, role badge, mobile drawer; sidebar tetap konsisten.
4. Refactor StoreShell: tambah cart counter + link kategori/brand.

Fase 4 — Dashboard revamp (TweakCN style):
1. Capture reference TweakCN dashboard di `https://tweakcn.com/editor/theme` (atau halaman dashboard preview) di viewport 1440/768/390 dengan workflow wait-stabilize-scroll-settle. Simpan di `.opencode/evidence/<task-id>/reference-captures.md` + `reference/` folder. Catat: digunakan hanya sebagai referensi style-equivalent, bukan untuk disalin.
2. Susun visual spec ringkas (sidebar anatomy, top bar anatomy, KPI card anatomy, recent orders table style, secondary panel) di plan section atau draft `visual-notes.md` jika diperlukan.
3. Bangun `AdminShell` + dashboard `/admin` dengan struktur: sidebar kiri persistent (logo + section nav + user/role footer), top bar (breadcrumb, search/quick action, theme/health, user menu), grid KPI cards (4 minimum), blok recent orders table, task queue list, system health panel.
4. KPI cards: Brands aktif, Produk published, Order open, Payment menunggu verifikasi. Tambah delta/badge ringan, tanpa chart berat di MVP.
5. Variasi per role: super_admin/admin lihat semua brand; seller/karyawan brand assigned. UI menampilkan scope eksplisit pada header.
6. Wire ke `/api/admin/dashboard/summary` lewat `+page.server.ts`.
7. Validasi a11y: focus visible, target >=44px, status tidak hanya warna, reduced-motion dijaga.
8. Capture final dashboard di 1440/768/390 dan tulis visual comparison vs reference TweakCN style-equivalent.

Fase 5 — Storefront clothing demo:
1. Update `/store` jadi storefront katalog clothing-friendly: hero, brand strip, filter (kategori, brand), product grid, pagination.
2. Reuse `static/assets/demo/product-*.png` sebagai placeholder demo; jika seed clothing aktif, gunakan asset gambar terkait.
3. Update cart/checkout/order tracking UX (stepper, instruksi pembayaran manual, upload bukti transfer responsif).

Fase 6 — Demo data clothing company:
1. Buat `scripts/demo/seed_clothing.sql` idempotent: brand `batik-nusantara`, `urban-threads`, `modest-wear-id`; seller demo per brand; produk + variant size/warna + harga + stok; flag `is_demo`.
2. Tambah `core-installer::demo` runner Rust yang membaca file SQL atau membangun query parameter (lebih aman) dan menjalankan transaction.
3. Tambah toggle "Isi data demo clothing" di install panel + checkbox `seed_demo`.
4. Tambah endpoint admin settings `POST /api/admin/demo/seed` dan `DELETE /api/admin/demo/clear`, super_admin-only.
5. Audit event untuk seed/clear.

Fase 7 — Auth + login UX + reset install:
1. Bangun `/admin/login` baru: title "Operator login", info role, link "Lupa password" (placeholder docs), responsive.
2. Tambah `/admin/settings` shell dengan tab (Marketplace, Storage, Tunnel, Package, Demo data, Install reset).
3. `Install reset` super_admin-only: butuh password re-entry + checkbox "Saya paham data tidak akan dipulihkan", panggil `POST /api/admin/install/reset`, advisory lock, audit event.

Fase 8 — Documentation + visual capture:
1. Update `DESIGN.md` jika ada token/utility baru.
2. Update `.opencode/evidence/<task-id>/*` dengan screenshot current/final dan visual comparison.
3. Update `README.md` (mode demo + new role) bila perlu.

## Expected Files to Change

Backend:
- `migrations/core/0007_roles_admin_karyawan.sql` (baru)
- `crates/core-db/src/auth.rs`
- `crates/core-db/src/install.rs`
- `crates/core-db/src/commerce.rs`
- `crates/core-db/src/media.rs`
- `crates/core-api/src/authz.rs`
- `crates/core-api/src/routes/admin.rs`
- `crates/core-api/src/routes/system.rs`
- `crates/core-api/src/routes/media.rs`
- `crates/core-api/src/routes/install.rs`
- `crates/core-api/src/routes/auth.rs`
- `crates/core-api/src/router.rs`
- `crates/core-installer/src/setup.rs`
- `crates/core-installer/src/demo.rs` (baru)
- `apps/backend/src/main.rs` jika ada wiring baru

Frontend:
- `apps/web/src/hooks.server.ts` (baru, opsional)
- `apps/web/src/routes/+layout.server.ts` (baru)
- `apps/web/src/routes/+page.svelte` (refactor) atau pindahkan ke `/about`
- `apps/web/src/routes/+page.server.ts` (baru)
- `apps/web/src/routes/(install)/install/+page.server.ts` (baru)
- `apps/web/src/routes/(install)/install/+page.svelte` (revamp + demo toggle)
- `apps/web/src/routes/(admin)/admin/+page.svelte` (revamp dashboard)
- `apps/web/src/routes/(admin)/admin/+layout.svelte` (baru jika perlu role guard)
- `apps/web/src/routes/(admin)/admin/login/+page.svelte` (revamp)
- `apps/web/src/routes/(admin)/admin/settings/+page.svelte` (baru)
- `apps/web/src/routes/(admin)/admin/settings/install/+page.svelte` (baru, reset install)
- `apps/web/src/routes/(admin)/admin/settings/demo/+page.svelte` (baru)
- `apps/web/src/routes/(admin)/admin/brands/+page.svelte`
- `apps/web/src/routes/(admin)/admin/products/+page.svelte`
- `apps/web/src/routes/(admin)/admin/orders/+page.svelte`
- `apps/web/src/routes/(admin)/admin/system/+page.svelte`
- `apps/web/src/routes/(storefront)/store/+page.svelte` (revamp)
- `apps/web/src/routes/(storefront)/store/cart/+page.svelte`
- `apps/web/src/routes/(storefront)/store/checkout/+page.svelte`
- `apps/web/src/routes/(storefront)/store/orders/[trackingToken]/+page.svelte`
- `apps/web/src/lib/ui/AdminShell.svelte` (extend)
- `apps/web/src/lib/ui/StoreShell.svelte` (extend)
- `apps/web/src/lib/ui/forms/InputField.svelte` (baru)
- `apps/web/src/lib/ui/forms/SubmitButton.svelte` (baru)
- `apps/web/src/lib/ui/forms/FormSection.svelte` (baru)
- `apps/web/src/lib/ui/AsyncState.svelte` (baru)
- `apps/web/src/lib/api/base.ts`
- `apps/web/src/lib/stores/cart.ts`
- `apps/web/playwright.config.ts` (baru)
- `apps/web/tests/smoke/*.spec.ts` (baru)

Demo + ops:
- `scripts/demo/seed_clothing.sql` (baru)
- `scripts/ops/reset_install.sql` (baru, fallback manual)

Docs/evidence:
- `.opencode/evidence/20260519-2158-revamp-uiux-rbac-install/*`
- Update `DESIGN.md`/`README.md` jika perlu

## Agent / Tool Routing

- `@orchestrator` koordinasi worklist.
- `@fixer` implementasi backend Rust (authz, install, demo seed) dan frontend Svelte (shell, primitives, dashboard, storefront, admin pages, Playwright). Termasuk migration baru.
- `@designer` review UI direction, density, motion, accessibility, capture screenshots.
- `@oracle` review boundary authz, migration risk, reset install endpoint.
- `@quality-gate` final signoff: security, accessibility, visual parity claim, evidence.
- `@architect` (optional) bila tradeoff role/karyawan boundary muncul saat implementasi.
- `@librarian` jika butuh konfirmasi docs SvelteKit/Playwright tambahan.

MCP / tools yang diharapkan:
- Playwright (browser MCP) untuk capture current/final.
- context7 untuk SvelteKit/Tauri jika perlu.
- GitHub search jika perlu mencari pola sejenis di repo lain (opsional).

## Execution-ready Worklist / Handoff Contract

`start_with: T01`

T01 Backend role matrix migration + helper
- depends_on: none
- owner: @fixer
- action: tulis migration `migrations/core/0007_roles_admin_karyawan.sql` (insert `admin`, `karyawan`, optional `is_demo` columns) + helper authz baru di `core-auth/core-api`.
- validation: `docker run --rm -v "$PWD":/app -w /app rust:1.88 cargo test -p core-db -p core-auth -p core-api`.
- exit: migration idempotent, role baru tersedia di test fixture, helper baru lulus unit test.
- status: ready
- requires_user_decision: no

T02 Refactor authz literal di handler
- depends_on: T01
- owner: @fixer
- action: hapus literal `"super_admin"`/`"seller"` di `routes/admin.rs`, `media.rs`, `system.rs`; gunakan helper.
- validation: `cargo test -p core-api`.
- exit: tidak ada `role_code == "super_admin"` literal di handler kecuali helper terpusat; test seller isolation tetap hijau.
- status: ready
- requires_user_decision: no

T03 Endpoint dashboard summary
- depends_on: T02
- owner: @fixer
- action: tambah `GET /api/admin/dashboard/summary` scoped per role; query gabungan brand/product/order/payment.
- validation: `cargo test -p core-api -- dashboard`.
- exit: endpoint mengembalikan KPI scoped; test super_admin/admin/seller/karyawan lulus.
- status: ready
- requires_user_decision: no

T04 SvelteKit root + install gating
- depends_on: none (frontend independent dari role baru, hanya butuh `/api/install/state`)
- owner: @fixer
- action: tambah `+layout.server.ts` + `/+page.server.ts` + `(install)/install/+page.server.ts` dengan redirect berdasar install state.
- validation: `npm --prefix apps/web run check && npm --prefix apps/web run build`.
- exit: pre-install → `/install`; post-install root → `/store`; `/install` post-lock → `/store`.
- status: ready
- requires_user_decision: no

T05 UI primitives + helper api fetch
- depends_on: T04
- owner: @fixer
- action: tambah `InputField`, `SubmitButton`, `FormSection`, `AsyncState`, `apiFetch`, `getCsrfToken`. Refactor minimal di install/login.
- validation: `npm run check && npm run build`.
- exit: primitives dipakai minimal di 2 halaman; tidak ada regresi check/build.
- status: ready
- requires_user_decision: no

T06 AdminShell + StoreShell extension
- depends_on: T05
- owner: @fixer + review @designer
- action: AdminShell tambah header user-menu, role badge, mobile drawer; StoreShell tambah cart count + link kategori; dukung 4 role di nav scope.
- validation: `npm run check && npm run build` + Playwright snapshot di breakpoint 1440/768/390 setelah Playwright tersedia.
- exit: shell konsisten lintas halaman, focus visible, reduced-motion dijaga.
- status: ready
- requires_user_decision: no

T07 Dashboard revamp (TweakCN style-equivalent)
- depends_on: T03, T06
- owner: @fixer + review @designer
- action: capture reference TweakCN dashboard (style-equivalent only, no asset copy), susun visual spec ringkas, bangun `/admin` baru: sidebar persistent, top bar, KPI cards, recent orders, task queue, system health panel. Variasi per role.
- validation: `npm run check`, browser screenshot 1440/768/390, visual comparison vs reference catatan style-equivalent.
- exit: dashboard memenuhi acceptance criteria 4 dan 5; reference + final captures tersimpan di `.opencode/evidence/<task-id>/`.
- status: ready
- requires_user_decision: no

T08 Storefront clothing revamp
- depends_on: T06
- owner: @fixer + review @designer
- action: revamp `/store` (hero, brand strip, filter, product grid, pagination), update cart/checkout/order tracking UX.
- validation: `npm run check`, screenshot mobile/tablet/desktop.
- exit: storefront menampilkan demo clothing dengan rapi; checkout responsif.
- status: ready
- requires_user_decision: no

T09 Login + admin settings shell
- depends_on: T06
- owner: @fixer
- action: revamp `/admin/login` (Operator login), tambah `/admin/settings` shell dengan tab (Marketplace, Storage, Tunnel, Package, Demo data, Install reset).
- validation: `npm run check`, smoke navigation.
- exit: settings shell siap menerima sub-page T10/T11.
- status: ready
- requires_user_decision: no

T10 Demo data clothing company
- depends_on: T01
- owner: @fixer
- action: tulis `scripts/demo/seed_clothing.sql` idempotent + runner di `core-installer::demo`. Tambah toggle install + endpoint `POST /api/admin/demo/seed` dan `DELETE /api/admin/demo/clear` (super_admin). UI di `/admin/settings/demo`.
- validation: `cargo test -p core-installer -p core-api -- demo`, `npm run check`.
- exit: data brand/seller/produk demo tampil; bisa di-clear.
- status: ready
- requires_user_decision: confirm OQ3 saat eksekusi (apakah seed termasuk order contoh).

T11 Reset install di admin settings
- depends_on: T01, T09
- owner: @fixer + review @oracle
- action: implementasi `POST /api/admin/install/reset` super_admin-only + double-confirm + advisory lock + audit event. UI di `/admin/settings/install`.
- validation: `cargo test -p core-api -- install_reset`, smoke Playwright.
- exit: hanya super_admin yang dapat eksekusi; audit event tertulis; install kembali dapat berjalan setelah reset.
- status: blocked (perlu konfirmasi UX double-confirm: password re-entry vs typed phrase) → resolved jika user pilih default password re-entry.
- requires_user_decision: yes (konfirmasi metode double-confirm, default: password re-entry).

T12 Playwright smoke + visual parity
- depends_on: T07, T08, T09, T10
- owner: @fixer
- action: konfigurasi Playwright (`apps/web/playwright.config.ts`), tulis smoke spec install→storefront→login→dashboard→orders→demo toggle, capture screenshot 1440/768/390.
- validation: `npx playwright test`.
- exit: smoke hijau; screenshot tersimpan di `.opencode/evidence/<task-id>/`.
- status: ready
- requires_user_decision: no

T13 Documentation + cleanup
- depends_on: T07–T12
- owner: @fixer + @designer
- action: update `DESIGN.md` (jika ada token baru), `README.md` (mode demo, role baru), evidence (screenshots, comparisons), final designer review, hapus draft yang tidak perlu.
- validation: link check ringan + read-through.
- exit: dokumentasi konsisten, evidence siap untuk quality gate.
- status: ready
- requires_user_decision: no

T14 Quality gate signoff
- depends_on: T01–T13
- owner: @quality-gate
- action: review evidence (screenshots, test logs, audit logs), security check (reset install, demo gating, role audit), accessibility check.
- validation: gate report.
- exit: approved atau revision list.
- status: ready
- requires_user_decision: no

## Validation Commands

- `npm --prefix apps/web run check`
- `npm --prefix apps/web run build`
- `npm --prefix apps/web exec -- playwright test` (setelah konfigurasi)
- `docker run --rm -v "$PWD":/app -w /app rust:1.88 cargo test --workspace --exclude market-desktop`
- `docker run --rm -v "$PWD":/app -w /app rust:1.88 cargo clippy --workspace --exclude market-desktop --all-targets -- -D warnings`
- `docker run --rm -v "$PWD":/app -w /app rust:1.88 cargo fmt --all -- --check`
- Smoke endpoint: `curl -s http://localhost:7300/api/install/state | jq .`
- Audit DB: `psql -c "SELECT action, target_type, created_at FROM audit_events ORDER BY created_at DESC LIMIT 10"`

## Evidence Requirements

Wajib hadir sebelum quality gate signoff:
- Backend test logs untuk authz role matrix, install reset, dashboard summary, demo seed.
- Frontend `npm run check` + `npm run build` log.
- Playwright smoke test log.
- Screenshot 1440x1200, 768x1024, 390x844 untuk: `/`, `/install`, `/admin/login`, `/admin`, `/admin/settings/install`, `/admin/settings/demo`, `/store`, `/store/cart`, `/store/checkout`, `/store/orders/[token]`.
- Audit log entries: `install.reset`, `demo.seed`, `demo.clear`, `auth.login.success` per role.
- Visual comparison ringkas di `.opencode/evidence/<task-id>/visual-comparison.md`.
- Final designer review di `.opencode/evidence/<task-id>/final-designer-review.md`.

## Done Criteria

- Semua acceptance criteria 1–10 tercapai.
- Tidak ada regresi pada test backend existing.
- Tidak ada regresi keamanan (seller isolation, payment proof private, install lock).
- Dokumentasi `DESIGN.md`/`README.md`/`.opencode` selaras dengan implementasi.
- Quality gate approved.

## Final Planning Summary

Plan ini ditulis sebagai source of truth tunggal di `.opencode/plans/20260519-2158-revamp-uiux-rbac-install.md`. Question gate dilakukan dan dijawab oleh user; semua keputusan material terkunci di seksi Decisions. Discovery dan evidence skeleton ditulis di:

- `.opencode/evidence/20260519-2158-revamp-uiux-rbac-install/discovery.md`
- `.opencode/evidence/20260519-2158-revamp-uiux-rbac-install/reference-captures.md`
- `.opencode/evidence/20260519-2158-revamp-uiux-rbac-install/current-captures.md`
- `.opencode/evidence/20260519-2158-revamp-uiux-rbac-install/visual-comparison.md`
- `.opencode/evidence/20260519-2158-revamp-uiux-rbac-install/animation-audit.md`
- `.opencode/evidence/20260519-2158-revamp-uiux-rbac-install/icon-system-audit.md`
- `.opencode/evidence/20260519-2158-revamp-uiux-rbac-install/generated-assets.md`
- `.opencode/evidence/20260519-2158-revamp-uiux-rbac-install/final-designer-review.md`

Tidak ada draft tambahan yang dibuat; jika perlu dibuka saat eksekusi (mis. detail asset prompt), simpan di `.opencode/draft/20260519-2158-revamp-uiux-rbac-install/`. Tidak ada artifact yang harus dihapus setelah finalisasi karena evidence di atas masih operasional untuk fase implementasi.

Open questions tersisa:
- OQ1 permission canonical untuk karyawan;
- OQ2 nasib landing marketing;
- OQ3 demo seed mencakup order/payment proof contoh;
- T11 metode double-confirm (password re-entry default).

Readiness: ready-for-implementation. T11 menunggu konfirmasi cepat saat eksekusi; sisanya dapat dimulai dari T01 oleh `@fixer` dengan koordinasi `@orchestrator` dan review `@designer`/`@oracle`/`@quality-gate`.
