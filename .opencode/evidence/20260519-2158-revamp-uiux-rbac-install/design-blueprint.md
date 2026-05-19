# UI/UX Design Blueprint — market.osource.id Revamp

**Task ID:** `20260519-2158-revamp-uiux-rbac-install`
**Status:** `ready`
**Tanggal:** 2026-05-19

## 0. Prinsip kunci

1. Dashboard admin ikut arah TweakCN style-equivalent — bukan copy asset/kode.
2. Token visual hanya dari `app.css` + `tailwind.config.ts` repo.
3. Admin: operational, calm, dense-moderate, state-forward.
4. Storefront: clothing-first, hangat, image-led.
5. No dark theme toggle palsu. No fake charts. No blank image frame di seeded storefront final.
6. Payment proof tetap private. Storefront order tracking tidak menampilkan raw image public.
7. Reset install UX = password re-entry + checkbox konfirmasi.

## 1. Experience direction

### Persona
- Super Admin/Admin: laptop/desktop 1440, secondary 768.
- Seller/Karyawan: 768, 390.
- Shopper: 390, secondary 768.

### Tone
- Admin: tegas, ringkas, profesional, tidak dekoratif.
- Install: aman, onboarding-first, meyakinkan.
- Storefront: lokal, modern, clothing-friendly.

### Density
- Admin: compact-breathable.
- Storefront: medium, lebih whitespace dan gambar.
- Forms: satu field satu ritme, helper text pendek, error inline.

## 2. Page-by-page UX blueprint

| Route | Tujuan | Action utama | Behavior kunci |
|---|---|---|---|
| `/` | router gate | redirect | pre-install → `/install`; locked → `/store`; error → notice |
| `/install` | first-run setup | buat super admin + seed demo | jika locked, server redirect `/store` |
| `/admin/login` | operator login | login operator | pre-install → `/install`; sudah login → `/admin` |
| `/admin` | dashboard operasional | cek KPI, recent orders, task queue | scoped per role |
| `/admin/settings` | hub pengaturan + lifecycle | kelola marketplace/system/demo/reset | role-filtered tab/panel |
| `/store` | browse clothing storefront | filter, lihat produk, tambah cart | seeded demo brands tampil kuat |
| `/store/cart` | review cart | update qty, lanjut checkout | grouped per brand |
| `/store/checkout` | buat order | submit order | manual transfer, order token setelah submit |
| `/store/orders/[token]` | lacak order | lihat status, upload bukti transfer | payment proof private |

## 3. Section-level visual spec

### 3.1 `/` — root gate
- Bukan landing. Gate route saja.
- Pre-install: hard redirect server ke `/install`.
- Post-lock: hard redirect server ke `/store`.
- Error fallback: full-screen centered panel, max-w-lg, title + CTA `Coba lagi` + CTA sekunder `Buka /install`.

### 3.2 `/install` — first-run panel
- Container `max-w-6xl`, `min-h-screen`, center vertically.
- Desktop: kiri form, kanan `320px` preflight.
- Tablet: form atas, preflight bawah.
- Mobile: 1 kolom, CTA sticky bawah.

**Section order:**
1. Header strip (eyebrow + H1 + session badge)
2. Form: nama marketplace, nama super admin, email, password
3. Demo data toggle (checkbox + 3 brand preview)
4. Submit area + copy "Setup hanya bisa dijalankan sekali"
5. Preflight sidebar (SystemHealthPanel)
6. Locked/success fallback

### 3.3 `/admin/login` — operator login
- Container `max-w-5xl`, center vertically.
- Desktop: kiri info panel (role chips + security note), kanan form.
- Mobile: info strip ringkas di atas form.

**Form:** email + password + CTA `Masuk ke admin` + link `Kembali ke storefront`.
**Error:** inline AsyncState destructive.
**Pre-install:** notice + CTA `/install`.

### 3.4 `/admin` — dashboard (TweakCN style-equivalent)
- Grid `280px / minmax(0,1fr)`.
- Mobile: rail jadi drawer.

**Shell anatomy:**
- Left rail: brand block + grouped nav + footer user card.
- Main: sticky top bar + KPI grid + main content split + secondary panels.

**Top bar:** breadcrumb + H1 + scope caption + search (dashboard only) + light badge + health badge + user menu + role badge.

