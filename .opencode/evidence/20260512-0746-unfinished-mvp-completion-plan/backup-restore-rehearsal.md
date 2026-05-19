# Backup / Restore Rehearsal Evidence

Task ID: `20260512-0746-unfinished-mvp-completion-plan`

Status: **Passed**

## Validation executed

### Backup artifact generation

- PostgreSQL dump generated from running proof stack PostgreSQL container via `docker exec pg_dump`
- storage archive generated from staged storage source
- metadata file created for DB dump
- checksum manifest created and verified with `restore_preflight.sh`

### Restore rehearsal

- isolated PostgreSQL restore container started on `127.0.0.1:19532`
- DB dump restored into `market_restore`
- storage archive extracted into `/tmp/opencode/mvp-restore/restore-target/storage`
- backend booted against restored DB + restored storage on `127.0.0.1:19601`

### Probe results after restore

- `/health` → `healthy`
- `/ready` → `ready`
- `/version` → returned runtime identity successfully

## Outcome

- Full isolated restore rehearsal with app boot and probe capture is complete for this slice.
