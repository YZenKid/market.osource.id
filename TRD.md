# TRD — Self-Hosted Multi-Brand Marketplace

## 1. Ringkasan Teknis

Dokumen ini menjabarkan rancangan teknis untuk aplikasi **self-hosted multi-brand marketplace** berdasarkan `PRD.md`.

Produk memakai model **open-core**:

- **Base open source** berisi fitur inti marketplace yang dapat berjalan mandiri.
- **Package/module berbayar** menambahkan fitur lanjutan seperti promo, online payment, delivery, analytics, cloud backup, dan theme customization.

Sistem mendukung dua mode runtime:

1. **Desktop local-hosted** — aplikasi Tauri v2 mengelola bundled PostgreSQL, backend Axum, local storage, dan bundled `cloudflared` sidecar.
2. **VPS hosted** — backend Axum berjalan sebagai service di VPS, dikonfigurasi melalui web install panel, dengan PostgreSQL lokal/server dan storage lokal.

Customer mengakses marketplace melalui web responsive. Mobile native tidak termasuk scope awal.

## 2. Tujuan Teknis

- Menyediakan satu backend core yang sama untuk desktop dan VPS.
- Memisahkan business logic marketplace dari runtime desktop/VPS.
- Menjadikan Tauri sebagai shell, process supervisor, dan control panel, bukan tempat domain logic.
- Menyediakan install flow aman untuk desktop dan VPS.
- Menyediakan model RBAC brand-scoped yang aman untuk seller multi-brand.
- Mendukung order lintas brand dengan fulfillment/status per brand.
- Menyediakan manual transfer dengan upload bukti transfer.
- Menyediakan local storage yang aman dan siap dikembangkan ke S3-compatible adapter.
- Menyediakan package/module foundation tanpa membuat MVP terlalu kompleks.
- Menyiapkan update/migration strategy yang aman untuk self-hosted production data.

## 3. Stack Teknis

| Area | Teknologi |
| --- | --- |
| Desktop shell | Tauri v2 |
| Backend | Rust Axum |
| Database | PostgreSQL |
| Desktop database | Bundled PostgreSQL local instance |
| Frontend | SvelteKit |
| Styling | Tailwind CSS + TweakCN |
| Tunnel | Bundled `cloudflared` sidecar |
| Storage MVP | Local storage |
| Storage roadmap | S3-compatible adapter |
| DB access | `sqlx` direkomendasikan |
| Auth/session | Server-side session direkomendasikan |
| Password hashing | Argon2id |
| Package model | Capability registry + package migrations |

## 4. Prinsip Arsitektur

1. **Single backend core**  
   Desktop dan VPS memakai backend Axum yang sama.

2. **Runtime adapter, bukan fork logic**  
   Perbedaan desktop/VPS diisolasi di layer runtime adapter.

3. **Tauri bukan domain layer**  
   Tauri hanya mengelola proses, status, konfigurasi runtime, dan membuka dashboard.

4. **Backend authoritative**  
   Auth, RBAC, business rules, validasi order, dan file authorization wajib di backend.

5. **PostgreSQL sebagai source of truth**  
   Data commerce, install state, package state, setting, dan migration state disimpan di PostgreSQL.

6. **Local-first, public-safe**  
   Walau bisa berjalan lokal, sistem harus aman ketika diekspos via Cloudflare Tunnel.

7. **Open-core dengan boundary jelas**  
   Base open source harus tetap berguna tanpa package berbayar.

8. **No dynamic plugin runtime untuk MVP**  
   MVP cukup menyediakan package registry, feature gate, dan package migration boundary.

## 5. High-Level Architecture

```txt
Customer / Admin Browser
        |
        | HTTPS / Localhost
        v
Reverse Proxy / Cloudflare Tunnel / Local Loopback
        |
        v
Rust Axum Backend
        |
        +-- Storefront Routes
        +-- Admin Routes
        +-- Install Routes
        +-- System/Runtime Routes
        +-- Package Routes
        +-- Media Routes
        |
        +-- Domain Services
        +-- Auth/RBAC
        +-- Storage Provider
        +-- Runtime Adapter
        +-- Package Feature Gate
        |
        v
PostgreSQL + Local Storage
```

Desktop runtime:

```txt
Tauri App
  |
  +-- Desktop Process Manager
  |     +-- Bundled PostgreSQL
  |     +-- Axum Backend
  |     +-- Bundled cloudflared Sidecar
  |
  +-- Runtime Status UI
  +-- Setup Wizard Launcher
  +-- Admin Dashboard WebView / Browser Launcher
  +-- Log Viewer
```

