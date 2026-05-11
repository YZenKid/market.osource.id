#!/usr/bin/env sh
set -eu

BACKUP_FILE="${BACKUP_FILE:-}"
TARGET_DIR="${TARGET_DIR:-}"
TARGET_DATABASE_URL="${TARGET_DATABASE_URL:-}"

if [ -z "${BACKUP_FILE}" ]; then
  echo "BACKUP_FILE is required" >&2
  exit 1
fi

if [ ! -f "${BACKUP_FILE}" ]; then
  echo "backup file not found: ${BACKUP_FILE}" >&2
  exit 1
fi

echo "[restore_preflight] backup_file=${BACKUP_FILE}"
echo "[restore_preflight] target_dir=${TARGET_DIR:-<unset>}"
echo "[restore_preflight] target_database_url_configured=$( [ -n "${TARGET_DATABASE_URL}" ] && echo true || echo false )"

if [ -n "${TARGET_DIR}" ] && [ -e "${TARGET_DIR}" ]; then
  echo "[restore_preflight] target_dir exists: ${TARGET_DIR}"
else
  echo "[restore_preflight] target_dir missing or unset; this is acceptable for checklist-only review"
fi

echo "[restore_preflight] checksum:"
sha256sum "${BACKUP_FILE}"

echo "[restore_preflight] no restore executed; validate target isolation manually before any destructive command"
