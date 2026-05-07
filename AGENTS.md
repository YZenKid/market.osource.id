# AGENTS.md — market.osource.id

Panduan untuk AI/coding agent yang bekerja di repository ini.

Repository ini sedang berada pada fase perencanaan dan dokumentasi. Implementasi aplikasi belum dimulai. Semua perubahan harus mengikuti dokumen otoritatif di root repository.

## 1. Dokumen Otoritatif

Baca dokumen berikut sebelum membuat keputusan produk, arsitektur, data model, atau implementasi:

1. [`README.md`](./README.md) — ringkasan project, status, stack, roadmap.
2. [`PRD.md`](./PRD.md) — kebutuhan produk dan scope MVP.
3. [`TRD.md`](./TRD.md) — rancangan teknis, arsitektur, runtime, security, deployment.
4. [`ERD.md`](./ERD.md) — entity relationship diagram, relasi data, index, status canonical.

Prioritas saat ada konflik:

1. Instruksi user terbaru.
2. `AGENTS.md`.
3. `PRD.md` untuk keputusan produk/scope.
4. `TRD.md` untuk keputusan teknis/arsitektur.
5. `ERD.md` untuk data model dan relasi.
6. `README.md` untuk ringkasan publik.

Jika menemukan konflik antar dokumen, jangan diam-diam memilih salah satu. Update dokumen terkait agar kembali sinkron atau tanyakan ke user jika keputusan berdampak besar.

## 2. Ringkasan Project

`market.osource.id` adalah **self-hosted, installable, multi-brand marketplace platform**.

Produk ini mendukung dua mode deployment:

- **Desktop local-hosted**: Tauri v2 menjalankan bundled PostgreSQL, Rust Axum backend, local storage, dan bundled `cloudflared` sidecar.
- **VPS hosted**: Rust Axum backend berjalan sebagai service di VPS, dikonfigurasi lewat web install panel.

Model produk:

- **Open-core**.
- Base marketplace open source.
- Package/module berbayar untuk fitur lanjutan seperti promo, online payment, delivery, analytics, backup/cloud sync, dan theme customization.

Target awal:

- Web responsive.
- Desktop/VPS installable.
- Mobile native bukan scope awal.
- Bukan SaaS hosted marketplace.
- Tidak ada self-service seller registration.

## 3. Stack Tetap

Jangan mengganti stack tanpa instruksi eksplisit user.

| Area | Stack |
| --- | --- |
| Desktop | Tauri v2 |
| Backend | Rust Axum |
| Database | PostgreSQL |
| Desktop DB | Bundled PostgreSQL local instance |
| Frontend | SvelteKit |
| Styling | Tailwind CSS + TweakCN |
| Tunnel | Bundled `cloudflared` sidecar |
| Storage MVP | Local storage |
| Storage roadmap | S3-compatible adapter |
| DB access | `sqlx` direkomendasikan |
| Auth/session | Server-side session direkomendasikan |
| Password hashing | Argon2id |

## 4. Keputusan Produk yang Sudah Dikunci

- Satu instalasi = satu marketplace.
- Satu domain = satu marketplace.
- Satu marketplace dapat memiliki banyak brand.
- Seller dibuat oleh Super Admin.
- Seller tidak mendaftar sendiri.
- Satu seller dapat mengelola banyak brand via `brand_members`.
- Produk berada di bawah brand.
- Produk dapat memiliki category dan variant.
- Satu order boleh berisi produk lintas brand.
- Order lintas brand memakai `order_brand_groups` untuk status fulfillment per brand.
- Checkout MVP menggunakan manual transfer.
- Customer dapat upload bukti transfer.
- UI MVP upload bukti transfer memakai satu file aktif per order; schema boleh tetap multi-proof untuk histori/ekstensi.
- Payment proof tidak otomatis membuat order confirmed.
- Payment proof dilindungi RBAC backend; Super Admin dapat melihat semua proof, seller default hanya melihat status pembayaran/order brand assigned sampai permission eksplisit untuk file proof ditambahkan.
- Storage aktif MVP adalah local storage.
- S3-compatible adapter adalah roadmap setelah MVP.
- Desktop installer membundel PostgreSQL.
- Distribusi aplikasi membundel `cloudflared` sidecar.
- Desktop installer menargetkan Windows, macOS, dan Linux sejak awal.
- VPS deployment baseline mendukung binary + systemd dan container sejak awal.
- Package berbayar diarahkan ke remote entitlement; base open source tetap berjalan tanpa paid entitlement.
- Package/module MVP hanya capability registry + migration boundary, bukan dynamic plugin runtime penuh.

