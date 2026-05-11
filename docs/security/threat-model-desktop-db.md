# Threat Model — Desktop Bundled PostgreSQL

## Asset utama

- database marketplace operator,
- kredensial database lokal,
- desktop data directory,
- backup lokal operator.

## Ancaman utama

1. PostgreSQL desktop bind ke interface publik.
2. File data lokal dicopy dari mesin operator tanpa kontrol OS-level.
3. Update/migration gagal dan meninggalkan data setengah termigrasi.
4. Diagnostics/log menampilkan path data atau DSN lengkap.

## Kontrol wajib

- PostgreSQL desktop wajib bind `127.0.0.1`.
- Data directory harus berada di path aplikasi/operator yang terdokumentasi dan tidak di-share publik.
- Backup/restore guide harus tersedia untuk operator non-DevOps.
- Migration runner harus memakai advisory lock dan recovery notes.
- Diagnostics hanya menampilkan keberadaan konfigurasi/path secara redacted, bukan raw path/DSN.

## Review checklist

- [ ] sidecar status mempertahankan loopback bind;
- [ ] backup preflight/checklist desktop tersedia;
- [ ] restore dry-run steps terdokumentasi;
- [ ] migration advisory lock diuji dengan live PostgreSQL test di CI atau workflow foundation;
- [ ] tidak ada path lokal operator di frontend/runtime export.

## Residual risk

- Endpoint security desktop tetap bergantung pada keamanan OS/operator machine.
- Bundled DB belum otomatis memberi encryption-at-rest.
