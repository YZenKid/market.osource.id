# market.osource.id

Self-hosted, installable, multi-brand marketplace platform.

`market.osource.id` dirancang sebagai marketplace yang bisa dijalankan sendiri oleh operator, baik dari desktop lokal maupun dari VPS. Customer mengakses marketplace melalui web responsive, sementara operator mengelola brand, seller, produk, order, storage, tunnel, dan package dari dashboard admin.

> Status: Phase 0–1 scaffold started. Repository now contains authoritative planning docs plus initial Rust Axum workspace, SvelteKit web skeleton, Tauri desktop shell skeleton, migration skeleton, and VPS/container deployment baseline. Production features are not complete yet.

## Konsep Produk

Project ini memakai pendekatan **open-core self-hosted marketplace**:

- **Base open source** menyediakan fitur inti untuk menjalankan marketplace mandiri.
- **Package/module berbayar** dapat menambahkan fitur lanjutan seperti promo, online payment, delivery, advanced analytics, cloud backup, dan theme customization.

Target awal bukan SaaS hosted marketplace. Setiap instalasi adalah milik operator sendiri.

## Mode Deployment

### 1. Desktop Local-Hosted

Aplikasi desktop menjalankan marketplace dari mesin lokal.

Komponen utama:

- Tauri v2 desktop shell
- Rust Axum backend
- Bundled PostgreSQL
- Local storage
- Bundled `cloudflared` sidecar
- SvelteKit storefront/admin/install UI

Mode ini cocok untuk operator kecil, pilot internal, demo, toko lokal, atau deployment yang ingin berjalan dari komputer sendiri.

### 2. VPS Hosted

Marketplace dijalankan di VPS dan dikonfigurasi melalui web install panel.

Komponen utama:

- Rust Axum backend service
- PostgreSQL lokal/server
- Local storage di VPS
- SvelteKit web UI
- Optional `cloudflared` atau reverse proxy seperti Nginx/Caddy

Mode ini direkomendasikan untuk deployment publik yang membutuhkan uptime lebih stabil.

## Stack

| Area | Stack |
| --- | --- |
| Desktop | Tauri v2 |
| Backend | Rust Axum |
| Database | PostgreSQL |
| Frontend | SvelteKit |
| Styling | Tailwind CSS + TweakCN |
| Tunnel | `cloudflared` sidecar |
| Storage MVP | Local storage |
| Storage roadmap | S3-compatible adapter |
| Package model | Open-core package/module system |

## Fitur Base Open Source

Rencana fitur base:

- Desktop local-hosted installer
- VPS web install panel
- Multi-brand marketplace
- Seller dibuat oleh Super Admin
- Satu seller dapat mengelola banyak brand
- Satu domain merepresentasikan satu marketplace
- Category, product, variant, dan inventory basic
- Storefront web responsive
- Admin dashboard basic
- Cart dan checkout manual transfer
- Upload bukti transfer
- Order lintas brand
- Status fulfillment per brand/sub-order
- Local storage untuk logo, gambar produk, dan bukti transfer
- Bundled PostgreSQL untuk desktop
- Bundled `cloudflared` sidecar
- Package/module awareness di admin settings

## Package Berbayar yang Direncanakan

Contoh package/module berbayar:

- **Promo Package** — voucher, campaign discount, bundle discount, flash sale.
- **Online Payment Package** — payment gateway, webhook, reconciliation, status automation.
- **Delivery Package** — ongkir otomatis, integrasi kurir, tracking shipment.
- **Advanced Analytics Package** — sales report, brand/product performance, export report.
- **Backup & Cloud Sync Package** — scheduled backup, encrypted remote backup, restore assistant.
- **Theme/Customization Package** — theme builder, custom storefront section, brand visual customization.

Base open source harus tetap dapat berjalan tanpa package berbayar.

## Arsitektur Ringkas

```txt
Customer / Admin Browser
        |
        | HTTPS / Localhost
        v
Cloudflare Tunnel / Reverse Proxy / Local Loopback
        |
        v
Rust Axum Backend
        |
        +-- Storefront API
        +-- Admin API
        +-- Install API
        +-- System Runtime API
        +-- Package API
        +-- Media API
        |
        v
PostgreSQL + Local Storage
```

Desktop runtime:

```txt
Tauri App
  |
  +-- Bundled PostgreSQL
  +-- Axum Backend
  +-- Bundled cloudflared Sidecar
  +-- Runtime Status UI
  +-- Setup/Admin Launcher
```

## Dokumentasi

- [`PRD.md`](./PRD.md) — Product Requirements Document.
- [`TRD.md`](./TRD.md) — Technical Requirements Document.
- [`ERD.md`](./ERD.md) — Entity Relationship Diagram dan rancangan data model.
- [`DESIGN.md`](./DESIGN.md) — Project-local design system and UI guidance.
- [`AGENTS.md`](./AGENTS.md) — Contributor/AI agent constraints.
- [`.opencode/plans/20260507-2054-production-ready-roadmap.md`](./.opencode/plans/20260507-2054-production-ready-roadmap.md) — Production-ready implementation roadmap.