VPS runtime:

```txt
VPS
  |
  +-- Axum Backend Service
  +-- PostgreSQL Service
  +-- Local Storage Path
  +-- Optional cloudflared Service
  +-- Optional Nginx/Caddy Reverse Proxy
  +-- Web Install Panel
```

## 6. Rekomendasi Struktur Repository

```txt
apps/
  backend/                  # Axum binary entrypoint
  desktop/                  # Tauri v2 desktop shell
  web/                      # SvelteKit app: storefront/admin/install

crates/
  core-domain/              # entities, value objects, domain rules
  core-app/                 # use cases/application services
  core-api/                 # Axum routes, DTOs, middleware, OpenAPI
  core-auth/                # auth, session, RBAC, CSRF, password hashing
  core-db/                  # sqlx repositories, transactions, migrations
  core-storage/             # StorageProvider trait + Local adapter
  core-runtime/             # config, paths, mode, health, version
  core-installer/           # preflight, setup, install lock, first admin
  core-packages/            # package registry, feature gate, package migrations
  sidecar-postgres/         # bundled PostgreSQL lifecycle adapter
  sidecar-cloudflared/      # cloudflared lifecycle/status adapter

migrations/
  core/                     # core DB migrations
  packages/                 # package migration folders

docs/
  deployment/
  security/
  packages/
```

Catatan: struktur ini dapat disederhanakan saat awal implementasi, tetapi boundary-nya tetap perlu dijaga.

## 7. Backend Module Boundaries

### 7.1 `core-domain`

Berisi domain model dan aturan inti.

Tidak boleh bergantung pada:

- Axum
- SQLx
- Tauri
- filesystem
- `cloudflared`
- HTTP/session implementation

Contoh tanggung jawab:

- Validasi stok tidak negatif.
- Produk draft tidak publish di storefront.
- Seller hanya boleh mengelola brand assigned.
- Order item menyimpan snapshot harga dan produk.
- Payment proof tidak otomatis mengubah order menjadi confirmed.

### 7.2 `core-app`

Berisi use case dan orchestration service.

Contoh service:

- `BrandService`
- `SellerAssignmentService`
- `CatalogService`
- `CartService`
- `CheckoutService`
- `OrderService`
- `PaymentProofService`
- `InstallService`
- `PackageService`

Authorization business-level wajib dilakukan di layer ini, bukan hanya middleware.

### 7.3 `core-db`

Berisi repository PostgreSQL, transaction boundary, dan migration runner.

Rekomendasi:

- Gunakan `sqlx`.
- Gunakan prepared statements/query macros.
- Gunakan transaction untuk checkout, order status update, setup, dan package activation.
- Pisahkan migration core dan package.

### 7.4 `core-api`

Berisi Axum router, request/response DTO, middleware, error mapping, dan OpenAPI export.

Tidak boleh berisi business rules besar.

### 7.5 `core-auth`

Berisi:

- Argon2id password hashing.
- Server-side session.
- CSRF protection.
- RBAC/permission checks.
- Rate limiting untuk login dan mutation sensitif.

### 7.6 `core-storage`

Berisi trait storage dan adapter.

MVP:

- `LocalStorageProvider`

Roadmap:

- `S3StorageProvider`

### 7.7 `core-runtime`

Berisi:

- `RuntimeMode`
- path resolution
- config loading
- secret resolution
- health checks
- version info
- runtime status

### 7.8 `core-installer`

Berisi:

- preflight desktop/VPS
- setup idempotency
- migration orchestration
- first super admin creation
- install lock
- rollback-safe setup state

### 7.9 `core-packages`

Berisi:

- package registry
- feature/capability gate
- package compatibility check
- package migration state
- package enable/disable state

## 8. Runtime Mode & Adapter

### 8.1 Runtime Mode

```rust
enum RuntimeMode {
    Desktop,
    Vps,
}
```

### 8.2 Runtime Adapter Contract

```rust
trait RuntimeAdapter {
    fn mode(&self) -> RuntimeMode;
    fn resolve_paths(&self) -> RuntimePaths;
    async fn preflight(&self) -> PreflightReport;
    async fn service_status(&self) -> RuntimeStatus;
}
```

### 8.3 Runtime Paths

Path tidak boleh hardcoded device-specific.

Desktop:

- config: OS app config dir
- data: OS app data dir
- logs: OS app log dir
- PostgreSQL data: app data dir
- storage: default app data dir atau user-selected path

VPS:

- config: env vars atau config file dengan permission ketat
- data/storage: path eksplisit dari install panel
- logs: service logs atau configured log path