## 5. Prinsip Arsitektur Wajib

Ikuti prinsip dari `TRD.md`:

- Single backend core untuk desktop dan VPS.
- Perbedaan desktop/VPS berada di runtime adapter, bukan domain logic.
- Tauri adalah shell/process supervisor/control panel, bukan domain layer.
- Backend Axum adalah sumber kebenaran untuk auth, RBAC, order, storage auth, package gate, dan business rules.
- PostgreSQL adalah source of truth.
- Local-first tetapi tetap public-safe ketika diekspos lewat tunnel.
- Base open source harus tetap berjalan tanpa package berbayar.
- Jangan membuat dynamic plugin runtime untuk MVP.

## 6. Struktur Project Target

Saat implementasi dimulai, ikuti struktur target di `TRD.md`:

```txt
apps/
  backend/                  # Axum binary entrypoint
  desktop/                  # Tauri v2 desktop shell
  web/                      # SvelteKit storefront/admin/install

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
  core/
  packages/

docs/
  deployment/
  security/
  packages/
```

Boundary rules:

- `apps/backend` hanya entrypoint/composition root.
- `apps/desktop` tidak boleh berisi marketplace business logic.
- `apps/web` tidak boleh menjadi sumber kebenaran authorization/status/package activation.
- `core-domain` tidak boleh depend ke Axum, SQLx, Tauri, filesystem, atau sidecar.
- `core-api` tidak boleh bypass `core-app` untuk mutation kritikal.
- `core-db` tidak boleh depend ke `core-api` atau frontend types.
- `sidecar-*` hanya untuk runtime/desktop adapter.

## 7. Data Model & Status Canonical

Data model harus mengikuti `ERD.md`.

Entitas penting:

- `users`
- `roles`
- `sessions`
- `brands`
- `brand_members`
- `categories`
- `products`
- `product_variants`
- `product_images`
- `file_objects`
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

Status canonical:

- `orders.payment_status`: `pending`, `waiting_payment_verification`, `confirmed`, `rejected`, `cancelled`.
- `orders.global_status`: `open`, `confirmed`, `in_progress`, `partially_shipped`, `completed`, `cancelled`.
- `order_brand_groups.fulfillment_status`: `not_ready`, `ready_to_process`, `processing`, `shipped`, `completed`, `cancelled`.
- `payment_proofs.status`: `uploaded`, `verified`, `rejected`.

Jangan memperkenalkan status baru tanpa update PRD/TRD/ERD.

## 8. Security Requirements

Semua implementasi harus menganggap app bisa diekspos ke internet via `cloudflared` atau reverse proxy.

Wajib:

- Auth/RBAC enforced di backend.
- Brand-scoped authorization untuk seller.
- Password hashing memakai Argon2id.
- Server-side session.
- CSRF protection untuk cookie-based mutation.
- Rate limit untuk login, checkout, upload, order lookup, dan install panel.
- Install panel locked server-side setelah setup.
- PostgreSQL desktop bind ke `127.0.0.1`.
- Tunnel opt-in, tidak aktif otomatis.
- CORS allowlist, bukan wildcard.
- Upload validation: size, MIME, extension, magic bytes, dimensi.
- Tolak SVG untuk MVP kecuali ada sanitasi ketat.
- Payment proof private/protected, tidak diserve sebagai public static file.
- Secret tidak boleh tampil di UI, log, diagnostics, crash report, atau frontend bundle.
- Audit log untuk event sensitif.

