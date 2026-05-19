# Current Capture Requirements

Task ID: `20260511-2156-project-completion-master-plan`

Status: **Validation green; browser capture still blocked**

## Tujuan

Mendokumentasikan baseline visual aktual project sebelum redesign/polish implementation berjalan.

## Minimum Capture Matrix

- Desktop:
  - `/store`
  - `/store/cart`
  - `/store/checkout`
  - `/admin`
  - `/admin/brands`
  - `/admin/products`
  - `/admin/packages`
  - `/admin/system`
  - `/install`
- Mobile:
  - `/store`
  - `/store/cart`
  - `/store/checkout`
  - `/install`
- Desktop app / control panel:
  - runtime status home

## Capture Notes Template

- viewport:
- URL/route:
- wait strategy:
- scroll pass done:
- screenshot path:
- console errors impacting render:
- network errors impacting render:
- limitations:

## Current Execution Notes

- Web preview server berhasil dibangun dan dapat dijalankan lokal (`npm run preview -- --host 127.0.0.1 --port 4173`).
- Backend local service juga dapat dijalankan untuk pengujian API/UI milestone saat ini.
- Validation rerun after payment-proof permission + fulfillment updates passed:
  - `cargo test -p core-api -p core-db` → passed (`45 passed, 8 ignored` after fulfillment validation hardening tests)
  - `npm run check` (from `apps/web`) → passed (`0 errors, 0 warnings`)
  - `npm run build` (from `apps/web`) → passed
- Hardening follow-up applied after review:
  - server now rejects non-canonical `fulfillment_status` values,
  - admin orders UI now renders payment-proof actions from backend permission flags instead of exposing blind 403-prone actions,
  - seller payment-proof response flags now resolve from actual brand-scoped permissions instead of fallback `false` placeholders.
- Attempted browser capture via Playwright MCP gagal karena runtime Chrome/Chromium belum tersedia pada environment ini.

### Attempted baseline capture blocker

- viewport: `1440x1200` planned first
- URL/route: `http://127.0.0.1:4173/store`
- wait strategy: planned `networkidle` + settle
- scroll pass done: no
- screenshot path: none
- console errors impacting render: capture session could not start
- network errors impacting render: none observed because browser failed before navigation
- limitations: `Chromium distribution 'chrome' is not found at /opt/google/chrome/chrome`

### Remediation attempt

- Tried `npx playwright install chrome`
- Install failed because the environment attempted privileged escalation (`sudo`) and no password was available.

## Current Readiness Assessment

- Functional validation for the current milestone slice is now **green**.
- UI surfaces are ready for baseline capture in principle:
  - `/store`
  - `/store/cart`
  - `/store/checkout`
  - `/store/orders/[trackingToken]`
  - `/admin`
  - `/admin/login`
  - `/admin/brands`
  - `/admin/products`
  - `/admin/orders`
  - `/admin/system`
  - `/install`
- Browser-evidence completion remains **blocked by local browser runtime availability**, not by route readiness.
