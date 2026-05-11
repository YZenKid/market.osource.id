#!/usr/bin/env sh
set -eu

DRY_RUN="${DRY_RUN:-1}"
OUTPUT_DIR="${OUTPUT_DIR:-./backups}"
DATABASE_URL="${DATABASE_URL:-}"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
OUTPUT_FILE="${OUTPUT_DIR}/postgres-${STAMP}.dump"

if [ -z "${DATABASE_URL}" ]; then
  echo "DATABASE_URL is required" >&2
  exit 1
fi

mkdir -p "${OUTPUT_DIR}"

CMD="pg_dump --format=custom --file \"${OUTPUT_FILE}\" \"${DATABASE_URL}\""

echo "[backup_postgres] dry_run=${DRY_RUN}"
echo "[backup_postgres] output=${OUTPUT_FILE}"
echo "[backup_postgres] command=${CMD}"

if [ "${DRY_RUN}" != "0" ]; then
  echo "[backup_postgres] dry-run only; nothing executed"
  exit 0
fi

sh -c "${CMD}"
sha256sum "${OUTPUT_FILE}"
