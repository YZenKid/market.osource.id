# Decisions Log Map

Canonical decision sources remain the root docs, especially `PRD.md`, `TRD.md`, `ERD.md`, and `README.md`.

Quick locked-decision reminders for routing:

- One installation equals one marketplace.
- The product is self-hosted, not SaaS-hosted.
- A marketplace can contain multiple brands.
- Seller accounts are created by Super Admin; no self-service seller registration.
- Orders may span multiple brands and use `order_brand_groups`.
- MVP checkout uses manual transfer with private payment-proof handling.
- Base open source must work without paid entitlements.
- MVP package system is capability registry plus migration boundary, not dynamic plugins.

If a task changes any of those assumptions, update the root docs instead of only changing code or local notes.