## Keputusan Produk yang Sudah Dikunci

- Desktop installer membundel PostgreSQL lokal.
- Distribusi aplikasi membundel `cloudflared` sebagai sidecar.
- MVP checkout memakai manual transfer dengan upload bukti transfer.
- UI MVP upload bukti transfer memakai satu file aktif per order; schema tetap dapat menyimpan histori/multi-proof.
- Payment proof private dan akses file proof dikendalikan RBAC backend; seller default hanya melihat status pembayaran/order brand assigned. Permission canonical: `payment_proof.view_assigned`, `payment_proof.verify`, dan `payment_proof.reject`; akses file proof seller wajib brand/order-scoped dan diaudit.
- Satu order boleh berisi produk lintas brand.
- Order lintas brand memakai status global + fulfillment status per brand/sub-order.
- Storage aktif MVP adalah local storage.
- Seller tidak mendaftar sendiri; akun seller dibuat oleh Super Admin.
- Satu seller dapat mengelola banyak brand melalui assignment.
- Satu domain merepresentasikan satu marketplace.
- Produk memakai model open-core: base open source + package/module berbayar.
- VPS baseline mendukung binary + systemd dan container sejak awal.
- Desktop installer menargetkan Windows, macOS, dan Linux sejak awal.
- Package berbayar diarahkan ke remote entitlement; base open source tetap berjalan tanpa paid entitlement.

## Scaffold Saat Ini

Phase 0–1 scaffold yang sudah ada:

- `Cargo.toml`, `Cargo.lock`, `apps/backend`, dan `crates/*` untuk workspace Rust/Axum awal.
- `migrations/core` dan `migrations/packages` untuk batas migration core/package.
- `apps/web` untuk SvelteKit + Tailwind skeleton dengan route groups storefront, admin, dan install panel.
- `apps/desktop/src-tauri` untuk Tauri v2 desktop shell skeleton.
- `Dockerfile`, `docker-compose.yml`, dan `docs/deployment/` untuk baseline VPS/container/systemd.

Known gaps:

- Host saat ini belum punya `rustc`/`cargo`; validasi Rust dilakukan via Docker `rust:1.88`.
- Non-desktop Rust workspace sudah build/test/fmt/clippy via Docker; full Tauri desktop build masih butuh dependency system Linux/Tauri dan smoke test per OS.
- Web skeleton sudah `npm run check` dan `npm run build`.
- Backend route, install lock, auth/session/CSRF/RBAC penuh, catalog, checkout, protected media, sidecar supervisor, dan real package entitlement belum diimplementasikan.

## Roadmap Ringkas

### Phase 0 — Foundation

- Struktur monorepo/app
- Axum backend skeleton
- SvelteKit storefront/admin/install skeleton
- Tauri control panel skeleton
- PostgreSQL runtime desktop
- `cloudflared` sidecar runtime
- Migration system
- Runtime config
- Health check
- Package registry basic

### Phase 1 — Installable MVP

- Desktop setup wizard
- VPS install panel
- Super Admin creation
- Brand/seller/product management
- Storefront basic
- Cart dan checkout manual
- Upload bukti transfer
- Order lintas brand
- Local storage upload
- Tunnel status/config
- Package awareness di settings

### Phase 2 — Operational Readiness

- Backup/restore flow
- Log viewer
- Diagnostics export
- Import/export produk
- Notification basic
- RBAC hardening
- Update/migration safety

### Phase 3+ — Commerce Expansion

- Promo package
- Online payment package
- Delivery package
- S3-compatible storage adapter
- Advanced analytics
- Theme customization
- Package distribution/update mechanism

## Development Status

Phase 0–1 scaffold sudah dimulai dan commit foundation awal sudah tersedia. Repository saat ini berisi:

- dokumen otoritatif: `README.md`, `PRD.md`, `TRD.md`, `ERD.md`, `DESIGN.md`, `AGENTS.md`;
- planning/evidence release di `.opencode/`;
- Rust workspace untuk Axum backend dan `crates/core-*`;
- SvelteKit web skeleton untuk storefront/admin/install;
- Tauri desktop shell skeleton;
- migration skeleton;
- baseline VPS container/systemd docs.

Implementasi production behavior masih berjalan bertahap mengikuti release gate plan. Backend route, install lock, auth/session/CSRF/RBAC penuh, catalog, checkout, protected media, sidecar supervisor, dan entitlement nyata belum selesai.

## Catatan Keamanan Awal

Karena aplikasi ini dapat diekspos ke internet melalui Cloudflare Tunnel atau reverse proxy, semua mode runtime harus diperlakukan seperti production server.

Kontrol minimum yang direncanakan:

- Auth dan RBAC di backend
- Brand-scoped authorization untuk seller
- Argon2id password hashing
- Server-side session
- CSRF protection
- Upload validation
- Payment proof private/protected
- Install panel locking
- Secret masking/redaction
- PostgreSQL desktop bind ke localhost
- Tunnel opt-in

Detail lengkap ada di [`TRD.md`](./TRD.md).
