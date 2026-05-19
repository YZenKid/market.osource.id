# VPS Deployment Baseline

This baseline supports two VPS modes from the first implementation phase:

1. Binary + systemd.
2. Container + Docker Compose.

Both modes assume PostgreSQL is reachable only from trusted local/container networking and that public traffic reaches the SvelteKit web service plus Axum API through a reverse proxy or Cloudflare Tunnel. Never expose PostgreSQL publicly.

## Required environment

Copy `.env.example` to `/etc/market-osource/market-osource.env` for systemd or set equivalent container environment variables.

Sensitive values, especially `DATABASE_URL`, must not be committed, printed in diagnostics, or sent to the frontend bundle.

## Binary + systemd sketch

1. Create a `market` system user.
2. Install `market-backend` to `/usr/local/bin/market-backend`.
3. Build and install the SvelteKit web service from `apps/web` as a separate Node service.
4. Create `/var/lib/market-osource/storage` owned by `market`.
5. Create `/etc/market-osource/market-osource.env` based on `.env.example`.
6. Install `docs/deployment/systemd/market-osource.service` and `docs/deployment/systemd/market-osource-web.service` to `/etc/systemd/system/`.
7. Keep both app services on loopback/internal networking; expose only the reverse proxy/tunnel public endpoint.
8. Run `systemctl daemon-reload && systemctl enable --now market-osource market-osource-web`.
9. Verify `/health`, `/ready`, `/api/system/runtime`, and the install UI through the local reverse proxy.

Untuk verification yang konsisten antar lane, gunakan juga helper runtime contract:

```bash
BASE_URL=http://127.0.0.1:8301 ./scripts/release/verify_runtime_contract.sh
```

`/health` adalah liveness probe, sedangkan `/ready` adalah readiness probe yang boleh bernilai `not_ready` pada fresh install sebelum lock/setup selesai.

Untuk lane executable smoke, gunakan helper berikut:

```bash
# systemd lane
BASE_URL=http://127.0.0.1:8301 WEB_URL=http://127.0.0.1:8300/ ./scripts/release/smoke_systemd.sh

# compose lane, non-destructive by default
BASE_URL=http://127.0.0.1:8301 WEB_URL=http://127.0.0.1:8300/ ./scripts/release/smoke_compose.sh
```

## VPS service topology

VPS production uses two app services:

- **Axum backend**: API, auth/session, install API, business rules, protected media, package gates.
- **SvelteKit web**: storefront, admin UI, install UI. It consumes Axum APIs but is not an authorization authority.

Recommended public reverse proxy shape:

```txt
https://market.example.com/
  ├─ /api/*          -> Axum backend, e.g. 127.0.0.1:8301
  ├─ /media/*        -> Axum protected media, e.g. 127.0.0.1:8301
  └─ all app routes  -> SvelteKit web, e.g. 127.0.0.1:8300
```

Even when the reverse proxy presents one public origin, keep the backend CORS allowlist explicit for configured web/local origins. Never use wildcard CORS for cookie-authenticated routes.

## Container sketch

Use `docker-compose.yml` for local VPS smoke testing. It models separate `backend` and `web` services plus PostgreSQL. Replace default database password before production. Keep named volumes for PostgreSQL data and payment-proof/product media storage.

The Compose `web` service is built from the repository `Dockerfile` (`web-runtime` target). Dependencies and SvelteKit build artifacts are baked into the image at build time; the container does not run `npm ci` at startup and does not require a writable source bind-mount. This avoids restart loops when source mounts are read-only.

Untuk menjalankan stack smoke dari helper yang sama secara eksplisit, set `START_STACK=1`. Default helper compose tetap non-destruktif dan hanya memeriksa stack yang sudah berjalan.

## Security notes

- Use HTTPS at the reverse proxy/tunnel boundary.
- Set `COOKIE_SECURE=true` for HTTPS production URLs.
- Use explicit `CORS_ALLOWED_ORIGINS`; never use wildcard origins for cookie-authenticated routes.
- Payment proof storage must remain private and served only through backend RBAC routes in later phases.
- Tunnel is opt-in and must route only the HTTP app, not PostgreSQL, debug ports, or sidecar internals.

## Runtime status notes

- `/api/system/runtime` reports backend readiness, install state, package registry counts, local storage path, desktop PostgreSQL sidecar detection, and tunnel default-off status.
- Runtime status is read-only in the current Gate C slice. It must not start PostgreSQL, backend, or `cloudflared` processes.
- The desktop shell reports the same default-off PostgreSQL/tunnel posture for operator visibility while real process supervision is implemented in a later Gate C slice.
- `/version` should be recorded in release evidence together with `/health` and `/ready` so operators can tie smoke results to a concrete build identity.
