# Current Captures — Revamp UI/UX

Status: `complete` — screenshots captured, Playwright 66/66 across 3 viewport projects.

## Browser evidence yang tersedia

### Playwright smoke — 66/66 pass

- Projects: `desktop` (1440x900), `tablet-chromium` (768x1024), `mobile-chromium` (390x844)
- Backend: evidence stack `marketevidence` port 7401 (locked, demo seeded)
- SSR: local node build port 4174 with `PUBLIC_API_BASE_URL=http://127.0.0.1:7401`
- Command: `PLAYWRIGHT_PORT=4174 PLAYWRIGHT_ORIGIN=http://127.0.0.1:4174 PLAYWRIGHT_BASE_URL=http://127.0.0.1:4174 PLAYWRIGHT_PUBLIC_API_BASE_URL=http://127.0.0.1:7401 npx playwright test tests/smoke/routes.spec.ts`
- Result: `66 passed (6.3s)` — 0 failures across all 3 projects

Note: tablet/mobile projects use Chromium emulation (not WebKit). WebKit system deps unavailable on host. Chromium emulation validates layout/responsive behavior but is not Safari parity.

### Screenshots captured

18 screenshots across 6 routes × 3 viewports (desktop 1440x1200, tablet 768x1024, mobile 390x844):

| Route | Desktop | Tablet | Mobile |
| --- | --- | --- | --- |
| `/store` | screenshots/store-desktop.png | screenshots/store-tablet.png | screenshots/store-mobile.png |
| `/admin/login` | screenshots/admin-login-desktop.png | screenshots/admin-login-tablet.png | screenshots/admin-login-mobile.png |
| `/admin` | screenshots/admin-desktop.png | screenshots/admin-tablet.png | screenshots/admin-mobile.png |
| `/admin/settings` | screenshots/admin-settings-desktop.png | screenshots/admin-settings-tablet.png | screenshots/admin-settings-mobile.png |
| `/admin/settings/demo` | screenshots/admin-settings-demo-desktop.png | screenshots/admin-settings-demo-tablet.png | screenshots/admin-settings-demo-mobile.png |
| `/admin/settings/install` | screenshots/admin-settings-install-desktop.png | screenshots/admin-settings-install-tablet.png | screenshots/admin-settings-install-mobile.png |

Admin screenshots captured with authenticated super_admin session (evidence stack cookies).

## Remaining gap

- `/install` pre-install screenshot not captured (evidence stack already locked).
- `/store/cart`, `/store/checkout`, `/store/orders/[token]` screenshots not captured (covered by Playwright smoke pass).
- WebKit/Safari parity not validated — Chromium emulation only.