## 9. Desktop Runtime Design

### 9.1 Tanggung Jawab Tauri

Tauri v2 bertanggung jawab untuk:

- menjalankan bundled PostgreSQL,
- menjalankan backend Axum,
- menjalankan `cloudflared` sidecar jika diaktifkan,
- melakukan preflight desktop,
- membaca status service,
- membuka setup/admin dashboard,
- menampilkan log ringkas,
- mengelola start/stop/restart service lokal.

Tauri tidak boleh:

- menjalankan business logic marketplace,
- bypass backend untuk menulis data commerce,
- mengekspos command native ke web customer,
- menyimpan secret di frontend bundle.

### 9.2 Bundled PostgreSQL

Desktop installer membundel PostgreSQL.

Requirements:

- Bind default ke `127.0.0.1`.
- Tidak listen ke `0.0.0.0`.
- Password database dibuat random saat setup.
- App runtime user memakai least-privilege DB user.
- Data directory persistent di app data dir.
- Port default memiliki fallback jika konflik.
- `initdb` berjalan hanya jika data directory belum ada.
- Start/stop dikelola oleh Tauri process manager.
- Log PostgreSQL ditulis ke log path aplikasi.
- Backup/restore command didokumentasikan.

### 9.3 Bundled `cloudflared`

Requirements:

- Sidecar dibundel bersama distribusi aplikasi.
- Tunnel opt-in, tidak aktif otomatis.
- Jika tunnel gagal, backend tetap berjalan lokal.
- Token/credential tunnel disimpan sebagai secret.
- UI hanya menampilkan token dalam bentuk masked.
- Tunnel hanya mengekspos HTTP app, bukan PostgreSQL/debug/internal admin port.

### 9.4 Desktop Startup Sequence

```txt
1. Tauri boot
2. Resolve runtime paths
3. Load desktop config + secrets
4. Preflight local ports/storage/binaries
5. Start bundled PostgreSQL jika belum aktif
6. Start Axum backend
7. Poll /health dan /ready
8. Jika tunnel enabled, start cloudflared
9. Tampilkan dashboard/status
```

## 10. VPS Runtime Design

### 10.1 VPS Service

VPS tidak memakai Tauri. Backend Axum berjalan sebagai service.

Deployment awal dapat berupa:

- binary + systemd,
- container,
- package manager script,
- manual service setup.

TRD ini merekomendasikan binary + systemd sebagai baseline production VPS.

### 10.2 VPS Install Panel

Install panel aktif hanya saat sistem belum configured.

Install panel melakukan:

- database connectivity check,
- storage writable check,
- base URL/domain validation,
- secret generation,
- migration execution,
- first super admin creation,
- install lock.

Setelah setup selesai:

- `/install/*` harus terkunci server-side,
- tidak boleh membuat super admin kedua,
- response boleh `404`, `403`, atau locked page tanpa detail sensitif.

### 10.3 VPS Reverse Proxy/Tunnel

VPS dapat memakai:

- Cloudflare Tunnel,
- Nginx,
- Caddy,
- reverse proxy lain.

Cloudflared tidak wajib untuk VPS jika operator memakai reverse proxy biasa.

## 11. Frontend Architecture

### 11.1 SvelteKit App

MVP direkomendasikan memakai satu SvelteKit app dengan route grouping:

```txt
src/routes/
  (storefront)/
    +layout.svelte
    +page.svelte
    products/
    brands/
    cart/
    checkout/
    order/

  (admin)/
    admin/
      +layout.svelte
      dashboard/
      brands/
      sellers/
      products/
      orders/
      settings/
      system/
      packages/

  (install)/
    install/
      +layout.svelte
      +page.svelte
```

### 11.2 Rendering Strategy

- Storefront: SSR/SPA hybrid sesuai kebutuhan.
- Admin: authenticated app shell.
- Install: hanya aktif saat installation state belum locked.

### 11.3 Styling

- Tailwind CSS sebagai utility foundation.
- TweakCN sebagai komponen/theme basis.
- Theme harus bisa dikembangkan menjadi package customization di roadmap.

### 11.4 Frontend Security Boundary

Frontend tidak boleh menjadi sumber kebenaran untuk:

- RBAC,
- brand assignment,
- order status validity,
- package activation,
- file authorization.

Semua validasi tersebut wajib di backend.

## 12. API Design

### 12.1 Route Groups

```txt
GET    /health
GET    /ready
GET    /version

/api/install/*
/api/auth/*
/api/storefront/*
/api/admin/*
/api/system/*
/api/packages/*
/media/*
```

### 12.2 Storefront API

