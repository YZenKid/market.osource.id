# Agent Routing

- `@orchestrator` owns routing, integration, validation posture, and final summaries.
- Use `@explorer` for broad discovery before planning.
- Use `@librarian` for version-sensitive library or framework behavior.
- Use `@fixer` for bounded implementation and tests once scope is clear.
- Use `@designer` for user-facing UI, responsive layout, reference matching, and visual polish. Read `DESIGN.md` first.
- Use `@oracle` for architecture review, simplification, and high-risk tradeoffs.
- Use `@quality-gate` after non-trivial, risky, security-sensitive, or prompt/config changes before claiming completion.

Risk-trigger routing:

- Product ambiguity → `@product-architect`
- SaaS, tenancy, RBAC, billing → `@saas-architect`
- AI, LLM, RAG, evals → `@ai-systems-architect`
- Auth, PII, uploads, payment proof, privacy → `@security-privacy-reviewer`
- CI/CD, deploy, migrations, rollback, monitoring → `@release-engineer`
- Native mobile, offline, push, deep links, camera/QR → `@mobile-architect`

Validation routing:

- UI validation stays with `@designer` plus browser evidence when runnable.
- Test writing and touched test files usually stay with `@fixer`.
- Final conformance and repo-risk signoff stays with `@quality-gate`.
