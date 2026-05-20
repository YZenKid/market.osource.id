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

### Full suite — 66/66 pass

- Projects: `desktop` (1440x900), `tablet-chromium` (768x1024 Chromium emulation), `mobile-chromium` (390x844 Chromium emulation)
- Backend: evidence stack `marketevidence` port 7401
- Command: `PLAYWRIGHT_PORT=4174 PLAYWRIGHT_ORIGIN=http://127.0.0.1:4174 PLAYWRIGHT_BASE_URL=http://127.0.0.1:4174 PLAYWRIGHT_PUBLIC_API_BASE_URL=http://127.0.0.1:7401 npx playwright test tests/smoke/routes.spec.ts`
- Result: `66 passed (6.3s)` — 0 failures

### Bugs ditemukan dan diperbaiki via Playwright

1. `section[aria-label="Product listing"]` di `/store` menjadi `hidden` pada viewport `390x844` ketika `visibleProducts.length === 0`. Root cause: empty CSS grid collapse ke zero height. Fix: loading/error/empty state dipindah ke dalam section dengan `col-span-full`.
2. Test `/store has accessible navigation` salah asumsi desktop nav visible di mobile. Fix: test diperbarui untuk menerima desktop nav ATAU mobile cart button ATAU category rail.

### Cross-viewport note

- Tablet + mobile projects menggunakan Chromium emulation, bukan WebKit.
- WebKit system deps tidak tersedia di host (`libicudata.so.66`, `libjpeg.so.8`, `libwebp.so.6`, dll).
- Ini bukan bug aplikasi; ini environment constraint.

Browser evidence terdiri dari:
- HTTP smoke test via curl (root redirect, page titles, API flows)
- Playwright 66/66 pass (desktop + tablet-chromium + mobile-chromium)
- Backend test logs (57 unit tests pass, 8 ignored, 0 failures)
- `npm run check` + `npm run build` clean

## DB audit_events capture

Capture final dilakukan pada stack evidence terisolasi `marketevidence` (DB: `market_osource`, user: `market`, backend: `http://127.0.0.1:7401`).

### Setelah setup + demo seed

```sql
SELECT action, target_type, result, metadata, created_at FROM audit_events ORDER BY created_at DESC LIMIT 10;
```

Result:
```
    action     |    target_type     | result  |                                          metadata                                           |          created_at
---------------+--------------------+---------+---------------------------------------------------------------------------------------------+-------------------------------
 demo.seed     | demo_data          | success | {"seeded": true}                                                                            | 2026-05-20 00:34:08.810327+00
 install.setup | installation_state | success | {"locked": true, "demo_seeded": true, "session_created": true, "demo_seed_requested": true} | 2026-05-20 00:33:58.972785+00
```

### Setelah demo clear

Result:
```
    action     |    target_type     | result  |                                          metadata                                           |          created_at
---------------+--------------------+---------+---------------------------------------------------------------------------------------------+-------------------------------
 demo.clear    | demo_data          | success | {"cleared": true}                                                                           | 2026-05-20 00:34:17.648879+00
 demo.seed     | demo_data          | success | {"seeded": true}                                                                            | 2026-05-20 00:34:08.810327+00
 install.setup | installation_state | success | {"locked": true, "demo_seeded": true, "session_created": true, "demo_seed_requested": true} | 2026-05-20 00:33:58.972785+00
```

### Setelah install reset

Backend bug diperbaiki: success audit reset kini ditulis **setelah** reset dengan `actor_user_id = NULL` dan metadata non-PII (`prior_actor_user_id`, `prior_actor_role_code`), sehingga tetap persisten walau tabel `users` dan `audit_events` dibersihkan saat reset.

Result:
```
    action     |    target_type     | result  |                                                          metadata                                                          |          created_at
---------------+--------------------+---------+----------------------------------------------------------------------------------------------------------------------------+-------------------------------
 install.reset | installation_state | success | {"confirmed": true, "prior_actor_user_id": "6632465b-11a4-4a20-bc05-3169f3acca96", "prior_actor_role_code": "super_admin"} | 2026-05-20 01:38:58.112394+00
```

### Verify install state after reset

```json
{"state":"unconfigured","locked":false,"installed":false,"runtime_mode":null,"core_version":null,"database_connected":true}
```

## Catatan deployment

Root redirect SSR bekerja karena web preview lokal dapat menjangkau backend di `127.0.0.1:7401` via `PUBLIC_API_BASE_URL`. Dalam compose split-origin tanpa reverse proxy, SSR fetch ke `/api/install/state` tidak dapat menjangkau backend dari dalam container web. Ini adalah expected behavior untuk split-origin VPS deployment — operator harus menggunakan reverse proxy (Nginx/Caddy) yang menyatukan origin, sesuai PRD/TRD.