Jika task menyentuh auth, RBAC, uploads, payment proof, tunnel, secrets, install panel, package activation, atau customer data, lakukan review security terhadap perubahan.

## 9. Storage Rules

- MVP memakai local storage.
- Semua file metadata dicatat di `file_objects`.
- Product image/logo boleh public saat published.
- Payment proof wajib private.
- Jangan gunakan filename user sebagai filesystem path.
- Gunakan UUID/content hash untuk object key.
- Cegah path traversal dan overwrite.
- S3-compatible adapter adalah roadmap, jangan aktifkan sebagai default MVP.

## 10. Package/Open-Core Rules

Base open source harus tetap berguna tanpa package berbayar.

MVP package system hanya:

- package registry,
- capability flags,
- feature gate,
- package migration state,
- admin status UI.

Jangan membuat dynamic plugin loader, arbitrary code execution, atau unrestricted package DB/filesystem access pada MVP.

Package capabilities contoh:

- `promo.voucher`
- `payment.online`
- `delivery.rate_lookup`
- `analytics.advanced`
- `backup.cloud`
- `theme.customization`

Endpoint package/fitur berbayar harus dicek server-side melalui feature gate.

## 11. Frontend/UI Rules

Frontend memakai SvelteKit + Tailwind + TweakCN.

Route grouping target:

```txt
src/routes/
  (storefront)/
  (admin)/
  (install)/
```

UI harus responsive untuk customer web.

Untuk UI substantial:

- Ikuti PRD/TRD.
- Jaga aksesibilitas: semantic headings, labels, keyboard, focus visible, contrast.
- Jangan mengandalkan UI untuk authorization.
- Admin dashboard minimal usable di desktop/tablet.
- Checkout harus usable di mobile browser.

## 12. Testing Expectations

Saat kode sudah ada, perubahan non-trivial harus disertai test sesuai area:

- Unit test: domain rules, status transition, feature gate, storage validation.
- Integration test: repository, migration, checkout, order lintas brand, seller isolation.
- API test: auth/session, install lock, upload validation, package disabled route.
- E2E test: setup, create brand/seller/product, checkout, upload proof, admin verify.
- Negative security test: seller A tidak bisa akses brand/order/file seller B.

Jika test belum tersedia, tambahkan minimal test untuk behavior baru atau dokumentasikan gap-nya.

## 13. Documentation Rules

Update dokumen saat mengubah keputusan:

- Produk/scope/user flow → `PRD.md`.
- Arsitektur/runtime/security/deployment → `TRD.md`.
- Entity/relasi/index/status → `ERD.md`.
- Ringkasan publik/project status → `README.md`.
- Instruksi agent/contributor AI → `AGENTS.md`.

Jangan mengubah data model, status order, auth model, package boundary, atau deployment assumption hanya di kode tanpa update dokumen.

## 14. Git & Commit Rules

- Commit hanya perubahan yang relevan dengan task.
- Jangan commit secrets, `.env`, token, credential, database dump, build artifacts, atau generated/vendor files tanpa persetujuan eksplisit.
- Jangan force push kecuali user eksplisit meminta dan memahami risikonya.
- Jangan push otomatis kecuali user meminta.
- Untuk dokumentasi, gunakan commit subject seperti `docs: ...`.

## 15. Open Questions Saat Ini

Lihat PRD/TRD/ERD untuk daftar terbaru. Pertanyaan yang masih perlu dikunci sebelum implementasi detail:

- Strategi update aplikasi desktop dan migrasi database setelah user punya data production.
- Detail format distribusi desktop per OS: Windows `.msi`/`.exe`, macOS `.dmg`, Linux `.deb`/AppImage/RPM.
- Detail privacy payload dan grace period remote entitlement.
- Apakah SvelteKit dan Axum diserve dari satu origin/backend, atau SvelteKit dipisahkan saat VPS.
- Apakah category wajib scoped per brand atau ada global marketplace category pada fase lanjut.
- Apakah product price disimpan hanya di variant, atau product juga punya default price jika tanpa variant.
