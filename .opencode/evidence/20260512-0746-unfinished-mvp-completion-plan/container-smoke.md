# Container Smoke Evidence

Task ID: `20260512-0746-unfinished-mvp-completion-plan`

Status: **Passed on alternate host ports**

## Scope

- `scripts/release/smoke_compose.sh`
- `docker-compose.yml` host-port parametrization
- web service healthcheck

## Validation executed

- `sh -n scripts/release/smoke_compose.sh`
  - Result: **passed**
- initial live run on default ports
  - Result: **failed** due to host port `8301` already allocated
- second live run on alternate ports:
  - `HOST_BACKEND_PORT=18301`
  - `HOST_WEB_PORT=18300`
  - `BACKEND_BASE_URL=http://127.0.0.1:18301`
  - `PUBLIC_API_BASE_URL=http://127.0.0.1:18301`
  - `WEB_ORIGIN=http://127.0.0.1:18300`
  - `BASE_URL=http://127.0.0.1:18301`
  - `WEB_URL=http://127.0.0.1:18300/`
  - Result: **passed**

## Evidence paths

- `dist/release-evidence/compose-smoke-20260512T010901Z/summary.txt`
- `dist/release-evidence/compose-smoke-20260512T010901Z/runtime-contract.txt`
- `dist/release-evidence/compose-smoke-20260512T010901Z/compose-ps.txt`
- `dist/release-evidence/compose-smoke-20260512T010901Z/web-home.html`

## Notes

- Runtime contract passed with expected fresh-install-style `not_ready` because install lock/setup was intentionally absent in the smoke stack.
