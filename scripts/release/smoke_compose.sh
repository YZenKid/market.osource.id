#!/usr/bin/env sh
set -eu

COMPOSE_BIN="${COMPOSE_BIN:-docker compose}"
CURL_BIN="${CURL_BIN:-curl}"
BASE_URL="${BASE_URL:-http://127.0.0.1:8301}"
WEB_URL="${WEB_URL:-http://127.0.0.1:8300/}"
OUTPUT_ROOT="${OUTPUT_ROOT:-dist/release-evidence}"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
OUTPUT_DIR="${OUTPUT_ROOT}/compose-smoke-${STAMP}"
START_STACK="${START_STACK:-0}"
STOP_STACK="${STOP_STACK:-0}"
PROJECT_NAME="${PROJECT_NAME:-market-osource-smoke}"
STARTED_STACK=0
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

if [ "${START_STACK}" = "1" ]; then
  sh -c "${COMPOSE_BIN} -p \"${PROJECT_NAME}\" up -d" 
  STARTED_STACK=1
fi

sh -c "${COMPOSE_BIN} -p \"${PROJECT_NAME}\" ps" > "${OUTPUT_DIR}/compose-ps.txt"
BASE_URL="${BASE_URL}" sh "$(dirname "$0")/verify_runtime_contract.sh" | tee "${OUTPUT_DIR}/runtime-contract.txt"
fetch_web

if [ "${STOP_STACK}" = "1" ] && [ "${STARTED_STACK}" = "1" ]; then
  sh -c "${COMPOSE_BIN} -p \"${PROJECT_NAME}\" down"
fi

cat > "${OUTPUT_DIR}/summary.txt" <<EOF
compose smoke: PASS
project_name=${PROJECT_NAME}
base_url=${BASE_URL}
web_url=${WEB_URL}
started_stack=${STARTED_STACK}
evidence_dir=${OUTPUT_DIR}
EOF

printf '%s\n' "${OUTPUT_DIR}"
