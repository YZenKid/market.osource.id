# Discovery — Revamp UI/UX, RBAC, dan Install Flow

Task ID: `20260519-2158-revamp-uiux-rbac-install`
Tanggal: 2026-05-19 (Asia/Bangkok)

## Files yang diinspeksi

Backend / Rust workspace:
- `crates/core-db/src/install.rs` (install state + lock + super admin bootstrap)
- `crates/core-db/src/auth.rs` (session model + role lookup)
- `crates/core-db/src/commerce.rs` (brand/seller/order queries scoped per user)
- `crates/core-db/src/media.rs` (Gate F payment proof permission queries)
- `crates/core-api/src/authz.rs` (`require_super_admin`, `AuthenticatedActor`)
- `crates/core-api/src/routes/admin.rs` (admin endpoints, role check hardcoded `super_admin`/`seller`)
- `crates/core-api/src/routes/install.rs` + `system.rs` (install/runtime endpoints)
- `crates/core-api/src/routes/media.rs` (private media `/media/:file_id`)
- `crates/core-api/src/routes/storefront.rs` (storefront katalog/cart/checkout)
- `crates/core-installer/src/setup.rs` + `preflight.rs`
- `migrations/core/0002_users_auth.sql` (roles seed: hanya `super_admin` + `seller`)
- `migrations/core/0006_payment_proof_permissions.sql`

Frontend / SvelteKit:
- `apps/web/package.json` (Svelte 5, Tailwind 3, adapter-node, `npm run check`/`build`)
- `apps/web/src/routes/+page.svelte` (landing marketing statis di `/`)
- `apps/web/src/routes/+layout.svelte`
- `apps/web/src/routes/(install)/install/+page.svelte` + `+page.ts` (`ssr=false`, fetch CSRF, setup, preflight)
- `apps/web/src/routes/(admin)/admin/+page.svelte` (dashboard ringan, hardcode role `super_admin`)
- `apps/web/src/routes/(admin)/admin/login/+page.svelte`
- `apps/web/src/routes/(admin)/admin/brands/+page.svelte` (form besar: brand + seller + assignment di satu page)
- `apps/web/src/routes/(admin)/admin/products/+page.svelte`
- `apps/web/src/routes/(admin)/admin/orders/+page.svelte` (manual grant brand_member_permissions)
- `apps/web/src/routes/(admin)/admin/packages/+page.svelte`
- `apps/web/src/routes/(admin)/admin/system/+page.svelte`
- `apps/web/src/routes/(storefront)/store/+page.svelte`
- `apps/web/src/routes/(storefront)/store/cart/+page.svelte`
- `apps/web/src/routes/(storefront)/store/checkout/+page.svelte`
- `apps/web/src/routes/(storefront)/store/orders/[trackingToken]/+page.svelte`
- `apps/web/src/lib/ui/AdminShell.svelte` (sidebar 6 item, helper text)
- `apps/web/src/lib/ui/StoreShell.svelte` (top nav pill: Produk/Cart/Checkout)
- `apps/web/src/lib/ui/StateNotice.svelte` + `StatusBadge.svelte`
- `apps/web/src/lib/styles/app.css` (token theme + utility class panel/metric-card/list-row)
- `apps/web/src/lib/api/base.ts` + `apps/web/src/lib/stores/cart.ts`

Dokumen sumber:
- `PRD.md`, `TRD.md`, `ERD.md`, `DESIGN.md`, `README.md`
- `.opencode/plans/20260511-2156-project-completion-master-plan.md`
- `.opencode/docs/AGENT_ROUTING.md` (lanes, ownership)

## Pola yang sudah ada di repo

- Stack tetap: Rust Axum + SvelteKit + Tailwind. `npm run check` dan `npm run build` menjadi gate frontend.
- Install lock benar di backend: `pg_advisory_xact_lock(7_310_202_607)` + `FOR UPDATE` + `AlreadyLocked` (HTTP 409). Concurrent test sudah ada di `core-db/install.rs`.
- Auth memakai server-side session + Argon2id + CSRF cookie/header (`/api/auth/csrf`, `/api/auth/login`, `/api/auth/me`).
- Brand-scoped RBAC seller sudah ada via `brand_members` + helper `user_is_brand_member` + `seller_can_access_order_brand_group`.
- Payment proof permission granular sudah ada via `brand_member_permissions` (`payment_proof.view_assigned`, `payment_proof.verify`, `payment_proof.reject`). Super Admin bypass.
- UI memakai pola token-based: `panel`, `metric-card`, `list-row`, `tabular-nums`. `StateNotice` dan `StatusBadge` reusable.
- Asset demo ada di `apps/web/static/assets/demo/product-1.png` … `product-3.png` plus ilustrasi onboarding/empty/upsell.

## Gap yang teridentifikasi

Routing dan install gating:
- `/` selalu landing marketing statis. Tidak ada redirect berbasis install state.
- `/install` tidak di-redirect setelah `installation_state.locked_at` terisi; lock hanya enforced di POST `/api/install/setup`. UX bingung: pengguna VPS yang sudah install bisa kembali ke install panel.
- Endpoint install state/preflight tetap accessible setelah locked (acceptable untuk read), tapi UI install panel harus segera redirect.

