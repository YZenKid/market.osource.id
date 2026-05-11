# Security Policy Map

Read first:

- `TRD.md` security sections
- `docs/security/README.md`
- The relevant threat model file under `docs/security/`

Non-negotiable implementation posture in this repo:

- Backend-enforced auth and RBAC
- Brand-scoped seller authorization
- Argon2id password hashing
- Server-side sessions and CSRF protection for cookie-authenticated mutations
- Rate limits for login, checkout, upload, install panel, and order lookup
- Install lock enforced server-side
- Desktop PostgreSQL bound to loopback only
- Tunnel opt-in and never exposing PostgreSQL or internal control ports
- Explicit CORS allowlist, never wildcard credentials
- Private payment-proof storage through backend authorization only
- Upload validation for size, MIME, extension, magic bytes, and dimensions
- No secrets in logs, diagnostics, frontend bundles, or commits

Any task touching auth, uploads, payment proof, tunnel, install flow, secrets, packages, or customer data should route through `@security-privacy-reviewer` and then `@quality-gate`.
