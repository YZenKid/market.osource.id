# Threat Model — Tunnel Exposure (`cloudflared` / reverse proxy)

## Asset utama

- origin app HTTP service,
- auth/session cookies,
- admin/install endpoints,
- protected media routes.

## Ancaman utama

1. Tunnel salah konfigurasi mengekspos port non-HTTP seperti PostgreSQL atau debug port.
2. Operator menganggap localhost mode aman padahal origin sudah public.
3. Weak origin hardening: wildcard CORS, insecure cookies, atau install panel masih terbuka.
4. Tunnel public URL bocor dan dipakai brute-force ke login/install.

## Kontrol wajib

- Tunnel bersifat opt-in dan default disabled.
- Route publik hanya boleh ke reverse proxy/app HTTP origin, bukan ke PostgreSQL atau sidecar control ports.
- `COOKIE_SECURE=true` saat HTTPS public.
- CORS harus explicit allowlist, bukan `*` untuk route cookie-authenticated.
- Install panel harus locked server-side setelah setup.
- Rate limit minimal untuk login, install setup, upload, dan order lookup.

## Review checklist

- [ ] tunnel doc/operator guide menegaskan hanya expose HTTP app;
- [ ] PostgreSQL bind loopback/internal-only;
- [ ] install lock tervalidasi setelah setup;
- [ ] reverse proxy forward headers dan request-id konsisten;
- [ ] diagnostics hanya melaporkan status tunnel, bukan secret/token tunnel.

## Residual risk

- Tunnel tidak menggantikan hardening aplikasi.
- Public reachability tetap membuka risiko credential stuffing dan enumeration jika rate limit/monitoring belum matang.
