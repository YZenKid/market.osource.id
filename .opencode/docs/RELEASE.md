# Release

Primary release sources:

- `.opencode/plans/20260508-0548-release-gate-plan.md`
- `docs/release/artifact-manifest.md`
- `docs/release/release-notes-template.md`
- `docs/deployment/vps.md`
- `docs/deployment/backup-restore.md`

Release posture for this repo includes:

- binary + systemd and container support for VPS,
- desktop all-platform release-candidate expectations,
- SBOM/checksum/manifest evidence,
- backup/restore dry-run evidence,
- smoke tests and health/readiness verification.

Use `scripts/release/generate_sbom.sh` and checksum generation helpers where applicable.