Contoh endpoint:

```txt
GET  /api/storefront/settings
GET  /api/storefront/brands
GET  /api/storefront/brands/:slug
GET  /api/storefront/categories
GET  /api/storefront/products
GET  /api/storefront/products/:slug
POST /api/storefront/cart
PUT  /api/storefront/cart/:item_id
POST /api/storefront/checkout
GET  /api/storefront/orders/:tracking_token
POST /api/storefront/orders/:tracking_token/payment-proof
```

### 12.3 Admin API

Contoh endpoint:

```txt
POST /api/auth/login
POST /api/auth/logout
GET  /api/admin/me

GET    /api/admin/dashboard
GET    /api/admin/brands
POST   /api/admin/brands
PATCH  /api/admin/brands/:id

GET    /api/admin/sellers
POST   /api/admin/sellers
POST   /api/admin/sellers/:id/brand-assignments
DELETE /api/admin/sellers/:id/brand-assignments/:brand_id

GET    /api/admin/products
POST   /api/admin/products
PATCH  /api/admin/products/:id
POST   /api/admin/products/:id/images

GET    /api/admin/orders
GET    /api/admin/orders/:id
PATCH  /api/admin/orders/:id/status
PATCH  /api/admin/order-brand-groups/:id/status
POST   /api/admin/payment-proofs/:id/verify
POST   /api/admin/payment-proofs/:id/reject

GET    /api/admin/settings
PATCH  /api/admin/settings
GET    /api/admin/system/status
GET    /api/admin/packages
```

### 12.4 Install API

```txt
GET  /api/install/state
GET  /api/install/preflight
POST /api/install/setup
```

### 12.5 System API

```txt
GET  /api/system/health
GET  /api/system/runtime
GET  /api/system/tunnel
POST /api/system/tunnel/start
POST /api/system/tunnel/stop
GET  /api/system/logs
```

Desktop-only actions should be guarded by runtime mode.

## 13. Authentication & Authorization

### 13.1 Session Model

Rekomendasi MVP:

- Server-side sessions.
- Cookie `HttpOnly`.
- Cookie `Secure` saat HTTPS.
- `SameSite=Lax` atau `Strict` untuk admin.
- CSRF protection untuk mutation endpoint.

Session table menyimpan:

- session id hash,
- user id,
- expiry,
- created at,
- last seen,
- revoked at,
- user agent/IP metadata opsional.

### 13.2 Password Hashing

- Gunakan Argon2id.
- Salt unik per user.
- Cost parameter terdokumentasi.
- Password tidak pernah masuk log.

### 13.3 Roles

Role minimum:

- `SuperAdmin`
- `Seller`
- `Guest`

### 13.4 Brand-Scoped RBAC

Seller dapat memiliki banyak brand assignment melalui `brand_members`.

Aturan:

- SuperAdmin: akses semua data marketplace.
- Seller: hanya brand assigned.
- Guest: storefront dan checkout.

Semua query seller wajib difilter berdasarkan membership backend, bukan parameter client.

## 14. Data Model

### 14.1 Core Tables

Entitas minimum:

- `users`
- `roles`
- `sessions`
- `brands`
- `brand_members`
- `categories`
- `products`
- `product_variants`
- `product_images`
- `carts`
- `cart_items`
- `orders`
- `order_brand_groups`
- `order_items`
- `order_status_history`
- `payment_proofs`
- `marketplace_settings`
- `storage_settings`
- `tunnel_settings`
- `packages`
- `package_migrations`
- `installation_state`
- `audit_events`

### 14.2 Order Lintas Brand

Order lintas brand wajib mendukung status global dan status per brand.

Rekomendasi struktur:

```txt
orders
  id
  order_number
  public_tracking_token
  customer_name
  customer_contact
  shipping_address
  payment_status
  global_status
  subtotal_snapshot
  total_snapshot
  created_at

order_brand_groups
  id
  order_id
  brand_id
  fulfillment_status
  subtotal_snapshot
  created_at

order_items
  id
  order_id
  order_brand_group_id
  brand_id
  product_id nullable
  product_variant_id nullable
  product_name_snapshot
  variant_name_snapshot
  brand_name_snapshot
  sku_snapshot
  unit_price_snapshot
  quantity
  line_total_snapshot
```

Alasan:

- Satu order dapat dibayar sekali.
- Fulfillment setiap brand bisa berbeda status.
- Seller hanya melihat `order_brand_groups` dan `order_items` brand miliknya.

### 14.2.1 Mapping Status Order

Status produk pada PRD dipetakan menjadi tiga kelompok status teknis:

