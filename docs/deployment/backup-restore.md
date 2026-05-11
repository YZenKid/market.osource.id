# Backup & Restore Guide

Dokumen ini memberi baseline aman untuk Gate I backup/restore tanpa default destruktif. Semua contoh harus diuji sebagai **dry-run terlebih dahulu**.

## Prinsip umum

- Backup database dan storage harus diperlakukan sebagai satu set konsisten.
- Simpan artefak backup di lokasi terpisah dari host runtime jika memungkinkan.
- Jangan overwrite data production saat restore rehearsal.
- Validasi checksum dan isi file sebelum restore.
- Simpan catatan versi aplikasi + migration state bersama backup.

## Yang wajib dibackup

1. PostgreSQL dump.
2. Storage directory lokal (gambar produk, logo, payment proof private).
3. Environment/config file operator yang dibutuhkan untuk restore.
4. Release/version note: commit/tag/build info, migration version, runtime mode.

## VPS binary + systemd

### Lokasi tipikal

- backend binary: `/usr/local/bin/market-backend`
- env file: `/etc/market-osource/market-osource.env`
- storage: `/var/lib/market-osource/storage`
- PostgreSQL: service lokal/server yang hanya reachable internal

### Backup checklist

- [ ] `systemctl status market-osource market-osource-web`
- [ ] konfirmasi `DATABASE_URL` mengarah ke instance yang benar
- [ ] jalankan `scripts/backup/backup_postgres.sh` dengan `DRY_RUN=1`
- [ ] jalankan `scripts/backup/backup_storage.sh` dengan `DRY_RUN=1`
- [ ] ulangi dengan `DRY_RUN=0` hanya setelah output command diverifikasi
- [ ] simpan checksum hasil backup

### Restore dry-run checklist

- [ ] siapkan host/VM/test path terpisah
- [ ] restore dump ke database kosong baru, bukan database production aktif
- [ ] extract storage archive ke direktori staging baru
- [ ] jalankan aplikasi dengan env staging dan verifikasi `/health` serta `/ready`
- [ ] cek apakah migration tambahan berjalan atau tidak

## Container / Docker Compose

### Volume yang harus dipreservasi

- `postgres_data`
- `app_storage`

### Backup checklist

- [ ] `docker compose ps`
- [ ] jalankan `scripts/backup/backup_postgres.sh` dari container/host dengan DSN yang benar
- [ ] archive volume storage ke file tar.gz
- [ ] catat image tag/backend build/web build yang sedang dipakai

### Restore dry-run checklist

- [ ] gunakan project name/compose stack berbeda untuk rehearsal
- [ ] restore DB ke volume baru atau database baru
- [ ] restore storage ke volume baru
- [ ] jalankan `docker compose up` di environment staging
- [ ] verifikasi healthcheck container backend dan web

## Desktop data directory

Desktop exact path akan mengikuti runtime adapter/final installer per OS, tetapi operator harus mengetahui tiga kelompok data:

- database cluster/data dir bundled PostgreSQL,
- local storage marketplace,
- runtime config/log yang dibutuhkan untuk diagnosis restore.

### Desktop backup checklist

- [ ] pastikan aplikasi idle atau operator memahami snapshot consistency risk
- [ ] identifikasi desktop app data root yang aktif
- [ ] jalankan `scripts/backup/backup_storage.sh` dalam mode dry-run untuk storage root
- [ ] buat PostgreSQL dump dengan kredensial lokal yang tervalidasi
- [ ] simpan backup di drive terpisah/cloud operator yang aman

### Desktop restore dry-run checklist

- [ ] jangan menimpa data operator aktif; gunakan mesin uji atau data root baru
- [ ] restore DB ke cluster baru/staging
- [ ] restore storage ke root baru
- [ ] jalankan aplikasi dengan config staging bila runtime mendukung override path

## Safe helper scripts

Helper scripts tersedia di `scripts/backup/`:

- `backup_postgres.sh` — default dry-run, opsional menjalankan `pg_dump`
- `backup_storage.sh` — default dry-run, opsional membuat `tar.gz`
- `restore_preflight.sh` — hanya validasi artefak/target, tidak melakukan restore

## Minimal evidence for Gate I

- command lines yang dipakai,
- checksum backup,
- restore rehearsal target yang dipakai,
- hasil `/health` dan `/ready` pada target restore rehearsal,
- catatan gap/residual risk.
