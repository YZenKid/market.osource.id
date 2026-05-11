# Threat Model — Install Lock

## Asset utama

- first-admin creation,
- installation state,
- environment/bootstrap secrets,
- install panel routes.

## Ancaman utama

1. Install panel tetap bisa dijalankan ulang setelah marketplace production.
2. Brute-force atau replay ke setup endpoint.
3. Race condition dua operator menjalankan setup bersamaan.
4. Setup error mengungkap detail sensitif env/database.

## Kontrol wajib

- Install setup harus dicek server-side terhadap `installation_state` dan `locked` flag.
- Setup mutation wajib CSRF-protected dan rate-limited.
- Migration/install path memakai locking yang aman untuk mengurangi concurrent bootstrap race.
- Error response setup harus generik; detail teknis cukup ke log server.

## Review checklist

- [ ] setup kedua ditolak ketika installation locked;
- [ ] setup membutuhkan CSRF dan session-less abuse dibatasi rate limiter;
- [ ] install UI tidak menjadi source of truth lock state;
- [ ] diagnostics menampilkan state install secara ringkas tanpa credential/env dump.

## Residual risk

- Workflow bootstrap masih perlu auto-login/first-session ergonomics pada gate berikutnya.
