#!/usr/bin/env sh
set -eu

DRY_RUN="${DRY_RUN:-1}"
SOURCE_DIR="${SOURCE_DIR:-}"
OUTPUT_DIR="${OUTPUT_DIR:-./backups}"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"

if [ -z "${SOURCE_DIR}" ]; then
  echo "SOURCE_DIR is required" >&2
  exit 1
fi

if [ ! -d "${SOURCE_DIR}" ]; then
  echo "SOURCE_DIR does not exist: ${SOURCE_DIR}" >&2
  exit 1
fi

mkdir -p "${OUTPUT_DIR}"
ARCHIVE_PATH="${OUTPUT_DIR}/storage-${STAMP}.tar.gz"
CMD="tar -C \"${SOURCE_DIR}\" -czf \"${ARCHIVE_PATH}\" ."

echo "[backup_storage] dry_run=${DRY_RUN}"
echo "[backup_storage] source=${SOURCE_DIR}"
echo "[backup_storage] output=${ARCHIVE_PATH}"
echo "[backup_storage] command=${CMD}"

if [ "${DRY_RUN}" != "0" ]; then
  echo "[backup_storage] dry-run only; nothing executed"
  exit 0
fi

sh -c "${CMD}"
sha256sum "${ARCHIVE_PATH}"
