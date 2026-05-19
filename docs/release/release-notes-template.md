# Release Notes Template

## Release identity

- Version:
- Channel:
- Commit:
- Build date:

## Included artifacts

- Backend binary:
- Web build/service artifact:
- Container image:
- Desktop installers (if any):
- Checksums:
- SBOM:

## Known limitations

- Example: desktop all-platform packaging smoke belum lengkap di host CI default.
- Example: entitlement remote verification masih foundation/static state.
- Example: payment proof malware scanning belum tersedia pada MVP.

## Upgrade notes

1. Ambil backup database + storage sebelum upgrade.
2. Verifikasi release manifest cocok dengan target runtime.
3. Terapkan artifact baru di staging/rehearsal dulu bila memungkinkan.
4. Jalankan health/readiness checks setelah deploy.
5. Periksa migration result dan package compatibility state.

## Rollback notes

1. Jangan rollback tanpa backup snapshot yang tervalidasi.
2. Jika migration irreversible sudah berjalan, restore dari backup konsisten.
3. Kembalikan backend/web/container artifact ke versi sebelumnya.
4. Verifikasi aplikasi lama cocok dengan schema/data restore target.
5. Catat insiden dan sebab rollback.

## Validation evidence

- CI workflow:
- Smoke test notes:
- Manual operator verification:
- Runtime contract verification (`/health`, `/ready`, `/version`):
- Backup metadata/checksum evidence:
- systemd smoke evidence dir:
- compose smoke evidence dir:
- release closeout summary:
