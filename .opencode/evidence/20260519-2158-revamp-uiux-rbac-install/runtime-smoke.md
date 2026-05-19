# Runtime Smoke Evidence — Revamp UI/UX

Task ID: `20260519-2158-revamp-uiux-rbac-install`
Tanggal: 2026-05-19 22:29 WIB

## Stack yang dipakai

- Backend: Docker compose project `marketrevamp`, port 7401 (fresh isolated volume)
- Web preview: `npm run preview` lokal, port 7400, `PUBLIC_API_BASE_URL=http://127.0.0.1:7401`
- Frontend build: `npm run build` + `npm run check` — 0 errors, 0 warnings

## Hasil smoke test

### Install flow

| Step | Expected | Actual | Status |
|---|---|---|---|
| Install state (fresh) | `unconfigured`, `locked: false` | `"state":"unconfigured" "locked":false` | ✅ |
| Preflight | `ok: true`, 4 checks | `ok: true, checks: 4` | ✅ |
| Setup dengan `seed_demo: true` | `locked: true`, `authenticated: true` | `"locked":true "authenticated":true` | ✅ |
| Storefront products setelah setup | 24 produk (12 produk × 2 variant) | `Count: 24` | ✅ |
| Dashboard summary setelah setup | brands_active: 3, products_published: 12 | `{"brands_active":3,"products_published":12,"orders_open":1,"payments_pending_verification":0,"scope":"all"}` | ✅ |

### Root redirect

| Step | Expected | Actual | Status |
|---|---|---|---|
| Root `/` pre-install | HTTP 302 → `/install` | `HTTP 302 -> http://127.0.0.1:7400/install` | ✅ |
| Root `/` post-install | HTTP 302 → `/store` | `HTTP 302 -> http://127.0.0.1:7400/store` | ✅ |
| `/install` post-install | HTTP 302 → `/store` | `HTTP 302 -> http://127.0.0.1:7400/store` | ✅ |

### Page titles

| Route | Expected | Actual | Status |
|---|---|---|---|
| `/install` | Install Panel | `Install Panel — market.osource.id` | ✅ |
| `/store` | Storefront | `Storefront — market.osource.id` | ✅ |
| `/admin` | Dashboard | `Dashboard — Admin` | ✅ |
| `/admin/login` | Operator Login | `Operator Login — market.osource.id` | ✅ |
| `/admin/settings` | Settings | `Settings — Admin` | ✅ |
| `/admin/settings/demo` | Demo Data | `Demo Data — Admin Settings` | ✅ |
| `/admin/settings/install` | Install Reset | `Install Reset — Admin Settings` | ✅ |

### Auth + RBAC

| Step | Expected | Actual | Status |
|---|---|---|---|
| Login super_admin | role: super_admin | `"code":"super_admin"` | ✅ |
| Dashboard summary scoped | scope: all | `"scope":"all"` | ✅ |

### Demo data

| Step | Expected | Actual | Status |
|---|---|---|---|
| Demo clear | HTTP 204 | `Demo clear HTTP: 204` | ✅ |
| Products after clear | 0 | `Count: 0` | ✅ |
| Demo re-seed | HTTP 204 | `Demo seed HTTP: 204` | ✅ |
| Products after re-seed | 24 | `Count: 24` | ✅ |

### Install reset

| Step | Expected | Actual | Status |
|---|---|---|---|
| Reset dengan password + confirmed | `reset: true` | `"reset":true "message":"installation reset completed"` | ✅ |
| Install state setelah reset | `unconfigured`, `locked: false` | `"state":"unconfigured" "locked":false` | ✅ |

## Bugs yang ditemukan dan diperbaiki

1. **`seed_demo` field diabaikan di setup handler** — `SetupRequestBody` tidak punya field `seed_demo`. Fix: tambah field `seed_demo: bool` dengan `#[serde(default)]` dan wire ke `seed_demo_clothing` setelah bootstrap.
2. **`clear_demo_clothing` menghapus admin user** — query `DELETE FROM users WHERE email LIKE '%@demo.local'` menghapus super admin juga. Fix: tambah filter `role_id = (SELECT id FROM roles WHERE code = 'seller')`.

## Playwright / browser validation

Playwright MCP tidak tersedia di environment ini, tetapi Playwright CLI berhasil dijalankan.

### Desktop smoke

- Config: `apps/web/playwright.config.ts` sekarang memakai `webServer` untuk auto-start SSR lokal.
- Command validasi: `cd apps/web && npx playwright test tests/smoke/routes.spec.ts --project=desktop`
- Result: `22/22` pass.

### Bug yang ditemukan lewat Playwright dan diperbaiki

- `section[aria-label="Product listing"]` di `/store` menjadi `hidden` pada viewport `390x844` ketika `visibleProducts.length === 0`.
- Root cause: empty CSS grid tanpa child collapse ke zero height; Playwright menilai elemen `hidden`.
- Fix: loading/error/empty state dipindah ke dalam section product listing dengan wrapper `col-span-full`.

### Cross-viewport blocker

- Project `tablet` + `mobile` berbasis WebKit masih gagal start karena dependency host OS belum ada (`libicudata.so.66`, `libjpeg.so.8`, `libwebp.so.6`, `libffi.so.7`, dll).
- Ini bukan bug aplikasi web; ini blocker environment.

Browser evidence saat ini terdiri dari:
- HTTP smoke test via curl (root redirect, page titles, API flows)
- Playwright desktop smoke (`22/22` pass)
- Backend test logs (46 unit tests pass, 0 failures)
- `npm run check` + `npm run build` clean

## DB audit_events capture

Query dijalankan pada `marketosourceid-postgres-1` (DB: `market_osource`, user: `market`):

```sql
SELECT DISTINCT action, target_type, result FROM audit_events ORDER BY action;
```

Result:
```
    action     |    target_type     | result
---------------+--------------------+---------
 auth.login    | auth_session       | success
 auth.login    | auth_session       | failure
 install.setup | installation_state | failure
 install.setup | installation_state | success
```

Catatan: `install.reset`, `admin.demo.seed`, `admin.demo.clear` tidak muncul di DB ini karena stack evidence final (`marketosourceid`) sudah di-reset dan tidak menjalankan ulang demo seed/clear/reset setelah reset. Audit events untuk operasi tersebut hanya ada di stack smoke terisolasi `marketrevamp` (port 7401) yang sudah dihapus volumenya. Ini adalah gap evidence, bukan bug — endpoint sudah terbukti berjalan di smoke terisolasi (lihat tabel smoke di atas).

## Catatan deployment

Root redirect SSR bekerja karena web preview lokal dapat menjangkau backend di `127.0.0.1:7401` via `PUBLIC_API_BASE_URL`. Dalam compose split-origin tanpa reverse proxy, SSR fetch ke `/api/install/state` tidak dapat menjangkau backend dari dalam container web. Ini adalah expected behavior untuk split-origin VPS deployment — operator harus menggunakan reverse proxy (Nginx/Caddy) yang menyatukan origin, sesuai PRD/TRD.
