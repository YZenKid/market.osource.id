# Release and Operational Readiness

Primary operational docs:

- `docs/deployment/vps.md`
- `docs/deployment/reverse-proxy.md`
- `docs/deployment/backup-restore.md`
- `docs/release/artifact-manifest.md`
- `docs/release/release-notes-template.md`

Operational helper scripts:

- `scripts/backup/backup_postgres.sh`
- `scripts/backup/backup_storage.sh`
- `scripts/backup/restore_preflight.sh`
- `scripts/release/generate_checksums.sh`
- `scripts/release/generate_sbom.sh`

Workflow notes:

- VPS baseline supports binary+systemd and Docker Compose.
- Public routing should expose one origin with SvelteKit app routes and Axum `/api/*` plus protected media.
- Release evidence should include checksum/SBOM notes when available, smoke validation, and rollback/upgrade references.
