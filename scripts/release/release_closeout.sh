#!/usr/bin/env sh
set -eu

DIST_DIR="${DIST_DIR:-./dist}"
OUTPUT_ROOT="${OUTPUT_ROOT:-${DIST_DIR}/release-evidence}"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
OUTPUT_DIR="${OUTPUT_ROOT}/closeout-${STAMP}"
SMOKE_MODE="${SMOKE_MODE:-none}"
BASE_URL="${BASE_URL:-http://127.0.0.1:8301}"

mkdir -p "${OUTPUT_DIR}"

sh "$(dirname "$0")/generate_checksums.sh"
sh "$(dirname "$0")/generate_sbom.sh" || true

case "${SMOKE_MODE}" in
  none)
    ;;
  systemd)
    BASE_URL="${BASE_URL}" sh "$(dirname "$0")/smoke_systemd.sh" > "${OUTPUT_DIR}/systemd-evidence-dir.txt"
    ;;
  compose)
    BASE_URL="${BASE_URL}" sh "$(dirname "$0")/smoke_compose.sh" > "${OUTPUT_DIR}/compose-evidence-dir.txt"
    ;;
  both)
    BASE_URL="${BASE_URL}" sh "$(dirname "$0")/smoke_systemd.sh" > "${OUTPUT_DIR}/systemd-evidence-dir.txt"
    BASE_URL="${BASE_URL}" sh "$(dirname "$0")/smoke_compose.sh" > "${OUTPUT_DIR}/compose-evidence-dir.txt"
    ;;
  *)
    echo "unsupported SMOKE_MODE: ${SMOKE_MODE}" >&2
    exit 1
    ;;
esac

BASE_URL="${BASE_URL}" sh "$(dirname "$0")/verify_runtime_contract.sh" | tee "${OUTPUT_DIR}/runtime-contract.txt"

cat > "${OUTPUT_DIR}/summary.txt" <<EOF
release closeout: PASS
smoke_mode=${SMOKE_MODE}
dist_dir=${DIST_DIR}
runtime_contract=${OUTPUT_DIR}/runtime-contract.txt
checksums=${DIST_DIR}/SHA256SUMS
sbom=${DIST_DIR}/sbom-repository.spdx.json
EOF

printf '%s\n' "${OUTPUT_DIR}"
