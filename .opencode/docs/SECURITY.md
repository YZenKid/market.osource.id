# Security

Primary security sources:

- `TRD.md`
- `docs/security/README.md`
- `docs/security/threat-model-payment-proof.md`
- `docs/security/threat-model-tunnel.md`
- `docs/security/threat-model-desktop-db.md`
- `docs/security/threat-model-entitlement-privacy.md`
- `docs/security/threat-model-install-lock.md`
- `docs/security/threat-model-diagnostics-redaction.md`

High-risk boundaries in this repo:

- auth/session/CSRF/CORS across separate SvelteKit and Axum services,
- brand-scoped RBAC,
- payment-proof private storage and file access,
- install lock,
- secret redaction in logs/diagnostics/artifacts,
- tunnel/public exposure posture.

Route these changes through `@security-privacy-reviewer` and finish with `@quality-gate`.
