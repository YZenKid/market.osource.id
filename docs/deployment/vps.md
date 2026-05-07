# VPS Deployment Baseline

This baseline supports two VPS modes from the first implementation phase:

1. Binary + systemd.
2. Container + Docker Compose.

Both modes assume PostgreSQL is reachable only from trusted local/container networking and that public traffic reaches the Axum HTTP app through a reverse proxy or Cloudflare Tunnel. Never expose PostgreSQL publicly.

## Required environment

Copy `.env.example` to `/etc/market-osource/market-osource.env` for systemd or set equivalent container environment variables.

Sensitive values, especially `DATABASE_URL`, must not be committed, printed in diagnostics, or sent to the frontend bundle.

## Binary + systemd sketch

1. Create a `market` system user.
2. Install `market-backend` to `/usr/local/bin/market-backend`.
3. Create `/var/lib/market-osource/storage` owned by `market`.
4. Create `/etc/market-osource/market-osource.env` based on `.env.example`.
5. Install `docs/deployment/systemd/market-osource.service` to `/etc/systemd/system/`.
6. Run `systemctl daemon-reload && systemctl enable --now market-osource`.
7. Verify `/health`, `/ready`, and `/version` through the local reverse proxy.

## Container sketch

Use `docker-compose.yml` for local VPS smoke testing. Replace default database password before production. Keep named volumes for PostgreSQL data and payment-proof/product media storage.

## Security notes

- Use HTTPS at the reverse proxy/tunnel boundary.
- Set `COOKIE_SECURE=true` for HTTPS production URLs.
- Use explicit `CORS_ALLOWED_ORIGINS`; never use wildcard origins for cookie-authenticated routes.
- Payment proof storage must remain private and served only through backend RBAC routes in later phases.
- Tunnel is opt-in and must route only the HTTP app, not PostgreSQL, debug ports, or sidecar internals.
