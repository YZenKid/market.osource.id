# Gate I/J Ops + Release Foundation Evidence

Task ID: `20260508-0548-release-gate-plan`

## Scope Implemented in This Slice

This evidence captures a bounded Gate I/J foundation slice. It does **not** claim Gate I or Gate J complete.

Implemented:

- admin-only diagnostics export endpoint at `GET /api/admin/diagnostics/export`;
- diagnostics redaction helpers and tests for secret/path/object-key/PII suppression;
- concise actionable security threat-model docs for payment proof, tunnel, desktop DB, entitlement privacy, install lock, and diagnostics redaction;
- backup/restore operator guide plus safe helper scripts with dry-run or preflight defaults;
- release artifact docs: manifest schema, checksum/SBOM command references, release notes template with known limitations / rollback / upgrade sections;
- safe CI foundation workflow for live PostgreSQL ignored migration tests and Docker build smoke.

## Files Changed

- `crates/core-api/src/lib.rs`
- `crates/core-api/src/diagnostics.rs`
- `crates/core-api/src/routes/admin.rs`
- `crates/core-api/src/routes/system.rs`
- `docs/security/README.md`
- `docs/security/threat-model-payment-proof.md`
- `docs/security/threat-model-tunnel.md`
- `docs/security/threat-model-desktop-db.md`
- `docs/security/threat-model-entitlement-privacy.md`
- `docs/security/threat-model-install-lock.md`
- `docs/security/threat-model-diagnostics-redaction.md`
- `docs/deployment/backup-restore.md`
- `docs/release/artifact-manifest.md`
- `docs/release/manifest.schema.json`
- `docs/release/release-notes-template.md`
- `scripts/backup/backup_postgres.sh`
- `scripts/backup/backup_storage.sh`
- `scripts/backup/restore_preflight.sh`
- `scripts/release/generate_checksums.sh`
- `scripts/release/generate_sbom.sh`
- `.github/workflows/release-foundation.yml`

## Validation Commands

| Command | Result | Notes |
| --- | --- | --- |
| `cargo fmt --check` | Passed | Rust formatting clean after diagnostics/docs additions. |
| `cargo test -p core-api -p core-db` | Passed | 33 passed, 6 ignored; ignored tests require disposable PostgreSQL via `TEST_DATABASE_URL`. |
| `npm --prefix apps/web run check` | Passed | `svelte-check found 0 errors and 0 warnings`. |

## Expected Operator / Reviewer Checks

- Review diagnostics JSON before sharing externally; redaction is designed to suppress secrets and private file references, but support exports should still be manually reviewed.
- Keep backup helpers in dry-run/preflight mode until target paths and database endpoints are explicitly verified.
- Treat `release-foundation` workflow as safe baseline only; it does not replace full RC packaging or restore rehearsal evidence.

## Remaining Gate Blockers

Gate I remaining blockers:

- Restore dry-run evidence for VPS systemd, container, and desktop data directory has not yet been captured.
- Broader dependency/container/static scans are still baseline-only.
- Payment proof implementation and end-to-end audit coverage remain incomplete.

Gate J remaining blockers:

- Real backend/web release artifacts are not yet produced in CI.
- Desktop all-platform RC packaging and smoke matrix remain out of scope for this slice.
- Signed checksums/SBOM publication flow is documented but not yet exercised as release evidence.

## Status

Status for this implementation slice: **PASS_WITH_RISKS**.

## 2026-05-11 Bounded Docker Smoke Fix

- Fixed backend Docker build blocker caused by Cargo workspace member resolution when `apps/desktop/src-tauri` was absent from image build context.
- `Dockerfile` now copies only minimal desktop crate files needed for workspace manifest/target validation (`Cargo.toml`, `build.rs`, `src/`) while still building only `market-backend`.
- This keeps release smoke aligned with CI intent: backend image build succeeds without introducing desktop runtime/toolchain dependencies into the backend container path.

Validation:

- `docker build --tag market-osource-smoke:local .` ✅