| PRD Status | `orders.payment_status` | `orders.global_status` | `order_brand_groups.fulfillment_status` | Catatan |
| --- | --- | --- | --- | --- |
| `pending` | `pending` | `open` | `not_ready` | Order dibuat, belum ada bukti transfer. |
| `waiting_payment_verification` | `waiting_verification` | `open` | `not_ready` | Bukti transfer diupload, menunggu verifikasi admin. |
| `confirmed` | `confirmed` | `confirmed` | `ready_to_process` | Pembayaran manual diverifikasi. |
| `processing` | `confirmed` | `in_progress` | `processing` | Seller/brand mulai memproses item. |
| `shipped` | `confirmed` | `in_progress` atau `partially_shipped` | `shipped` | Dapat berbeda antar brand. |
| `completed` | `confirmed` | `completed` | `completed` | Semua brand group selesai. |
| `cancelled` | `cancelled` atau status terakhir | `cancelled` | `cancelled` | Pembatalan dapat global atau per brand sesuai aturan bisnis. |

Aturan agregasi global:

- `global_status=completed` hanya jika semua `order_brand_groups.fulfillment_status=completed`.
- `global_status=in_progress` jika minimal satu brand group sedang diproses dan belum semua selesai.
- `global_status=partially_shipped` dapat digunakan jika sebagian brand group sudah shipped dan sebagian belum.
- Seller hanya boleh mengubah `fulfillment_status` brand group miliknya.
- Super Admin dapat mengubah status global dan status brand group sesuai kebutuhan operasional.

### 14.3 Payment Proofs

```txt
payment_proofs
  id
  order_id
  uploaded_by_user_id nullable
  file_object_id
  status
  note nullable
  verified_by_user_id nullable
  verified_at nullable
  rejected_by_user_id nullable
  rejected_at nullable
  created_at
```

Schema harus mendukung lebih dari satu proof per order, walau UI MVP dapat membatasi satu file aktif.

Status:

- `uploaded`
- `verified`
- `rejected`

### 14.4 File Objects

Disarankan menambahkan tabel file metadata generik:

```txt
file_objects
  id
  storage_provider
  bucket
  object_key
  original_filename
  mime_type
  size_bytes
  checksum
  visibility
  created_by_user_id nullable
  created_at
```

Visibility:

- `public`
- `authenticated`
- `private_admin`

Product image/logo dapat `public`. Payment proof harus private.

## 15. Database & Migration Strategy

### 15.1 Core Migrations

- Versioned.
- Transactional jika memungkinkan.
- Dijalankan pada setup/startup dengan migration lock.
- Tidak boleh menjalankan destructive migration tanpa explicit confirmation.
- Tidak mendukung downgrade otomatis untuk production.

### 15.2 Package Migrations

Package migrations terpisah dari core migrations.

Tabel:

```txt
package_migrations
  id
  package_id
  package_version
  migration_version
  checksum
  applied_at
```

Aturan:

- Package migration hanya dijalankan jika compatibility valid.
- Package migration harus tercatat terpisah.
- Package disabled tidak menghapus data secara otomatis.

### 15.3 Migration Lock

Gunakan advisory lock PostgreSQL atau tabel lock untuk mencegah dua proses menjalankan migrasi bersamaan.

## 16. Storage Design

### 16.1 Storage Provider Trait

```rust
trait StorageProvider {
    async fn put_object(&self, input: PutObjectInput) -> Result<StoredObject>;
    async fn get_object(&self, key: &str) -> Result<ObjectStream>;
    async fn delete_object(&self, key: &str) -> Result<()>;
    async fn public_url(&self, key: &str) -> Result<Option<String>>;
}
```

### 16.2 Local Storage MVP

Logical folders:

```txt
brand-logos/
product-images/
payment-proofs/
```

Rules:

- Jangan gunakan filename user sebagai path.
- Gunakan UUID/content hash.
- Cegah path traversal.
- Validasi MIME, extension, magic bytes, ukuran, dan dimensi.
- Tolak SVG untuk MVP.
- Pertimbangkan re-encoding gambar untuk menghapus metadata/payload.

### 16.3 Media Access

- Public media: brand logo dan product images published.
- Protected media: payment proofs.

Payment proof tidak boleh diserve sebagai static public file. Akses harus melalui route yang melakukan authorization.

## 17. Checkout & Payment Manual

### 17.1 Checkout Flow

```txt
1. Customer submit cart + contact + address
2. Backend validasi cart
3. Backend validasi stok
4. Backend buat order
5. Backend buat order_brand_groups
6. Backend snapshot order_items
7. Backend set payment_status=pending
8. Backend return order_number + tracking_token + instruksi transfer
```