RBAC:
- Roles seed di migration hanya `super_admin` dan `seller`. Tidak ada `admin` (ops marketplace) atau `karyawan` (staff brand internal).
- Semua check otorisasi memakai literal `role_code == "super_admin" | "seller"`. Menambah role baru wajib audit dan refactor di `core-api/src/routes/admin.rs`, `media.rs`, `system.rs`, `authz.rs`.
- UI dashboard hardcode redirect ke unauthenticated ketika `role.code !== 'super_admin'`. Seller/karyawan tidak punya dashboard sendiri.
- Tidak ada login page yang terbedakan untuk operator vs storefront customer.

Demo data:
- Tidak ada `scripts/demo/` atau seed clothing company. README menyebut mode desktop cocok untuk demo, tapi tidak ada script aktual.
- Asset gambar `apps/web/static/assets/demo/product-{1,2,3}.png` sudah tersedia, bisa direuse untuk seed.

Dashboard:
- Dashboard admin saat ini hanya 4 status card statis + 5 link card sub-page. Tidak ada KPI order, payment menunggu verif, recent orders, atau status fulfillment.
- Frontend memanggil `/api/admin/brands` + `/api/admin/products` terpisah untuk count. Tidak ada `/api/admin/dashboard/summary`.
- AdminShell sudah top-down sidebar layout, tapi tidak punya: user menu, role badge, breadcrumbs, notification center, search.

Storefront:
- Product card masih placeholder kotak (no image), tidak ada filter brand/kategori, tidak ada pagination, tidak ada hero section.
- StoreShell hanya 3 link top nav (Produk / Cart / Checkout). Tidak ada link brand atau katalog kategori.

Form/UX:
- Banyak halaman menggunakan `onMount` fetch (no server `load`). Loading state agresif, FCP buruk untuk halaman terproteksi.
- `fetchCsrfToken()` direplika di install/login/brands/products/orders. Tidak ada helper bersama.
- Form input/button memakai class Tailwind inline berulang. Tidak ada primitif `InputField`, `SubmitButton`, `FormSection`, `AsyncState`.

Test posture:
- Backend punya unit test (mis. `super_admin_helper_rejects_non_super_admin`).
- Frontend hanya `svelte-check` + `vite build`. Belum ada Playwright.

## Reuse candidates

- `AdminShell` (sidebar + header) → extend dengan user menu + role badge.
- `StoreShell` → extend dengan link kategori/brand + cart count.
- `StateNotice` + `StatusBadge` → tetap.
- `apiUrl()` di `lib/api/base.ts` → tambahkan helper `apiFetch()` + `getCsrfToken()`.
- `cart.ts` store → tetap.
- `panel`/`metric-card`/`list-row` utility class → tetap, jadi basis primitives baru.
- `core-db::user_is_brand_member` + `brand_member_permissions` → tetap source of truth untuk authz.
- Asset `apps/web/static/assets/demo/product-*.png` → reuse untuk seed clothing.

## Commands & docs yang sudah dicek

- `npm run check`, `npm run build`, `npm run dev` di `apps/web` (gate frontend).
- Rust workspace via Docker `rust:1.88` (per README) — host saat ini belum punya `cargo`.
- Migration via `sqlx` (lihat `core-db/migrations.rs`).
- `.github/workflows/ci.yml` + `release-foundation.yml` untuk CI.
- `.opencode/docs/AGENT_ROUTING.md` (lane ownership), `DESIGN.md` (project token, do/don't), `PRD.md` §3.1/§4 (RBAC + open-core), `TRD.md` §13/§17 (auth/RBAC + checkout).

## Constraints

- Tidak boleh memodifikasi PRD/TRD/ERD scope inti tanpa eskalasi (open-core, manual transfer, satu domain = satu marketplace, seller dibuat super admin).
- Migration harus additive. Tidak ada destructive migration di revamp ini.
- Authorization tetap di backend; frontend cuma menyembunyikan UI.
- Demo data tidak boleh masuk migration core; harus script terpisah dengan flag/header `-- DEMO ONLY`.
- Payment proof tetap private (`/media/:file_id`); tidak ada perubahan akses publik.
- Reduced motion dan focus-visible harus dipertahankan (DESIGN.md).
- Stack tidak berubah (Svelte 5 + Tailwind 3 + Axum + sqlx).

## Risiko (singkat)

- Refactor authz literal `super_admin`/`seller` ke matriks 4 role berisiko membuka bug brand isolation kalau audit tidak lengkap.
- Redirect root `/` → `/store` membutuhkan call ke `/api/install/state` di server; jika API belum siap, halaman bisa loop atau blank.
- Menambah role mengubah seed dan test fixture `core-db/commerce.rs` (banyak `setup_test_pool` mengandalkan id role).
- Demo mode toggle bisa men-trigger seed berulang; harus idempotent dan punya guard agar tidak dijalankan di produksi.
- Reset install via UI memperbesar attack surface; harus dibatasi superadmin-only + double-confirm + audit event.
- Dashboard “seperti pada umumnya” riskan scope creep ke analytics berbayar; harus eksplisit hanya operasional MVP.
- AdminShell sidebar saat ini di-grid 260px; menambah user menu/role badge harus mempertahankan responsive dan reduced motion.
