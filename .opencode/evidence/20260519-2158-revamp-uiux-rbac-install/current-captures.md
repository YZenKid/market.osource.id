# Current Captures — Revamp UI/UX

Status: `partial`.

## Browser evidence yang tersedia

- Playwright smoke desktop terhadap SSR lokal (`http://127.0.0.1:4173`) lulus `22/22` test setelah perbaikan product listing mobile-empty-state.
- SSR lokal sekarang auto-start dari `apps/web/playwright.config.ts` via `webServer` dengan:
  - `PUBLIC_API_BASE_URL=http://127.0.0.1:7301`
  - `ORIGIN=http://127.0.0.1:4173`
  - `PORT=4173`
- Route yang tervalidasi browser-driven pada project `desktop`:
  - `/`
  - `/install`
  - `/admin/login`
  - `/admin`
  - `/admin/settings`
  - `/store`
  - `/store/cart`
  - `/store/checkout`
  - `/store/orders/invalid-token-smoke-test`

## Viewport target

- Desktop: `1440x1200`
- Tablet: `768x1024`
- Mobile: `390x844`

## Notes capture/evidence

- Desktop smoke dijalankan penuh via Playwright test runner, bukan Browser MCP.
- Browser MCP tetap tidak tersedia: `ECONNREFUSED 127.0.0.1:9333`.
- Screenshot current/final per viewport belum tersimpan sebagai artifact terpisah.
- Tablet + mobile project Playwright berbasis WebKit masih terblokir dependency host OS (`libicudata.so.66`, `libjpeg.so.8`, `libwebp.so.6`, dll).

## Current blocker

- Artifact screenshot `1440x1200`, `768x1024`, `390x844` belum ada.
- Visual claim final tetap harus dibatasi ke `style-equivalent` dengan evidence gap screenshot lintas viewport.
