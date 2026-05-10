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

## Validation Commands

| Command | Result | Notes |
| --- | --- | --- |
| `cargo fmt --check` | Passed | Rust formatting checked after auto-format. |
| `cargo clippy --workspace --exclude market-desktop --all-targets -- -D warnings` | Passed | Non-desktop Rust workspace linted locally. |
| `cargo test --workspace --exclude market-desktop` | Passed | Includes installer setup validation tests. |
| PostgreSQL 16 container + `TEST_DATABASE_URL=... cargo test -p core-db bootstrap -- --ignored --test-threads=1` | Passed | Confirms first setup succeeds, repeat setup is rejected, and concurrent first setup attempts produce exactly one Super Admin. |
| `npm run check` in `apps/web` | Passed | Svelte install page has no type/check diagnostics. |

## Known Gaps

- This is the first Gate C batch, not the entire Gate C.
- Setup endpoint does not yet issue a login session cookie after setup; full session/auth work belongs to Gate D.
- Install mutation currently has no CSRF/rate limit because Gate D owns CSRF/session/rate-limit middleware.
- Desktop process supervisor is not implemented yet; only prior desktop compile validation exists.
- Systemd web unit and deeper VPS smoke tests remain for later Gate C/J work.

## Security Notes

- Password hashing uses `core-auth` Argon2id helper before storing the first Super Admin.
- Install state is locked server-side; UI disabled state is not relied on for enforcement.
- Fresh setup is protected by a transaction-scoped PostgreSQL advisory lock before lock-state check and first admin creation.
- Tunnel defaults to disabled in `tunnel_settings` during setup.
- Repeated setup attempts are rejected before creating additional admin users.
- Setup API logs internal DB errors server-side and returns a sanitized failure message to the client.