### 17.2 Upload Bukti Transfer

```txt
1. Customer buka order via tracking_token
2. Customer upload proof
3. Backend validasi file
4. File disimpan private
5. payment_proofs dibuat
6. order.payment_status = waiting_payment_verification
7. Audit event dicatat
```

### 17.3 Verification

Admin memverifikasi bukti transfer.

Status disarankan:

- `pending`
- `waiting_payment_verification`
- `payment_rejected`
- `confirmed`

Proof tidak boleh otomatis mengkonfirmasi order.

## 18. Package/Open-Core System

### 18.1 MVP Package Model

MVP tidak menggunakan dynamic plugin runtime penuh.

MVP menyediakan:

- static/internal package registry,
- capability flags,
- package metadata table,
- package migration boundary,
- admin UI untuk melihat status package.

### 18.2 Package Metadata

```txt
packages
  id
  package_id
  name
  version
  core_version_range
  license_status
  enabled
  installed_at
  updated_at
```

Capability examples:

- `promo.voucher`
- `payment.online`
- `delivery.rate_lookup`
- `analytics.advanced`
- `backup.cloud`
- `theme.customization`

### 18.3 Feature Gate

```rust
trait FeatureGate {
    async fn is_enabled(&self, capability: &str) -> bool;
    async fn require(&self, capability: &str) -> Result<()>;
}
```

Rules:

- Endpoint package wajib melewati feature gate.
- UI hanya menyembunyikan fitur; backend tetap wajib menolak akses.
- Base open source tidak boleh bergantung pada package berbayar.
- Package disabled tidak boleh merusak base flow.

### 18.4 Paid Package Security

Package harus memiliki:

- package id,
- version,
- core compatibility range,
- required capabilities,
- required scopes,
- migration manifest,
- license/entitlement state jika relevan.

Jika package didistribusikan sebagai artefak binary/wasm/dynamic library di masa depan, perlu signature/checksum verification. Untuk MVP cukup siapkan metadata dan boundary.

## 19. Install & Setup Design

### 19.1 Installation State

```txt
installation_state
  id
  state
  installed_at nullable
  locked_at nullable
  core_version
```

State:

- `unconfigured`
- `configuring`
- `installed`
- `failed`

### 19.2 Setup Atomicity

Setup harus:

- mencegah double submit,
- membuat first admin sekali,
- menjalankan migration dengan lock,
- mengunci install panel setelah sukses,
- masuk state aman jika gagal.

### 19.3 Preflight Checks

Desktop:

- binary backend tersedia,
- PostgreSQL bundled tersedia,
- PostgreSQL dapat start,
- storage path writable,
- port tersedia,
- `cloudflared` binary tersedia,
- app data dir writable.

VPS:

- database connection valid,
- database privilege cukup,
- storage path writable,
- base URL valid,
- config writable,
- migration dapat dijalankan.

## 20. Runtime Config & Secret Management

### 20.1 Config Fields

```txt
runtime_mode
base_url
database_url_ref
storage_provider
storage_path
s3_config_optional
cloudflared_config_ref
session_secret_ref
encryption_key_ref
install_locked
```

### 20.2 Secret Storage

Desktop:

- gunakan OS keychain/secure storage jika tersedia,
- fallback encrypted local secret file dengan permission ketat.

VPS:

- environment variables atau config file permission `0600`,
- secret tidak tampil di UI/log/diagnostics.

Secret meliputi:

- database password/connection string,
- session signing key,
- encryption key,
- cloudflared token,
- S3 credentials,
- package license key,
- future webhook secret.

## 21. Security Requirements

### 21.1 Must-Have Controls

- Auth/RBAC enforced di backend.
- Argon2id password hashing.
- Server-side session dengan secure cookie.
- CSRF protection untuk cookie-based mutation.
- Rate limit untuk login, checkout, upload, order lookup, dan install panel.
- Install panel locked server-side setelah setup.
- PostgreSQL desktop bind ke localhost.
- Tunnel opt-in.
- CORS allowlist, bukan wildcard.
- Security headers minimal:
  - `Content-Security-Policy`,
  - `X-Content-Type-Options: nosniff`,
  - `Frame-Options` atau `frame-ancestors`,
  - `Referrer-Policy`,
  - `Permissions-Policy`.
- Upload validation ketat.
- Payment proof private.
- Secret redaction di UI/log/diagnostics.

### 21.2 Privacy Classification

