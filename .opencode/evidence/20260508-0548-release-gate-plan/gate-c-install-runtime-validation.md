# Gate C Install/Runtime Validation Evidence

Task ID: `20260508-0548-release-gate-plan`

## Scope Implemented in This Batch

- Added DB-backed install state repository helpers in `core-db`.
- Added first setup bootstrap helper that creates:
  - first Super Admin user with Argon2id hash from API layer,
  - marketplace settings,
  - local storage settings,
  - default disabled `cloudflared` tunnel settings,
  - locked `installation_state` row.
- Added install lock protection: repeated setup returns `AlreadyLocked`.
- Added a transaction-scoped PostgreSQL advisory lock around bootstrap setup so concurrent first setup attempts on an empty database serialize safely.
- Added setup validation in `core-installer` for marketplace name, admin name, email shape, and 12-character minimum password.
- Replaced placeholder `/api/install/state` and `/api/install/preflight` with state-aware responses.
- Added `/api/install/setup` mutation endpoint.
- Updated install page to fetch state/preflight and submit setup against Axum `/api/install/setup`.
- Replaced `/api/system/runtime` install/tunnel placeholders with read-only DB-backed install state and shared sidecar status helpers.
- Added shared default-off PostgreSQL/cloudflared sidecar status helpers for backend and desktop shell visibility.
- Added `market-osource-web.service` systemd unit for the separate SvelteKit service and updated VPS deployment notes.
- Added bounded setup-session completion: successful `POST /api/install/setup` now creates a server-side session for the newly inserted Super Admin and returns the same `market_session` cookie format used by login.
- Extended setup response with safe `authenticated: true` signal and updated install page success copy to tell the operator the Super Admin session is active.
- Kept install setup CSRF validation in place; this slice did not add runtime process supervisor behavior.

## Validation Commands

| Command | Result | Notes |
| --- | --- | --- |
| `cargo fmt --check` | Passed | Rust formatting checked after auto-format. |
| `cargo clippy --workspace --exclude market-desktop --all-targets -- -D warnings` | Passed | Non-desktop Rust workspace linted locally. |
| `cargo test --workspace --exclude market-desktop` | Passed | Includes installer setup validation tests. |
| PostgreSQL 16 container + `TEST_DATABASE_URL=... cargo test -p core-db bootstrap -- --ignored --test-threads=1` | Passed | Confirms first setup succeeds, repeat setup is rejected, and concurrent first setup attempts produce exactly one Super Admin. |
| `npm run check` in `apps/web` | Passed | Svelte install page has no type/check diagnostics. |
| `cargo fmt --check` after runtime status slice | Passed | Formatting remained clean. |
| `cargo clippy --workspace --exclude market-desktop --all-targets -- -D warnings` after runtime status slice | Passed | Sidecar/core-api runtime status code linted cleanly. |
| `cargo test --workspace --exclude market-desktop` after runtime status slice | Passed | 7 passed, 5 ignored across 23 suites. |
| `npm run check` in `apps/web` after runtime status slice | Passed | Svelte checks still clean. |
| `cargo check -p market-desktop` on host | Blocked by host deps | Fails before project code on missing system `dbus-1` pkg-config metadata; host also previously lacked WebKit/rsvg Tauri deps. Prior Fedora/container desktop validation remains the available compile evidence. |
| `cargo fmt --check` after setup-session slice | Passed | Rust formatting checked for setup/session changes. |
| `cargo test -p core-api` after setup-session slice | Passed | 16 passed across unit/doc tests; includes session cookie helper and install setup response/CSRF tests. |
| `cargo clippy -p core-api -p core-db --all-targets -- -D warnings` after setup-session slice | Passed | Touched Rust crates lint clean after removing stale constant assertions in router tests. |
| `npm run check` in `apps/web` after setup-session slice | Passed | Svelte install page type/check diagnostics clean. |
| `cargo test -p core-db install::tests::bootstrap_installation_locks_repeat_setup -- --ignored` without `TEST_DATABASE_URL` | Blocked by local env | Ignored DB integration test requires `TEST_DATABASE_URL` and panicked when unset; previous PostgreSQL container evidence above remains the latest DB-backed run. |

## Known Gaps

- This is the first Gate C batch, not the entire Gate C.
- Setup endpoint now issues a `market_session` cookie after successful first setup; this behavior was later revalidated through the split-origin runtime path (`8300` -> `8301`) with `authenticated: true` returned by setup and `GET /api/auth/me` succeeding afterward.
- Install mutation has CSRF validation and single-process rate limiting; distributed/proxy-aware limiting remains future work.
- Desktop process supervisor is not implemented yet; only prior desktop compile validation exists.
- Runtime status is read-only; it does not start/stop PostgreSQL, backend, or `cloudflared` processes yet.
- Deeper VPS smoke tests remain for later Gate C/J work.

## Later Runtime Smoke Addendum

The install/runtime slice was later smoke-tested again after the split-origin published-port move to `8300/8301`.

Confirmed later behavior:

- `GET /api/install/state` from origin `http://127.0.0.1:8300` succeeded.
- `GET /api/install/preflight` from the same origin succeeded.
- `GET /api/auth/csrf` returned a token plus `market_csrf` cookie with `Access-Control-Allow-Origin: http://127.0.0.1:8300` and `Access-Control-Allow-Credentials: true`.
- `POST /api/install/setup` with the required payload (`marketplace_name`, `admin_name`, `admin_email`, `admin_password`) succeeded and returned:

```json
{"state":"locked","locked":true,"authenticated":true}
```

- `GET /api/auth/me` then succeeded using the setup-created `market_session` cookie.

## Security Notes

- Password hashing uses `core-auth` Argon2id helper before storing the first Super Admin.
- Install state is locked server-side; UI disabled state is not relied on for enforcement.
- Fresh setup is protected by a transaction-scoped PostgreSQL advisory lock before lock-state check and first admin creation.
- Tunnel defaults to disabled in `tunnel_settings` during setup.
- Shared tunnel status helper forces disabled tunnels to report `running=false` and hides `public_url`.
- Desktop/runtime status keeps PostgreSQL bind posture at `127.0.0.1` for the current read-only slice.
- Repeated setup attempts are rejected before creating additional admin users.
- Setup API logs internal DB errors server-side and returns a sanitized failure message to the client.
- Setup-session creation uses the existing login session cookie builder semantics (`HttpOnly`, `SameSite=Lax`, `Path=/`, same max age and secure flag behavior) and stores only the hashed server-side token in `sessions`.
- If session creation fails after bootstrap, the API returns a sanitized recovery message instructing login with the Super Admin account rather than exposing internals.
