#!/usr/bin/env sh
set -eu

SYSTEMCTL_BIN="${SYSTEMCTL_BIN:-systemctl}"
CURL_BIN="${CURL_BIN:-curl}"
BASE_URL="${BASE_URL:-http://127.0.0.1:7301}"
WEB_URL="${WEB_URL:-http://127.0.0.1:7300/}"
BACKEND_SERVICE="${BACKEND_SERVICE:-market-osource}"
WEB_SERVICE="${WEB_SERVICE:-market-osource-web}"
OUTPUT_ROOT="${OUTPUT_ROOT:-dist/release-evidence}"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
OUTPUT_DIR="${OUTPUT_ROOT}/systemd-smoke-${STAMP}"
CAPTURE_JOURNAL="${CAPTURE_JOURNAL:-0}"
WEB_RETRIES="${WEB_RETRIES:-20}"
WEB_RETRY_DELAY="${WEB_RETRY_DELAY:-2}"

mkdir -p "${OUTPUT_DIR}"

fetch_web() {
  attempt=1
  while [ "$attempt" -le "${WEB_RETRIES}" ]; do
    if "${CURL_BIN}" --fail --silent --show-error "${WEB_URL}" > "${OUTPUT_DIR}/web-home.html"; then
      return 0
    fi
    sleep "${WEB_RETRY_DELAY}"
    attempt=$((attempt + 1))
  done
  return 1
}

"${SYSTEMCTL_BIN}" is-active --quiet "${BACKEND_SERVICE}"
"${SYSTEMCTL_BIN}" is-active --quiet "${WEB_SERVICE}"

BASE_URL="${BASE_URL}" sh "$(dirname "$0")/verify_runtime_contract.sh" | tee "${OUTPUT_DIR}/runtime-contract.txt"
fetch_web

if [ "${CAPTURE_JOURNAL}" = "1" ]; then
  journalctl -u "${BACKEND_SERVICE}" -u "${WEB_SERVICE}" -n 200 --no-pager > "${OUTPUT_DIR}/journal-tail.txt"
fi

cat > "${OUTPUT_DIR}/summary.txt" <<EOF
systemd smoke: PASS
backend_service=${BACKEND_SERVICE}
web_service=${WEB_SERVICE}
base_url=${BASE_URL}
web_url=${WEB_URL}
evidence_dir=${OUTPUT_DIR}
EOF

printf '%s\n' "${OUTPUT_DIR}"
