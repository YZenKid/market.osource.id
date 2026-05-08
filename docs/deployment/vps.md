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
6. Install `docs/deployment/systemd/market-osource.service` to `/etc/systemd/system/`.
7. Add a separate web systemd unit before production. The web service should listen on loopback/internal networking only.
8. Run `systemctl daemon-reload && systemctl enable --now market-osource`.
9. Verify `/health`, `/ready`, and `/version` through the local reverse proxy.

## VPS service topology

VPS production uses two app services:

- **Axum backend**: API, auth/session, install API, business rules, protected media, package gates.
- **SvelteKit web**: storefront, admin UI, install UI. It consumes Axum APIs but is not an authorization authority.

Recommended public reverse proxy shape:

```txt
https://market.example.com/
  ├─ /api/*          -> Axum backend, e.g. 127.0.0.1:8080
  ├─ /media/*        -> Axum protected media, e.g. 127.0.0.1:8080
  └─ all app routes  -> SvelteKit web, e.g. 127.0.0.1:3000
```

Even when the reverse proxy presents one public origin, keep the backend CORS allowlist explicit for configured web/local origins. Never use wildcard CORS for cookie-authenticated routes.

## Container sketch

Use `docker-compose.yml` for local VPS smoke testing. It models separate `backend` and `web` services plus PostgreSQL. Replace default database password before production. Keep named volumes for PostgreSQL data and payment-proof/product media storage.

## Security notes

- Use HTTPS at the reverse proxy/tunnel boundary.
- Set `COOKIE_SECURE=true` for HTTPS production URLs.
- Use explicit `CORS_ALLOWED_ORIGINS`; never use wildcard origins for cookie-authenticated routes.
- Payment proof storage must remain private and served only through backend RBAC routes in later phases.
- Tunnel is opt-in and must route only the HTTP app, not PostgreSQL, debug ports, or sidecar internals.