**KPI grid (4 cards):**
1. Brands aktif
2. Produk published
3. Order open
4. Payment menunggu verifikasi

**Card anatomy:** label uppercase kecil + value tabular-nums besar + helper + chip scope + delta opsional.

**Recent Orders panel:** filter chips + table (Order/Pelanggan/Brand/Total/Payment/Fulfillment/Waktu/Action). Mobile: card list.

**Task Queue panel:** 3 items (payment menunggu verifikasi, order belum diproses, produk stok tipis). Count 0 tetap tampil.

**System Health panel:** Install lock / Database / Storage / Tunnel / Demo data + link Settings.

### 3.5 `/admin/settings` — settings hub
**Sidebar nav baru (5 item):**
- Dashboard
- Brands
- Products
- Orders
- Settings (Packages + System pindah ke sini)

**Settings tabs:**
1. Marketplace
2. Storage
3. Tunnel
4. Package
5. Demo Data
6. Install Reset

**Role visibility:** super_admin semua tab; admin hanya Marketplace; seller/karyawan → AsyncState warning.

**Install Reset panel:** password re-entry + checkbox konfirmasi + button destructive `Reset install`.

### 3.6 `/store` — storefront clothing
**Section order:** hero → brand strip → filter row → product grid → empty/error.

**Hero:** 2 kolom desktop (copy kiri, collage kanan). H1 + CTA primer + CTA sekunder + brand badges.

**Brand strip:** 3 brand cards (Batik Nusantara, Urban Threads, Modest Wear ID). Text-first, no fake logos wajib.

**Filter row:** chips horizontal (Semua/Batik/Casual/Modest/Outerwear/Aksesori) + dropdown Brand/Harga.

**Product grid:** 4 col desktop, 2 col tablet/mobile. Card: image `aspect-[4/5]` + brand label + name + price + stock badge + CTA.

### 3.7 `/store/cart`
- Desktop: kiri list, kanan summary sticky 360px.
- Mobile: stack + sticky checkout bar bawah.
- Item group per brand. Qty stepper 44px. Numbers tabular-nums.

### 3.8 `/store/checkout`
- Desktop: form kiri + summary kanan 400px.
- Mobile: stack.
- Fields: nama, email/telepon, alamat, catatan.
- Payment method panel: info manual transfer.
- Submit → redirect ke `/store/orders/[token]`.

### 3.9 `/store/orders/[token]`
- Desktop: status+items+timeline kiri, payment action kanan.
- Mobile: stack.
- Upload proof: metadata file saja, **tidak ada image preview public**.
- Invalid token: destructive state + CTA.

## 4. Component system plan

### AdminShell anatomy baru
- Left rail: brand block + grouped nav (Overview/Commerce/Control) + footer user card.
- Main: sticky top bar + page intro + content slot.
- Mobile: drawer overlay.
- Footer user card: name + email + role badge + scope + logout.

### StoreShell anatomy baru
- Sticky header: marketplace name + nav (Store/Cart/Checkout) + cart count chip + category quick links.
- Mobile: row 1 brand+cart, row 2 category scroller.

### Primitives baru
- `InputField`: label + input + helper/error. Min-h-11.
- `SubmitButton`: primary/secondary/destructive. Loading state. aria-busy.
- `FormSection`: title + description + content slot.
- `AsyncState`: loading/error/empty/success/warning/locked/unauthenticated/permission.
- `KPICard`: label + value tabular-nums + helper + badge + delta opsional + href.
- `RecentOrdersTable`: desktop table, mobile card list. Row states.
- `TaskQueue`: label + count + helper + CTA. Count 0 tetap tampil.
- `SystemHealthPanel`: label + badge + helper. Read-only.

## 5. Visual system

### Color roles
- Canvas: `bg-background`
- Surface: `bg-surface`
- Primary text: `text-foreground`
- Secondary text: `text-muted-foreground`
- Borders: `border-border`
- Primary action: `primary`
- Semantic: `success`, `warning`, `destructive`

### Typography scale
- H1: `text-3xl sm:text-4xl font-bold tracking-tight`
- H2: `text-xl sm:text-2xl font-semibold`
- KPI value: `text-2xl lg:text-3xl font-bold tabular-nums`
- Body: `text-sm sm:text-base leading-6`
- Eyebrow: `text-xs uppercase tracking-[0.2em] font-semibold`

