# Golden Principles

- Prefer evidence over assertion.
- Prefer repo-local docs over chat memory.
- Keep `AGENTS.md` short and use `.opencode/docs/` as the harness-facing map.
- Treat `PRD.md`, `TRD.md`, `ERD.md`, `README.md`, and `DESIGN.md` as the canonical project sources.
- Backend authorization is authoritative; frontend never defines access control.
- Do not commit secrets, tokens, dumps, or `.env` files.
- Update canonical docs when changing product, architecture, security, or data-model decisions.
