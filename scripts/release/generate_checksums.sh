#!/usr/bin/env sh
set -eu

DIST_DIR="${DIST_DIR:-./dist}"

if [ ! -d "${DIST_DIR}" ]; then
  echo "DIST_DIR not found: ${DIST_DIR}" >&2
  exit 1
fi

find "${DIST_DIR}" -maxdepth 1 -type f ! -name 'SHA256SUMS' -print | sort | while read -r file; do
  sha256sum "${file}"
done > "${DIST_DIR}/SHA256SUMS"

echo "wrote ${DIST_DIR}/SHA256SUMS"
