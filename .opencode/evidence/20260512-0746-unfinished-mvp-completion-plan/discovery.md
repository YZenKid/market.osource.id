# Discovery — Unfinished MVP Completion Plan

Task ID: `20260512-0746-unfinished-mvp-completion-plan`

## Files inspected

- `.opencode/plans/20260511-2156-project-completion-master-plan.md`
- `.opencode/evidence/20260511-2156-project-completion-master-plan/gate-i-j-ops-release-foundation.md`
- `.opencode/evidence/20260511-2156-project-completion-master-plan/current-captures.md`

## Project patterns found

- Master plan sebelumnya sudah menyelesaikan commerce core dan release/runtime evidence foundation.
- Browser evidence masih blocked oleh runtime Chromium/Chrome yang belum tersedia.
- Runtime/release contract (`/health`, `/ready`, `/version`) sudah naik dan lolos quality gate.
- Gap tersisa lebih dominan pada lane operasional nyata: desktop supervision, deployment smoke, restore rehearsal, release closeout, browser usability proof.

## Reuse candidates

- `crates/core-api/src/routes/health.rs`
- `crates/core-api/src/routes/system.rs`
- `apps/desktop/src-tauri/src/main.rs`
- `scripts/backup/*`
- `scripts/release/verify_runtime_contract.sh`
- `docs/deployment/*`
- `docs/release/*`

## Commands / docs checked

- master plan implementation steps / evidence requirements
- ops release evidence from Gate I/J
- browser blocker evidence from current captures

## Constraints

- Visual/browser lane masih environment-blocked.
- Plan baru harus fokus ke pekerjaan yang benar-benar belum selesai, bukan mengulang master plan penuh.

## User decisions captured

- Desktop runtime supervision harus mencakup PostgreSQL sidecar control juga.
- Execution priority berjalan langsung untuk desktop dan VPS lane sekaligus.

## Risks

- Scope dapat melebar jika desktop supervision tidak dibatasi.
- Restore rehearsal bisa memakan waktu tinggi jika staging target tidak siap.
- Browser blocker dapat bertahan dan harus diperlakukan sebagai documented limitation bila belum terselesaikan.
