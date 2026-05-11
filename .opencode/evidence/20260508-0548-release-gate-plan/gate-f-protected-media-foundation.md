# Gate F Protected Media / Payment Proof Foundation Evidence

Date: 2026-05-10

Update: 2026-05-11

## Scope Implemented

- Added `0006_payment_proof_permissions.sql` with explicit `permissions` and `brand_member_permissions` tables.
- Added payment proof permission seeds:
  - `payment_proof.view_assigned`
  - `payment_proof.verify`
  - `payment_proof.reject`
- Kept seller file access deny-by-default: seller access requires both assigned brand/order scope and explicit `payment_proof.view_assigned` on `brand_member_permissions`.
- Extended local storage with safe `read_object(bucket, object_key)` using the existing traversal-safe path guard.
- Added DB primitives for:
  - file object metadata creation,
  - payment proof creation by order tracking token,
  - order payment status transition to `waiting_payment_verification`,
  - payment proof/file lookup,
  - assigned seller brand checks,
  - explicit brand-scoped permission checks,
  - super-admin proof verify/reject foundation.
- Added API foundation routes:
  - `POST /api/storefront/orders/:tracking_token/payment-proof`
  - `GET /media/:file_id`
  - `POST /api/admin/payment-proofs/:proof_id/verify`
  - `POST /api/admin/payment-proofs/:proof_id/reject`
- Added seller-scoped admin commerce access foundation:
  - seller can list only assigned brands on `GET /api/admin/brands`,
  - seller can list only assigned-brand products on `GET /api/admin/products`,
  - seller can create product only under assigned brand on `POST /api/admin/products`.

## Security Notes

- Payment proof files are stored under the private `payment-proofs` bucket with generated object keys; user filenames are metadata only and never used as storage paths.
- Protected media denies unauthenticated users and guest/customer tracking-token media access.
- Super Admin can read proof media and decide verify/reject.
- Seller proof media reads require:
  1. role code `seller`,
  2. order contains an assigned brand through `order_brand_groups` + `brand_members`,
  3. explicit `payment_proof.view_assigned` on that brand membership.
- Seller proof verify/reject now enforces explicit seeded permission + scope:
  - order contains assigned brand,
  - `payment_proof.verify` for verify action,
  - `payment_proof.reject` for reject action.
- Allowed and denied uploads/media reads/decisions are audited through `audit_events` best-effort writes.
- Existing upload validation remains in `core-storage`: size max 5MiB, image extension + magic-byte checks, SVG denial, filename traversal denial.
- Global request body limit was raised from 2MiB to `MAX_UPLOAD_BYTES + 64KiB` to allow 5MiB multipart proof upload. Remaining hardening need: route-specific limits so non-upload endpoints can keep a smaller cap.

## Tests / Validation

Focused validation command run:

```txt
cargo test -p core-storage -p core-db -p core-api
```

Observed result:

```txt
25 passed, 6 ignored
```

Covered checks include:

- local storage safe-read traversal rejection,
- local storage reading a previously stored private object,
- protected media helper denies non-seller/non-super-admin without DB lookup,
- seller media auth path does not silently allow without DB-backed assignment/permission evidence,
- permission query shape includes `order_brand_groups`, `brand_members`, and `brand_member_permissions`.

## Known Remaining Gaps

- No full checkout UI or commerce checkout flow implemented by design.
- No customer tracking-token media-read policy; guests remain denied.
- Seller verify/reject is no longer super-admin-only; it remains deny-by-default and requires explicit brand-scoped permission grants.
- Route-specific body limits are still needed.
- Integration tests requiring disposable PostgreSQL remain ignored unless `TEST_DATABASE_URL` is provided.