### Spacing
- Page padding: `px-4 py-6` mobile, `sm:px-6`, `lg:px-8`
- Panel padding: `p-5` default, `p-6` header
- Section gap: `gap-6`
- Card grid gap: `gap-4` mobile, `gap-5` desktop

### Radius + elevation
- Controls: `rounded-xl`
- Panels: `rounded-2xl`
- Shadow: `shadow-panel` only
- Hover: border tint > shadow jump

## 6. Asset decision

| Section | Decision | Notes |
|---|---|---|
| Admin panels | `no-generation-needed` | data-driven, no imagery |
| Install support image | `use-provided-assets` → fallback `generate` | `install-onboarding.png` jika layak |
| Login | `no-generation-needed` | role chips + trust panel cukup |
| Store hero | `use-provided-assets` atau `generate` | fashion/textile still-life, bukan tech art |
| Brand strip | `no-generation-needed` | text-first cards |
| Product grid | `use-provided-assets` prioritas | `demo/product-*.png` reuse; no blank frame final |
| Order tracking | `no-generation-needed` | no payment proof public preview |
| Empty catalog | `use-provided-assets` | `empty-catalog.png` |

**Art direction storefront:** tekstur kain, siluet pakaian, fotografi produk bersih. Netral hangat. No neon, no glossy blobs, no mannequin uncanny.

## 7. Motion system

- CSS transitions + Svelte native saja. No new animation dependency.
- Buttons/pills: `140ms` background/border/color/transform. Hover max `translateY(-1px)`.
- Cards: `180ms` border tint. Image scale max `1.02`.
- Admin drawer mobile: `220ms` translateX + overlay fade `180ms`.
- Async notices: fade `160ms`.
- Reduced motion: hapus transform, sisakan color/border change saja.

## 8. Interaction/state design

Global states: default, hover, focus-visible, active, disabled, loading, empty, error, success, locked, unauthenticated, permission denied, offline/unavailable, validation error.

Key rules:
- Install: form disabled saat loading; inline validation; redirect saat locked.
- Admin login: destructive notice invalid; pre-install warning; redirect jika sudah login.
- Dashboard: skeleton KPI saat loading; empty assignment state untuk karyawan tanpa brand.
- Storefront: skeleton grid saat loading; empty filter state; add-to-cart success notice.
- Order tracking: metadata proof saja, no raw image preview.

## 9. Responsive plan

### 1440
- Admin: sidebar 280px, KPI 4 col, split recent orders/right rail.
- Store: hero 2 col, product grid 4 col, cart/checkout split.

### 768
- Admin: sidebar → drawer, KPI 2x2, panels stack.
- Store: product grid 2 col, cart/checkout stack.

### 390
- Admin: drawer only, KPI 1 col, table → cards.
- Store: header 2 row, product grid 2 col, CTA sticky bottom.

## 10. Accessibility gate

- Focus visible global dipertahankan.
- Contrast: body min 4.5:1, heading/KPI min 3:1.
- All inputs: visible label + aria-describedby untuk helper/error.
- Target size: min-h-11 semua CTA.
- Async notices: role="status" / role="alert".
- Status: selalu kombinasi badge + text label + helper copy.
- Reduced motion: semua transform non-esensial mati.

## 11. Locked decisions untuk @fixer

1. Sidebar admin: Dashboard, Brands, Products, Orders, Settings (5 item).
2. Packages + System pindah ke Settings.
3. Top bar wajib: breadcrumb + title/scope + search (dashboard) + light badge + health badge + user menu + role badge.
4. Product card ratio: `4:5`.
5. Storefront mobile grid: 2 kolom.
6. Upload payment proof hanya di `/store/orders/[token]`, bukan di checkout.
7. Payment proof public preview dilarang.
8. Install reset: password re-entry + checkbox konfirmasi.
9. No dark toggle palsu.
10. No fake sparkline/chart.

## 12. Validation evidence requirements

Screenshots wajib: 1440x1200, 768x1024, 390x844 untuk semua route utama.
State screenshots: install (default/error/locked), login (default/invalid), dashboard (super_admin/seller/empty), settings (full/restricted), store (seeded/empty), cart (filled/empty), checkout (ready/error), order (awaiting/invalid).
Visual parity: reference TweakCN style-equivalent vs final admin dashboard.