| Kategori | Data |
| --- | --- |
| Public | nama produk, deskripsi produk, brand logo, product images published |
| Internal | stok, dashboard metrics, order summary |
| Sensitive | customer contact, alamat, payment proof, secrets, license keys |

### 21.3 Audit Log

Audit wajib untuk:

- login sukses/gagal,
- pembuatan super admin,
- seller-brand assignment,
- perubahan brand/produk,
- perubahan status order,
- upload/verifikasi/reject bukti transfer,
- perubahan setting storage/tunnel,
- install setup/lock,
- package enable/disable,
- migration core/package,
- akses/download payment proof.

Audit fields:

- actor user id,
- action,
- target entity,
- timestamp,
- IP/user agent jika tersedia,
- result,
- metadata aman tanpa secret.

## 22. Observability & Diagnostics

### 22.1 Health Endpoints

```txt
GET /health   # process alive
GET /ready    # db/storage/migration ready
GET /version  # app/core/package versions
```

### 22.2 Runtime Status

Admin/Tauri harus bisa melihat:

- backend status,
- database status,
- migration status,
- storage status,
- tunnel status,
- install state,
- package status.

### 22.3 Logging

Requirements:

- structured logging,
- request id,
- no secret in logs,
- no full customer address/payment proof URL in logs,
- desktop log viewer membaca log ringkas.

### 22.4 Diagnostics Export

Diagnostics export harus redacted by default:

- tidak ada secret,
- tidak ada payment proof,
- tidak ada customer full address,
- tidak ada connection string mentah.

## 23. Backup & Restore

### 23.1 MVP Minimum

MVP minimal menyediakan dokumentasi:

- backup PostgreSQL,
- backup local storage,
- restore PostgreSQL,
- restore local storage,
- backup sebelum upgrade.

### 23.2 Backup Security

Backup yang berisi customer data, payment proof, atau secret wajib dienkripsi.

### 23.3 Restore Validation

Restore harus memvalidasi:

- core version,
- schema version,
- package compatibility,
- storage path integrity.

## 24. Update Strategy

### 24.1 Versioning

Gunakan semantic versioning:

- app version,
- core API/schema version,
- package version,
- package compatibility range.

### 24.2 Desktop Update Flow

```txt
1. Check update
2. Tampilkan changelog + migration warning
3. Preflight backup warning
4. Stop backend
5. Stop cloudflared jika aktif
6. Backup config metadata minimal
7. Update binary/assets
8. Start PostgreSQL
9. Run migration dengan lock
10. Start backend
11. Health check
12. Start cloudflared jika sebelumnya aktif
```

### 24.3 VPS Update Flow

```txt
1. Operator update binary/package/container
2. Service start
3. Backend validate config
4. Backend validate package compatibility
5. Backend run migration dengan lock
6. Backend expose /ready jika sukses
```

### 24.4 Migration Safety

- No destructive migration tanpa confirmation.
- Backup warning untuk schema critical.
- Package incompatible harus ditandai sebelum core upgrade major.
- Downgrade DB tidak otomatis.

## 25. Testing Strategy

### 25.1 Unit Tests

- domain rules,
- status transition,
- checkout validation,
- RBAC helper,
- feature gate,
- storage path safety.

### 25.2 Integration Tests

- PostgreSQL repository,
- migrations,
- install setup,
- checkout transaction,
- order lintas brand,
- seller isolation,
- payment proof access control,
- package migration tracking.

### 25.3 API Tests

- auth/session,
- admin routes,
- storefront routes,
- install locked behavior,
- upload validation,
- CORS/security headers.

### 25.4 E2E Tests

- desktop setup wizard mock/runtime,
- VPS install panel,
- create brand/seller/product,
- customer checkout,
- upload proof,
- admin verify payment,
- seller view order brand-specific.

### 25.5 Security Negative Tests

- Seller A tidak bisa akses Brand B.
- Payment proof tidak bisa diakses tanpa auth.
- SVG berbahaya ditolak.
- Path traversal upload ditolak.
- Install setup tidak bisa dipakai ulang.
- Package disabled endpoint ditolak.

## 26. Performance Requirements

MVP target:

- katalog kecil-menengah,
- pagination untuk produk/order,
- index untuk slug, brand_id, product status, order status, created_at,
- lazy loading gambar,
- upload size limit,
- query dashboard tidak full scan besar.

Index awal yang disarankan:

- `brands(slug)` unique
- `products(brand_id, status)`
- `products(slug)` unique/scoped
- `product_variants(product_id)`
- `brand_members(user_id, brand_id)` unique
- `orders(order_number)` unique
- `orders(public_tracking_token)` unique
- `order_brand_groups(order_id, brand_id)`
- `order_items(order_brand_group_id)`
- `payment_proofs(order_id)`

