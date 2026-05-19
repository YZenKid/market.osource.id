# Reverse Proxy Baseline

VPS production uses separate SvelteKit and Axum services while exposing one public origin where possible.

## Recommended route split

```txt
https://market.example.com/
  /api/*    -> Axum backend service
  /media/*  -> Axum protected media service
  /*        -> SvelteKit web service
```

## Security requirements

- Terminate HTTPS at the reverse proxy or tunnel boundary.
- Set `COOKIE_SECURE=true` in production.
- Keep `CORS_ALLOWED_ORIGINS` explicit; do not use `*` with credentials.
- Forward `X-Forwarded-Proto`, `X-Forwarded-Host`, and request IDs consistently.
- Preserve `/api/*` and `/media/*` path prefixes when proxying to Axum; do not use prefix-stripping rules unless the backend routes are changed intentionally.
- Do not expose PostgreSQL, Tauri/desktop internals, debug ports, or sidecar control ports.
- Protected payment proof media must route to Axum, never to static SvelteKit assets.

## Caddy sketch

```caddyfile
market.example.com {
  encode zstd gzip

  handle /api/* {
    reverse_proxy 127.0.0.1:7301
  }

  handle /media/* {
    reverse_proxy 127.0.0.1:7301
  }

  handle {
    reverse_proxy 127.0.0.1:7300
  }
}
```

## Nginx sketch

```nginx
server {
  listen 443 ssl http2;
  server_name market.example.com;

  location /api/ {
    proxy_pass http://127.0.0.1:7301;
    proxy_set_header Host $host;
    proxy_set_header X-Forwarded-Proto $scheme;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
  }

  location /media/ {
    proxy_pass http://127.0.0.1:7301;
    proxy_set_header Host $host;
    proxy_set_header X-Forwarded-Proto $scheme;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
  }

  location / {
    proxy_pass http://127.0.0.1:7300;
    proxy_set_header Host $host;
    proxy_set_header X-Forwarded-Proto $scheme;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
  }
}
```
