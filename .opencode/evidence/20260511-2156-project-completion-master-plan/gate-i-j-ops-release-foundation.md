# Gate I/J Ops Release Foundation Evidence

Task ID: `20260511-2156-project-completion-master-plan`

Status: **Functional release/runtime evidence slice implemented; browser evidence remains separate blocker**

## Scope of this slice

- runtime probe contract enrichment for `/health`, `/ready`, `/version`
- desktop release/runtime visibility uplift
- backup helper metadata/checksum contract uplift
- restore preflight safety checks
- release/deployment docs aligned to executable verification helper

## Changed files

- `crates/core-api/src/routes/health.rs`
- `apps/desktop/src-tauri/src/main.rs`
- `scripts/backup/backup_postgres.sh`
- `scripts/backup/backup_storage.sh`
- `scripts/backup/restore_preflight.sh`
- `scripts/release/verify_runtime_contract.sh`
- `docs/deployment/backup-restore.md`
- `docs/deployment/vps.md`
- `docs/release/artifact-manifest.md`
- `docs/release/release-notes-template.md`

## What changed

### Runtime contract

- `/health` now returns explicit liveness metadata:
  - `probe`
  - `app`
  - `version`
  - `git_sha`
  - `runtime_mode`
  - `uptime_seconds`
- `/ready` now returns:
  - `probe`
  - `blocking_checks`
  - existing readiness checks
- `/version` now returns:
  - `runtime_mode`
  - `build_profile`

### Desktop visibility

- Desktop status now includes:
  - `release_identity`
  - `runtime_contract`
- This stays read-only and does not claim supervision.

### Backup/restore helpers

- `backup_postgres.sh` writes `*.metadata.json`
- `backup_storage.sh` writes `*.metadata.json`
- metadata includes:
  - `app_version`
  - `git_sha`
  - `runtime_mode`
  - `created_at`
  - `backup_type`
- `restore_preflight.sh` now accepts and validates optional:
  - `METADATA_FILE`
  - `CHECKSUM_FILE`
- when a checksum manifest references the supplied backup or metadata path directly, `restore_preflight.sh` now runs `sha256sum -c` instead of presence-only validation.
- `restore_preflight.sh` rejects obviously high-risk rehearsal targets such as `/`, `/var`, `/var/lib`, `/var/lib/market-osource`, `/etc`, `/etc/market-osource`.

### Release verification helper

- Added `scripts/release/verify_runtime_contract.sh`
- It verifies minimal probe contract presence for:
  - `/health`
  - `/ready`
  - `/version`

## Validation executed

### Rust

- `cargo test -p core-api -p core-app -p core-db`
  - Result: **passed** (`49 passed, 8 ignored`)

### Web

- `npm run check` from `apps/web`
  - Result: **passed** (`0 errors, 0 warnings`)
- `npm run build` from `apps/web`
  - Result: **passed**

### Backup helper dry-run validation

Executed with temporary paths under `/tmp/opencode/market-backup-test`:

- `backup_postgres.sh` in `DRY_RUN=1`
  - metadata path printed successfully
- `backup_storage.sh` in `DRY_RUN=1`
  - metadata path printed successfully
- `restore_preflight.sh`
  - accepted staged fake artifact + metadata + checksum file inputs
  - printed checksum(s)
  - preserved non-destructive behavior
  - checksum manifest validation was also exercised with `sha256sum -c` and returned `OK` for both backup and metadata sample files

### Live runtime contract validation

Executed against a live local backend process:

- backend launch env:
  - `RUNTIME_MODE=vps`
  - `BIND_ADDR=127.0.0.1:18080`
  - `BASE_URL=http://127.0.0.1:18080`
  - `DATABASE_URL=''`
  - `STORAGE_PATH=/tmp/opencode/market-runtime-contract/storage`
- verification command:
  - `BASE_URL=http://127.0.0.1:18080 sh scripts/release/verify_runtime_contract.sh`
- result:
  - `/health` verified as liveness probe
  - `/ready` verified as readiness probe
  - `/version` verified as runtime identity probe
  - expected fresh-runtime `not_ready` status observed because DB/install state was intentionally absent in this smoke lane

## Remaining risks

- This slice improves runtime/release evidence, but does **not** implement true desktop process supervision.
- Browser screenshot evidence remains blocked by missing Chromium/Chrome runtime and is outside this slice.

## Acceptance impact

This slice materially improves these master-plan release gates:

- `/health`, `/ready`, `/version` are clearer and more evidence-friendly
- backup/restore dry-run support now carries release/runtime metadata
- systemd/container release docs now point to an executable runtime verification helper
- desktop control panel exposes release/runtime identity without overclaiming supervision