## 27. Deployment Matrix

| Area | Desktop | VPS |
| --- | --- | --- |
| Process manager | Tauri | systemd/supervisor/container |
| PostgreSQL | Bundled | OS service/local server |
| cloudflared | Bundled optional | Optional |
| Storage | App data/user path | Server path |
| Config | App config dir + secret store | env/config file |
| Setup | Tauri/WebView wizard | Web install panel |
| Logs | Tauri log viewer | service logs/admin diagnostics |
| Update | desktop updater/manual | service/package update |
| Recommended production | small/local deployments | stable public deployment |

## 28. Technical Risks & Mitigations

| Risiko | Dampak | Mitigasi |
| --- | --- | --- |
| Bundled PostgreSQL gagal start | Desktop setup gagal | preflight, log jelas, port fallback, localhost bind |
| Data dir PostgreSQL corrupt | Data hilang | backup docs, safe shutdown, restore guide |
| Tunnel expose service tidak aman | Internet attack surface | tunnel opt-in, auth/RBAC backend, CORS allowlist |
| Install panel reusable | takeover setup | install lock server-side, setup atomic |
| RBAC brand bocor | seller lihat data brand lain | service-layer auth, query scoped, negative tests |
| Order lintas brand kompleks | seller/order status rancu | `order_brand_groups` sejak awal |
| Payment proof public | privacy leak | protected route, private storage visibility |
| Upload berbahaya | XSS/path traversal | magic bytes, UUID path, reject SVG, size limit |
| Package system terlalu kompleks | MVP lambat | capability registry dulu, no dynamic plugin MVP |
| Package migration rusak | upgrade gagal | package migration table, compatibility check |
| Secret bocor | takeover DB/tunnel/package | keychain/env, masking, redaction |
| Update destructive | data production rusak | migration lock, backup warning, no auto destructive migration |

## 29. Keputusan Teknis Terkunci

- Backend utama memakai Rust Axum.
- Desktop memakai Tauri v2 sebagai shell/control panel.
- Desktop membundel PostgreSQL.
- Distribusi membundel `cloudflared` sidecar.
- Database utama PostgreSQL.
- Frontend memakai SvelteKit.
- Styling memakai Tailwind CSS + TweakCN.
- Storage aktif MVP adalah local storage.
- S3-compatible adapter masuk roadmap setelah MVP.
- Seller dibuat oleh Super Admin, bukan self-registration.
- Satu seller dapat mengelola banyak brand.
- Satu domain merepresentasikan satu marketplace.
- Order boleh lintas brand.
- Order lintas brand direkomendasikan memakai `order_brand_groups`.
- Manual transfer memakai upload bukti transfer.
- Package model MVP berupa capability registry + package migration boundary, bukan dynamic plugin runtime.

## 30. Pertanyaan Teknis Tersisa

- Apakah UI MVP membatasi satu bukti transfer aktif per order, meski schema mendukung banyak file?
- Apakah seller boleh melihat bukti transfer order lintas brand, atau hanya Super Admin?
- Format distribusi desktop awal: `.deb`, `.AppImage`, `.msi`, `.dmg`, atau prioritas platform tertentu?
- Apakah VPS install akan didukung lewat binary + systemd saja, atau container juga sejak awal?
- Apakah SvelteKit dan Axum diserve dari satu origin/backend, atau SvelteKit dipisahkan saat VPS?
- Apakah package berbayar akan didistribusikan sebagai source/private crate, binary artifact, atau remote entitlement pada fase awal monetisasi?

## 31. MVP Technical Acceptance Criteria

- Desktop app dapat menjalankan bundled PostgreSQL, Axum backend, dan optional `cloudflared`.
- VPS install panel dapat mengunci setup setelah super admin dibuat.
- Backend memiliki health/readiness/version endpoints.
- Super Admin dapat membuat seller dan assign ke banyak brand.
- Seller tidak dapat mengakses brand/order/file brand lain melalui API langsung.
- Storefront responsive dapat browse produk, cart, checkout, dan upload proof.
- Order lintas brand tersimpan dengan grouping per brand.
- Payment proof tersimpan private dan hanya dapat diakses dengan authorization.
- Local storage menolak upload invalid dan path traversal.
- Package registry menampilkan status package dan menolak fitur disabled server-side.
- Core migration dan package migration terpisah.
- Secret tidak muncul di UI, log, diagnostics, atau frontend bundle.
- Tunnel aktif tidak mengekspos PostgreSQL atau endpoint internal/debug.
