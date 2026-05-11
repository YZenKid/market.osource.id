# Gate E Commerce MVP Foundation Evidence

Date: 2026-05-10

## Scope Implemented

- Added `core-db` commerce repository primitives for:
  - brand create/list,
  - category create/list foundation,
  - product creation with a default/internal variant,
  - admin product listing,
  - storefront published product list/detail,
  - skeletal guest cart create/upsert item primitives,
  - super-admin brand member assignment,
  - checkout from explicit submitted variant/quantity items.
- Checkout uses `product_variants.price` as the canonical price source and snapshots brand/product/variant/SKU/unit price/quantity/line total into `order_items`.
- Checkout creates one order with canonical default statuses:
  - `orders.payment_status = pending`,
  - `orders.global_status = open`,
  - `order_brand_groups.fulfillment_status = not_ready`.
- Checkout groups submitted items by brand and creates one `order_brand_groups` row per brand.
- Checkout validates:
  - non-empty submitted items,
  - quantity greater than zero,
  - active variant + published product + active brand,
  - stock greater than zero,
  - requested quantity not greater than current stock.
- Added API routes:
  - `GET /api/admin/brands`
  - `POST /api/admin/brands`
  - `GET /api/admin/products`
  - `POST /api/admin/products`
  - `POST /api/admin/brand-members`
  - `GET /api/storefront/products`
  - `GET /api/storefront/products/{id}`
  - `POST /api/storefront/checkout`
  - `GET /api/storefront/orders/{tracking_token}` (safe order summary)

## Abuse-hardening slice (bounded)

- Added route-specific in-memory throttling keys for:
  - checkout (`storefront_checkout:{client_identity}`),
  - order lookup by tracking token (`storefront_order_lookup:{client_identity}:{tracking_token}`),
  - payment-proof upload (`payment_proof_upload:{client_identity}:{tracking_token}`).
- Kept existing login/install throttles and ensured checkout + payment-proof are now also throttled.
- Added minimal safe order lookup endpoint `GET /api/storefront/orders/{tracking_token}` that returns only:
  - `order_number`,
  - `payment_status`,
  - `global_status`,
  - `total_snapshot`.
- Tightened body limits by endpoint:
  - checkout route has a specific JSON body limit (`64 KiB`),
  - payment-proof upload route has its own multipart ceiling (`MAX_UPLOAD_BYTES + envelope allowance`),
  - global body limit remains as a broad cap for non-overridden routes.

## Security / Boundary Notes

- Admin commerce routes require `AuthenticatedActor` and `require_super_admin` for now.
- Seller-scoped mutation is intentionally not enabled beyond skeletal `brand_members` assignment foundation; backend remains authoritative.
- Storefront product listing/detail only returns active brands, published products, and active variants.
- Checkout now uses transaction-safe stock reservation/decrement semantics with PostgreSQL row locking (`FOR UPDATE`) on `product_variants` during order creation.
- Stock decrement is performed atomically in the same checkout transaction, and the transaction aborts on overstock to prevent partial writes.

## Stock Safety Semantics

- Checkout decrements `product_variants.stock` immediately at order creation time (reservation-by-decrement model).
- Decrement and order snapshot inserts are wrapped in one PostgreSQL transaction.
- Variant rows are locked with `FOR UPDATE`, and decrement only succeeds when `stock >= requested_quantity`.
- Concurrent overstock attempts are rejected (`Overstock`) and stock is never allowed to go negative (enforced by query condition + table check constraint `stock >= 0`).

## Tests / Validation

Focused validation command run:

```txt
cargo fmt --all && cargo test -p core-db -p core-api
```

Observed result:

```txt
34 passed, 8 ignored
```

Covered Gate E checks include:

- multi-brand checkout grouping helper aggregates line totals per brand,
- checkout status defaults match ERD canonical values (`pending`, `open`, `not_ready`),
- checkout integration test: successful checkout decrements stock and persists order snapshots,
- checkout integration test: concurrent checkouts against the same variant reject overstock and preserve non-negative stock,
- existing API/auth/router tests still pass after admin/storefront route wiring.
- route-specific throttle key tests for storefront checkout/order lookup and payment-proof upload.

## Later Runtime Smoke Addendum

After the initial Gate E foundation slice, the commerce routes were revalidated through the split-origin runtime path using published Docker host ports `8300` (web) and `8301` (backend).

Observed smoke/E2E results:

- `GET /api/storefront/products` from origin `http://127.0.0.1:8300` succeeded through explicit credentialed CORS.
- Minimal smoke data was created through the existing admin APIs:
  - one brand via `POST /api/admin/brands`,
  - one product with one default variant via `POST /api/admin/products`.
- A successful checkout E2E then completed via `POST /api/storefront/checkout` with one submitted variant and quantity `2`.
- The checkout response returned HTTP `201` with:
  - canonical `payment_status = pending`,
  - canonical `global_status = open`,
  - one `order_brand_groups` row with `fulfillment_status = not_ready`,
  - line-item snapshots for brand/product/variant/SKU/unit price/quantity/line total.
- Safe order lookup via `GET /api/storefront/orders/{tracking_token}` then succeeded for the returned public tracking token.

Successful checkout smoke snapshot:

```txt
order_number: ORD-8E261FD382AC
public_tracking_token: 9dab4d2e-1106-45b3-82f3-e0eaec1cb004
payment_status: pending
global_status: open
total_snapshot: 30000.00
```

## Known Remaining Gaps

- No advanced search/filter/sort.
- No polished UI or E2E flow by design.
- Browser-executed visual E2E remains blocked by the current environment because Chrome/Chromium is unavailable for Playwright, but API-level checkout success and order lookup have been proven through runtime smoke.
- Status-aware stock release/restock flows (e.g., cancellation/refund policies) are not implemented yet.
- No seller-scoped product mutation yet; current admin CRUD foundation is super-admin-only.
- PostgreSQL-backed repository integration tests remain a future step unless an isolated `TEST_DATABASE_URL` is provided.
- Rate limiting is still single-process in-memory (not distributed/proxy-trust-aware).
- Body limiting for multipart still depends on framework layering and storage validation; there is no streaming early-abort parser yet.
