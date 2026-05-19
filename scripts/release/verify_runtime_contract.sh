#!/usr/bin/env sh
set -eu

BASE_URL="${BASE_URL:-http://127.0.0.1:8301}"
CURL_BIN="${CURL_BIN:-curl}"

health_json="$(${CURL_BIN} --fail --silent "${BASE_URL}/health")"
ready_json="$(${CURL_BIN} --fail --silent "${BASE_URL}/ready")"
version_json="$(${CURL_BIN} --fail --silent "${BASE_URL}/version")"

printf '%s' "${health_json}" | grep '"probe":"liveness"' >/dev/null
printf '%s' "${health_json}" | grep '"status":"healthy"' >/dev/null
printf '%s' "${ready_json}" | grep '"probe":"readiness"' >/dev/null
printf '%s' "${ready_json}" | grep '"checks":' >/dev/null
printf '%s' "${version_json}" | grep '"app":"market.osource.id"' >/dev/null
printf '%s' "${version_json}" | grep '"version":' >/dev/null

echo "[verify_runtime_contract] base_url=${BASE_URL}"
echo "[verify_runtime_contract] health=${health_json}"
echo "[verify_runtime_contract] ready=${ready_json}"
echo "[verify_runtime_contract] version=${version_json}"
echo "[verify_runtime_contract] runtime contract verified"
