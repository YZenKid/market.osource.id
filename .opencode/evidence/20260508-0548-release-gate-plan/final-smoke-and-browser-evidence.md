# Final Smoke and Browser Evidence

Task ID: `20260508-0548-release-gate-plan`

## Scope

This evidence records the final environment-facing checks performed after the bounded Gate B–J implementation slices and the later split-origin published-port move to the 83xx range:

- backend Docker build smoke,
- Docker Compose smoke for backend/web/postgres,
- fresh runtime migration bootstrap behavior,
- browser runtime availability for Playwright screenshot evidence.

## Commands and Results

| Command | Result | Notes |
| --- | --- | --- |
| `docker build --tag market-osource-smoke:local .` | Passed | Backend image now builds successfully after Docker workspace fix. |
| `docker compose up --build -d` | Passed | Fresh compose stack can start after web immutable-build fix and backend Docker fix. |
| `curl http://127.0.0.1:8301/health` | Passed | Returned `{"status":"healthy"}` on the current published backend port. |
| `curl http://127.0.0.1:8301/ready` | Partial / expected fresh-install state | Database, migrations, storage, and package compatibility are ready. `install_state` remains `not_ready` before setup/lock on a fresh install. |
| `docker compose exec -T postgres psql -U market -d market_osource -c "\\dt"` | Passed | Core tables, package tables, permission tables, and audit/payment/order tables exist after startup migration bootstrap. |
| `python -c "urllib.request.urlopen('http://127.0.0.1:8300/')..."` | Passed | Storefront/admin shell homepage responds and contains `market.osource.id` on the current published web port. |
| Split-origin install/admin/storefront API smoke from `8300` to `8301` | Passed | CSRF token, setup session cookie, `auth/me`, admin reads, runtime read, and storefront reads all reached backend successfully across explicit credentialed CORS. |
| `playwright_browser_navigate` to `http://127.0.0.1:8300/` | Blocked by environment | Browser runtime requires Chrome/Chromium at `/opt/google/chrome/chrome`. |
| `npx playwright install chrome` | Blocked by environment | Attempt required privileged install path / interactive sudo password and could not complete. |

## Key Observations

### Deploy smoke status

- Backend image build is no longer blocked by workspace member resolution.
- Web compose service is no longer blocked by `npm ci` on a read-only source mount.
- Backend startup now applies core/package migrations when DB is reachable.
- Published Docker host ports now use the lower-collision 83xx range: web `8300`, backend `8301`.
- Fresh runtime `/ready` correctly remains `not_ready` until install setup is completed; this is valid for a fresh install panel state and not a migration failure.

### Split-origin smoke status

- `GET /api/auth/csrf` from origin `http://127.0.0.1:8300` returned a CSRF token, `market_csrf` cookie, `Access-Control-Allow-Origin: http://127.0.0.1:8300`, and `Access-Control-Allow-Credentials: true`.
- `POST /api/install/setup` with CSRF succeeded and returned `{"state":"locked","locked":true,"authenticated":true}` with `market_session` set.
- `GET /api/auth/me`, `GET /api/admin/brands`, `GET /api/admin/products`, `GET /api/admin/packages`, `GET /api/system/runtime`, and `GET /api/storefront/products` all succeeded through the split-origin path after setup.
- `POST /api/storefront/checkout` with a valid payload shape and empty `items` reached backend successfully and returned the expected business validation error `checkout requires at least one item`, confirming error responses also preserve the required CORS headers.

### Browser evidence blocker

Playwright screenshot/browser evidence is still blocked by environment constraints:

- no `google-chrome`, `chromium`, or `chromium-browser` binary is installed,
- Playwright browser MCP expects Chrome at `/opt/google/chrome/chrome`,
- local install attempt via `npx playwright install chrome` required privileged system changes and failed because sudo/password interaction is unavailable in this environment.

This means visual screenshot evidence cannot be produced from the current environment without:

1. a preinstalled Chrome/Chromium runtime, or
2. explicit privileged install capability for Playwright browser runtime.

## Remaining Environment Blocker

The primary remaining blocker is now **environmental browser runtime availability for visual evidence**, not core application implementation.
