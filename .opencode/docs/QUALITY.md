# Quality and Evidence

This repo expects evidence, not assertion.

- Plans live under `.opencode/plans/`.
- Task evidence lives under `.opencode/evidence/<task-id>/`.
- For non-trivial work, record commands run, outputs, screenshots when relevant, and residual risks.

Current validation baseline reflects the actual repo state:

- Rust format: `cargo fmt --check`
- Rust lint: `cargo clippy --workspace --exclude market-desktop --all-targets -- -D warnings`
- Rust tests: `cargo test --workspace --exclude market-desktop`
- Live DB ignored tests: `TEST_DATABASE_URL=... cargo test -p core-db -- --ignored`
- Web checks: `npm --prefix apps/web run check`
- Web build: `npm --prefix apps/web run build`
- Docker smoke: `docker build --tag market-osource-smoke:local .`

Current repo notes from `README.md` and evidence:

- Host Rust may be unavailable locally; Docker `rust:1.88` is the accepted fallback for non-desktop validation.
- Full Tauri desktop build still depends on platform/system prerequisites and should not be assumed green unless explicitly validated.

Use `.github/workflows/ci.yml` and `.github/workflows/release-foundation.yml` as the CI baseline when selecting checks.
