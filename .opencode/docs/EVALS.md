# Evals and Replayability

Keep tasks replayable:

- Create or reuse a task plan in `.opencode/plans/` for non-trivial work.
- Store validation notes and artifacts in `.opencode/evidence/<task-id>/`.
- Record exact commands, environment assumptions, and any skipped checks.

Replay baselines in this repo:

- `.github/workflows/ci.yml` for Rust fmt/clippy/test, web check/build, and secret hygiene
- `.github/workflows/release-foundation.yml` for ignored live PostgreSQL migration tests and Docker build smoke

When DB-backed behavior matters, document the `TEST_DATABASE_URL` shape used.
When UI work is runnable, add browser screenshots and note viewport(s).
