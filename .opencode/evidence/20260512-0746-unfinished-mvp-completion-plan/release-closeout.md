# Release Closeout Evidence

Task ID: `20260512-0746-unfinished-mvp-completion-plan`

Status: **Helper implemented and dry closeout executed**

## Scope

- `scripts/release/release_closeout.sh`
- `scripts/release/smoke_systemd.sh`
- `scripts/release/smoke_compose.sh`
- docs wiring for release artifacts

## Validation executed

- `sh -n scripts/release/release_closeout.sh`
  - Result: **passed**
- dry closeout:
  - `DIST_DIR=/tmp/opencode/release-closeout-live SMOKE_MODE=none BASE_URL=http://127.0.0.1:18080 sh scripts/release/release_closeout.sh`
  - Result: **passed**
  - checksum file written
  - SBOM skipped safely because `syft` was not installed

## Evidence paths

- `/tmp/opencode/release-closeout-live/SHA256SUMS`
- `/tmp/opencode/release-closeout-live/release-evidence/closeout-20260512T010924Z/summary.txt`
- `/tmp/opencode/release-closeout-live/release-evidence/closeout-20260512T010924Z/runtime-contract.txt`

## Residual risks

- This host run was a dry closeout lane (`SMOKE_MODE=none`), not a full systemd+compose combined closeout.
- SBOM generation remains best-effort because `syft` is not installed here.
