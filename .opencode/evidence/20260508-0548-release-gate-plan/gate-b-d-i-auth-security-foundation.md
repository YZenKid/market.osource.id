# Gate B/D/I Auth + Security Foundation Evidence

Task ID: `20260508-0548-release-gate-plan`

## Scope Implemented in This Slice

This evidence records a bounded implementation slice toward Gate B, Gate D, and Gate I. It does **not** claim Gate B–J are complete.

Implemented:

- API router hardening:
  - credentialed CORS uses explicit `CORS_ALLOWED_ORIGINS` allowlist;
  - wildcard `*` is ignored when credentials are enabled;
  - `x-csrf-token` is allowed for browser mutations;
  - global request body limit is set to 2 MiB;
  - response headers include `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, and `Referrer-Policy: same-origin`.
- Session/auth foundation:
  - `POST /api/auth/login` verifies Argon2id password hashes and creates server-side sessions;
  - session cookie is named `market_session`, `HttpOnly`, `SameSite=Lax`, `Path=/`, with `Secure` controlled by runtime config;
  - raw session tokens are only returned as cookies; database stores SHA-256 hex session hashes;
  - `GET /api/auth/me` resolves current active, non-expired, non-revoked sessions;
  - `POST /api/auth/logout` revokes the current session by hash and clears the cookie.
- CSRF foundation:
  - `GET /api/auth/csrf` issues a URL-safe CSRF token and sets `market_csrf` cookie;
  - CSRF cookie is intentionally readable by frontend JavaScript, `SameSite=Lax`, `Path=/`, with `Secure` controlled by runtime config;
  - `POST /api/auth/login`, `POST /api/auth/logout`, and `POST /api/install/setup` require matching `market_csrf` cookie and `X-CSRF-Token` header.
- Install UI integration:
  - the SvelteKit install panel fetches `/api/auth/csrf` and sends `x-csrf-token` for setup.
- Runtime endpoint minimization:
  - public `/api/system/runtime` no longer exposes raw local storage path; it reports storage as configured/local only.

Implemented in the next bounded Gate D/I blocker slice:

- Single-process MVP rate limiting foundation:
  - added `core_auth::InMemoryRateLimiter` with fixed-window counters;
  - wired it through shared `AppState`;
  - applied it to `POST /api/auth/login` keyed by best-effort client identity plus normalized email;
  - applied it to `POST /api/install/setup` keyed by best-effort client identity;
  - rate-limited requests return HTTP `429` with generic `rate_limited` messaging;
  - TODOs remain in code for distributed, proxy-aware limiter work.
- Audit writer primitive:
  - added `core_db::write_audit_event` for `audit_events`;
  - added safe metadata construction/validation that rejects obvious sensitive keys like password, token, email, phone, address, and secret;
  - records login success/failure/rate-limited attempts, logout success, and install setup success/failure where a DB pool is available;
  - avoids logging plaintext credentials, raw session tokens, CSRF tokens, and email addresses in audit metadata.
- Reusable API auth helper:
  - added `AuthenticatedActor` extractor/helper resolving `market_session` cookie to current session;
  - added `require_super_admin` helper returning `401` for unauthenticated and `403` for non-super-admin actors.
- System endpoint authorization:
  - `GET /api/system/runtime` now requires a Super Admin session;
  - `GET /api/system/tunnel` now requires a Super Admin session;
  - `/health`, `/ready`, and `/version` remain public.

Implemented in this bounded throttle identity hardening slice:

- Client identity derivation for throttling no longer trusts forwarding headers blindly.
- Backend now serves Axum with `into_make_service_with_connect_info::<SocketAddr>()` so handlers receive peer socket identity.
- Rate-limit key derivation now uses peer `ConnectInfo<SocketAddr>` as primary identity signal.
- `x-forwarded-for` / `x-real-ip` are only consulted when the immediate peer is loopback (`127.0.0.1`/`::1`), i.e. bounded local proxy assumption.
- Forwarding header values are parsed as valid `IpAddr`; invalid/spoofed values are ignored with fallback to peer IP.
- Route coverage updated for throttled endpoints:
  - `POST /api/auth/login`
  - `POST /api/install/setup`
  - `POST /api/storefront/checkout`
  - `GET /api/storefront/orders/{tracking_token}`
  - `POST /api/storefront/orders/{tracking_token}/payment-proof`
- Unit coverage added for non-loopback header ignore, loopback header allow, and invalid header fallback behavior.

## Files Changed

- `Cargo.lock`
- `apps/web/src/routes/(install)/install/+page.svelte`
- `crates/core-api/Cargo.toml`
- `crates/core-api/src/authz.rs`
- `crates/core-api/src/router.rs`
- `crates/core-api/src/routes/auth.rs`
- `crates/core-api/src/routes/install.rs`
- `crates/core-api/src/routes/mod.rs`
- `crates/core-api/src/routes/system.rs`
- `crates/core-auth/Cargo.toml`
- `crates/core-auth/src/csrf.rs`
- `crates/core-auth/src/lib.rs`
- `crates/core-auth/src/rate_limit.rs`
- `crates/core-auth/src/session.rs`
- `crates/core-app/src/state.rs`
- `crates/core-db/src/audit.rs`
- `crates/core-db/src/auth.rs`
- `crates/core-db/src/lib.rs`
- `crates/core-runtime/src/config.rs`

## Validation Commands

| Command | Result | Notes |
| --- | --- | --- |
| `cargo fmt --check` | Passed | Rust formatting is clean. |
| `cargo test -p core-auth -p core-api -p core-db` | Passed | 26 passed, 6 ignored; ignored tests require disposable PostgreSQL via `TEST_DATABASE_URL`. |
| `npm --prefix apps/web run check` | Passed | `svelte-check found 0 errors and 0 warnings`. |

Focused validation for the next bounded Gate D/I blocker slice:

- `cargo fmt --check && cargo test -p core-auth -p core-db -p core-api` — Passed, 26 passed and 6 ignored.

Earlier focused validation during implementation also passed:

- `cargo test -p core-auth -p core-api` after CSRF addition: 17 passed.
- `cargo check -p core-api` after router/auth hardening.

Focused validation for throttle identity hardening slice:

- `cargo test -p core-api` — Passed (33 passed).
- `cargo test -p core-api client_identity -- --nocapture` — Passed (4 passed, 29 filtered).

## Remaining Gate Blockers

Gate B remaining blockers:

- Live PostgreSQL migration tests are still not wired into CI by default.
- Container/systemd smoke tests are not yet automated as release evidence.
- Remote CI result is not recorded in this evidence.

Gate C remaining blockers:

- Desktop process supervisor, bundled PostgreSQL lifecycle, and `cloudflared` lifecycle remain incomplete.
- Setup-session creation after first-admin bootstrap is no longer a blocker; successful `POST /api/install/setup` now creates an authenticated `market_session` and this was later revalidated through split-origin runtime smoke.
- Install setup now has single-process MVP rate limiting; distributed/proxy-aware limiter remains later work.

Gate D remaining blockers:

- A reusable session actor extractor and Super Admin guard exist; seller brand-scoped authorization remains partial and must still be expanded/reviewed where not yet covered.
- Explicit persisted proof permissions (`payment_proof.view_assigned`, `payment_proof.verify`, `payment_proof.reject`) were later added as schema foundation; broader proof workflow/runtime evidence still belongs to later Gate F/D slices.
- Login/install/checkout/order lookup/payment-proof upload now have single-process MVP route-specific throttling; distributed/proxy-aware limiting remains incomplete.
- Audit writes for login/logout/install setup now exist; broader permission/package/payment-proof audit coverage remains incomplete.

Gate E remaining blockers:

- Commerce foundation is implemented and later runtime-smoke-validated, including successful checkout/order lookup. Remaining work is broader production UX, richer commerce coverage, and later lifecycle/status flows rather than absence of the foundation itself.

Gate F remaining blockers:

- Payment proof upload workflow is not implemented.
- Protected media route is not implemented.
- `file_objects` metadata persistence for uploads is not implemented.
- Proof file access audit is not implemented.

Gate G remaining blockers:

- Package registry is still static; DB-backed package state/entitlement behavior remains incomplete.
- Server-side feature gate enforcement on paid endpoints is not yet evidenced.

Gate H remaining blockers:

- Production UI flows, reusable component system, Playwright E2E, screenshots, and accessibility evidence are not complete.

Gate I remaining blockers:

- Diagnostics export and redaction tests are not implemented.
- Threat model docs for payment proof, tunnel, desktop DB, entitlement, install lock, and diagnostics remain missing.
- Backup/restore docs/scripts/dry-runs are not complete.
- Dependency/container/static scans are not complete beyond current baseline.

Gate J remaining blockers:

- Backend/web release artifacts, production web image, SBOM/checksums/release manifest, systemd/container smoke evidence, and desktop all-platform RC artifacts are not complete.

## Status

Status for this implementation slice: **PASS_WITH_RISKS**.

Status for original user request “complete Gate B–J”: **not complete**. Gate B/D/I foundations improved, but Gate C–J remain multi-phase work requiring additional implementation and evidence.
