# Systemd Smoke Evidence

Task ID: `20260512-0746-unfinished-mvp-completion-plan`

Status: **Helper implemented; live smoke blocked because required market services are not installed/running on this host**

## Scope

- `scripts/release/smoke_systemd.sh`
- docs wiring to executable smoke command

## Validation executed

- `sh -n scripts/release/smoke_systemd.sh`
  - Result: **passed**
- `systemctl --version`
  - Result: **available on host**
- attempted live run:
  - `BACKEND_SERVICE=market-osource WEB_SERVICE=market-osource-web ... sh scripts/release/smoke_systemd.sh`
  - Result: **blocked** because those services are not provisioned/running in this environment.

## Residual risks

- The helper exists and is syntax-valid, but live systemd smoke still needs a host with the actual unit files installed and services started.
