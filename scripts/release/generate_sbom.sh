#!/usr/bin/env sh
set -eu

DIST_DIR="${DIST_DIR:-./dist}"
OUTPUT_FILE="${OUTPUT_FILE:-${DIST_DIR}/sbom-repository.spdx.json}"

if ! command -v syft >/dev/null 2>&1; then
  echo "syft not installed; skipping SBOM generation" >&2
  exit 0
fi

mkdir -p "$(dirname "${OUTPUT_FILE}")"
syft packages dir:. -o "spdx-json=${OUTPUT_FILE}"
echo "wrote ${OUTPUT_FILE}"
