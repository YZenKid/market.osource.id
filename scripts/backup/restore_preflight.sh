#!/usr/bin/env sh
set -eu

BACKUP_FILE="${BACKUP_FILE:-}"
TARGET_DIR="${TARGET_DIR:-}"
TARGET_DATABASE_URL="${TARGET_DATABASE_URL:-}"
METADATA_FILE="${METADATA_FILE:-}"
CHECKSUM_FILE="${CHECKSUM_FILE:-}"

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
echo "[restore_preflight] metadata_file=${METADATA_FILE:-<unset>}"
echo "[restore_preflight] checksum_file=${CHECKSUM_FILE:-<unset>}"

if [ -n "${TARGET_DIR}" ] && [ -e "${TARGET_DIR}" ]; then
  echo "[restore_preflight] target_dir exists: ${TARGET_DIR}"
else
  echo "[restore_preflight] target_dir missing or unset; this is acceptable for checklist-only review"
fi

echo "[restore_preflight] checksum:"
sha256sum "${BACKUP_FILE}"

if [ -n "${METADATA_FILE}" ]; then
  if [ ! -f "${METADATA_FILE}" ]; then
    echo "metadata file not found: ${METADATA_FILE}" >&2
    exit 1
  fi
  echo "[restore_preflight] metadata present"
  sha256sum "${METADATA_FILE}"
fi

if [ -n "${CHECKSUM_FILE}" ]; then
  if [ ! -f "${CHECKSUM_FILE}" ]; then
    echo "checksum file not found: ${CHECKSUM_FILE}" >&2
    exit 1
  fi
  echo "[restore_preflight] checksum file present: ${CHECKSUM_FILE}"

  if grep -F "${BACKUP_FILE}" "${CHECKSUM_FILE}" >/dev/null 2>&1; then
    (cd / && sha256sum -c "${CHECKSUM_FILE}")
  elif [ -n "${METADATA_FILE}" ] && grep -F "${METADATA_FILE}" "${CHECKSUM_FILE}" >/dev/null 2>&1; then
    (cd / && sha256sum -c "${CHECKSUM_FILE}")
  else
    echo "[restore_preflight] checksum file does not reference backup or metadata path directly; presence check only" >&2
  fi
fi

case "${TARGET_DIR}" in
  "/"|"/var"|"/var/lib"|"/var/lib/market-osource"|"/etc"|"/etc/market-osource")
    echo "[restore_preflight] refusing high-risk target_dir for rehearsal: ${TARGET_DIR}" >&2
    exit 1
    ;;
esac

echo "[restore_preflight] no restore executed; validate target isolation manually before any destructive command"
