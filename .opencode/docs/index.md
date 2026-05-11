# OpenCode Docs Index

This repository uses `.opencode/docs/` as the harness-facing map for agent workflow, while product and engineering truth stays in the root project docs.

Read in this order unless the task is trivial:

1. `AGENT_ROUTING.md`
2. `ARCHITECTURE.md`
3. `QUALITY.md`
4. `SECURITY.md`
5. `DECISIONS.md`
6. `RELEASE.md` when deployment, CI, or rollback is in scope

Then read the canonical project sources:

- `README.md` for current project status and validation reality
- `PRD.md` for product scope and locked behavior
- `TRD.md` for architecture, runtime, and boundaries
- `ERD.md` for data model and canonical statuses
- `DESIGN.md` for UI/design direction

Supporting harness docs here should stay short and point back to those canonical sources instead of duplicating them.
