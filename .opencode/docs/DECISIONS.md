# Decisions

Current locked decisions are maintained primarily in `README.md`, `PRD.md`, `TRD.md`, and the release-gate plans.

Important examples already locked there:

- self-hosted, not centrally hosted SaaS,
- separate SvelteKit web service and Axum API for VPS production,
- variant-only pricing direction in the release-gate plan,
- payment proof private by default with explicit seller-file access permission,
- tunnel opt-in and no public PostgreSQL exposure.

When a new durable decision is made, update the canonical source doc and reflect it here only as an index if needed.
