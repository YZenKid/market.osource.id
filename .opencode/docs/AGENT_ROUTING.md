# Agent Routing

Default owner is `@orchestrator`: route, decompose, integrate, and keep evidence legible.

Use these lanes first:

- `@artifact-planner` — plans and scoped execution blueprints under `.opencode/plans/`.
- `@fixer` — bounded implementation, test updates, and focused code changes.
- `@oracle` — architecture decisions, code review, simplification, and hard debugging.
- `@designer` — UI/UX direction, responsive behavior, design polish, and downstream UI review lanes.
- `@explorer` — codebase discovery when paths or ownership are unclear.
- `@librarian` — version-sensitive library/framework docs.
- `@quality-gate` — final conformance and risk signoff for material changes.

Escalate by risk domain:

- product ambiguity → `@product-architect`
- SaaS/RBAC/tenancy/billing boundaries → `@saas-architect`
- AI/LLM/RAG/tool-calling/evals → `@ai-systems-architect`
- auth/PII/uploads/payments/privacy/tunnel exposure → `@security-privacy-reviewer`
- CI/CD/deploy/migrations/rollback/ops → `@release-engineer`
- mobile/hybrid/offline/push/deep links/camera → `@mobile-architect`

Do not let all work collapse into direct `@orchestrator` execution when a bounded specialist lane is clearer.
