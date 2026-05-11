# Quality

Evidence, not assertion, is required for material changes.

Validation baseline for this repo should reflect real workflow:

- Rust workspace: prefer Docker-based validation when host `cargo`/`rustc` are unavailable.
- Web app: `npm run check` and `npm run build` in `apps/web`.
- Release/ops helpers: use `scripts/backup/restore_preflight.sh`, `scripts/release/generate_sbom.sh`, and related backup scripts where relevant.
- Deployment posture: verify against `docs/deployment/vps.md` and `docs/deployment/backup-restore.md`.

Use `@quality-gate` before claiming completion for non-trivial, risky, prompt/config, release, or security-sensitive changes.
