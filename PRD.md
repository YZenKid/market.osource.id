# PRD — Self-Hosted Multi-Brand Marketplace

## 1. Ringkasan

Produk ini adalah aplikasi marketplace multi-brand yang dapat diinstal dan dijalankan secara mandiri oleh operator marketplace. Sistem mendukung dua mode deployment utama:

1. **Desktop local-hosted** — aplikasi desktop berbasis Tauri v2 menjalankan backend Rust Axum, PostgreSQL lokal, storage lokal, dan `cloudflared` sidecar agar marketplace dapat diakses publik melalui web.
2. **VPS hosted** — marketplace diinstal di VPS melalui web install panel untuk konfigurasi awal, domain/tunnel, database, storage, dan admin pertama.

Customer mengakses marketplace melalui web responsive. Mobile native tidak menjadi fokus awal.

Produk menggunakan model **open-core**: core/base marketplace dirilis sebagai open source, sementara fitur lanjutan tertentu tersedia sebagai package/module tambahan berbayar.

## 2. Tujuan Produk

- Membuat marketplace multi-brand yang bisa dimiliki dan dijalankan sendiri oleh operator.
- Memungkinkan instalasi lokal di desktop tanpa harus punya infrastruktur cloud penuh.
- Memungkinkan instalasi VPS untuk operator yang membutuhkan uptime lebih stabil.
- Memberikan storefront web responsive untuk customer.
- Memberikan dashboard admin/seller untuk mengelola brand, produk, order, dan konfigurasi.
- Menyediakan jalur akses publik melalui Cloudflare Tunnel tanpa harus membuka port langsung.
- Menyediakan fondasi open source yang bisa digunakan mandiri.
- Membuka jalur monetisasi melalui package/module berbayar untuk fitur commerce lanjutan.

## 3. Non-Goals Awal

- Mobile app native Android/iOS.
- Marketplace skala besar seperti Shopee/Tokopedia.
- Multi-region/high-availability.
- Payment gateway otomatis penuh.
- Shipping aggregator penuh.
- Plugin marketplace.
- Advanced analytics.
- Chat buyer-seller.
- Recommendation engine.
- Sistem SaaS hosted multi-tenant yang dikelola pusat.
- Self-service seller registration.

Catatan: beberapa fitur non-goal awal dapat masuk sebagai package berbayar pada fase lanjutan, bukan bagian dari base open source MVP.

## 3.1 Model Produk & Monetisasi

Produk memakai pendekatan **open-core self-hosted marketplace**.

### Base Open Source

Base open source berisi fitur inti yang cukup untuk menjalankan marketplace mandiri:

- Installer desktop local-hosted.
- VPS web install panel.
- Backend marketplace core.
- Storefront responsive.
- Admin dashboard basic.
- Multi-brand basic.
- Seller assignment by Super Admin.
- Category, product, variant, dan inventory basic.
- Cart dan checkout manual transfer.
- Upload bukti transfer.
- Order management basic.
- Local storage.
- `cloudflared` sidecar bundled.
- PostgreSQL bundled untuk desktop.

### Package/Module Berbayar

Package berbayar adalah fitur tambahan yang tidak wajib untuk marketplace basic, tetapi memberi nilai bisnis lanjutan.

Contoh package berbayar:

- **Promo Package**
  - voucher,
  - campaign discount,
  - bundle discount,
  - flash sale,
  - brand-specific promotion.
- **Online Payment Package**
  - payment gateway,
  - webhook payment,
  - payment reconciliation,
  - payment status automation.
- **Delivery Package**
  - ongkir otomatis,
  - integrasi kurir/logistic provider,
  - tracking shipment,
  - pickup/dropoff workflow.
- **Advanced Analytics Package**
  - sales report,
  - brand performance,
  - product performance,
  - export report.
- **Backup & Cloud Sync Package**
  - scheduled backup,
  - encrypted remote backup,
  - restore assistant.
- **Theme/Customization Package**
  - theme builder,
  - custom storefront sections,
  - brand visual customization.

### Prinsip Packaging

- Base harus tetap berguna tanpa package berbayar.
- Package tidak boleh merusak data portability base open source.
- Package harus punya boundary teknis jelas.
- Package dapat diaktifkan/nonaktifkan dari admin settings.
- Fitur berbayar tidak boleh mencampur logic kritikal ke core tanpa kontrak API/module yang jelas.
- Marketplace tetap dapat berjalan offline/lokal tanpa package cloud-dependent, kecuali package tersebut memang membutuhkan layanan eksternal.

## 4. Target Pengguna

### 4.1 Operator Marketplace

Pihak yang menginstal dan mengoperasikan marketplace.

Contoh:

- Komunitas UMKM.
- Koperasi.
- Distributor.
- Marketplace lokal/niche.
- Agensi yang membuat marketplace untuk klien.
- Pemilik bisnis yang mengelola beberapa brand.

Kebutuhan utama:

- Instalasi mudah.
- Data dimiliki sendiri.
- Bisa berjalan lokal maupun di VPS.
- Bisa online via domain/tunnel.
- Bisa mengelola beberapa brand/seller.
- Tidak membutuhkan DevOps kompleks.

### 4.2 Super Admin

Pengguna yang memiliki kontrol penuh atas marketplace.

Kebutuhan utama:

- Mengelola konfigurasi marketplace.
- Mengelola brand/seller.
- Mengelola produk dan order lintas brand.
- Melihat status sistem.
- Mengelola storage, tunnel, dan backup.

### 4.3 Brand Admin / Seller

Pihak brand/vendor yang mengelola katalog, stok, dan order miliknya.

Kebutuhan utama:

- Mengelola profil brand.
- Mengelola produk.
- Mengelola stok.
- Melihat dan memproses order.
- Tidak bisa mengakses data brand lain.
- Satu seller dapat ditugaskan untuk mengelola lebih dari satu brand.

### 4.4 Customer / Buyer

Pengunjung web marketplace.

Kebutuhan utama:

- Browse produk dari berbagai brand.
- Mencari/filter produk.
- Melihat detail produk.
- Checkout dari web responsive.
- Mendapat nomor order dan status order.

## 5. Stack Teknologi Tetap

| Area | Stack |
| --- | --- |
| Desktop | Tauri v2 |
| Backend | Rust Axum |
| Database | PostgreSQL lokal |
| Frontend | SvelteKit |
| Styling | Tailwind CSS + TweakCN |
| Tunnel | `cloudflared` sidecar |
| Storage | Local storage dengan konfigurasi S3-compatible |
| Extension Model | Open-core package/module system |

Prinsip teknis:

- Backend marketplace core berada di Rust Axum.
- Tauri berperan sebagai desktop control panel, process manager, dan shell lokal.
- Frontend customer dan dashboard dibuat dengan SvelteKit.
- Styling menggunakan Tailwind dan TweakCN sebagai fondasi komponen/theme.
- PostgreSQL menjadi database utama di desktop maupun VPS dan **dibundel untuk desktop installer**.
- Local storage menjadi default MVP; konfigurasi S3-compatible disiapkan untuk ekspansi setelah MVP.
- `cloudflared` berjalan sebagai sidecar dan **dibundel bersama distribusi aplikasi**.
- Fitur lanjutan dirancang sebagai package/module agar base open source tetap ringan dan monetisasi dapat dilakukan tanpa mengubah core menjadi SaaS.

## 6. Mode Deployment

### 6.1 Desktop Local-Hosted Mode

Mode ini ditujukan untuk operator yang ingin menjalankan marketplace dari komputer lokal.

Komponen:

- Tauri desktop app.
- Axum backend local service.
- PostgreSQL lokal yang dibundel bersama installer desktop.
- SvelteKit storefront/admin.
- Local file storage.
- `cloudflared` sidecar yang dibundel, namun aktivasi tunnel tetap opsional.

Kemampuan utama:

- Start/stop backend dari app desktop.
- Preflight check environment lokal.
- Setup wizard marketplace.
- Monitoring status service.
- Konfigurasi tunnel.
- Konfigurasi storage.
- Akses dashboard admin.
- Log viewer dasar.
- Backup/restore database dan media pada fase lanjut.

Keterbatasan yang harus dikomunikasikan:

- Jika komputer mati, marketplace tidak dapat diakses.
- Uptime bergantung pada koneksi internet operator.
- Payment webhook langsung ke host lokal berisiko jika host offline.
- Backup wajib disediakan karena data berada di mesin operator.

### 6.2 VPS Hosted Mode

Mode ini ditujukan untuk operator yang ingin marketplace online 24/7 di server/VPS.

Komponen:

- Axum backend berjalan sebagai service.
- PostgreSQL lokal di VPS atau host yang sama.
- SvelteKit storefront/admin.
- Local file storage di VPS.
- Web install panel.
- `cloudflared` sidecar opsional, atau domain reverse proxy biasa.

Kemampuan utama:

- Instalasi awal lewat web install panel.
- Preflight check server.
- Konfigurasi database.
- Konfigurasi storage.
- Konfigurasi domain/base URL.
- Pembuatan super admin pertama.
- Migrasi database.
- Lock install panel setelah setup selesai.
- Health check runtime.

## 7. Arsitektur Konseptual

```txt
Customer Browser
    |
    | HTTPS
    v
Cloudflare / Domain / Reverse Proxy
    |
    v
Rust Axum Backend
    |
    +-- SvelteKit Storefront
    +-- SvelteKit Admin Dashboard
    +-- Marketplace API
    +-- Install Panel API
    +-- Storage API
    +-- Tunnel Status API
    |
    v
PostgreSQL + Local Storage
```

Desktop mode:

```txt
Tauri Desktop App
    |
    +-- Process Manager
    +-- Axum Backend
    +-- PostgreSQL Connection
    +-- cloudflared Sidecar
    +-- Local Storage
    +-- Admin WebView / Browser Launcher
```

VPS mode:

```txt
VPS Service
    |
    +-- Axum Backend Service
    +-- PostgreSQL
    +-- Local Storage
    +-- Web Install Panel
    +-- Optional cloudflared Sidecar
```

## 8. MVP Scope

### 8.1 Instalasi & Setup

#### Desktop Install

- Aplikasi Tauri dapat dijalankan di desktop.
- App dapat melakukan preflight check:
  - backend binary tersedia,
  - PostgreSQL dapat dihubungi,
  - storage path writable,
  - port lokal tersedia,
  - `cloudflared` tersedia atau dapat dikonfigurasi.
- Setup wizard membuat konfigurasi marketplace awal.
- Setup wizard membuat super admin pertama.
- Migrasi database berjalan otomatis.
- App menampilkan status service:
  - backend,
  - database,
  - storage,
  - tunnel.

#### VPS Install Panel

- Web install panel tersedia pada instalasi baru.
- Panel melakukan preflight check:
  - koneksi PostgreSQL,
  - permission storage,
  - base URL/domain,
  - migrasi database,
  - kemampuan menulis konfigurasi.
- User mengisi konfigurasi awal:
  - nama marketplace,
  - base URL/domain,
  - database connection,
  - storage path,
  - S3 settings opsional,
  - akun super admin.
- Setelah setup selesai, install panel terkunci.

### 8.2 Marketplace Core

- Multi-brand basic.
- Brand profile:
  - nama,
  - slug,
  - logo,
  - deskripsi,
  - status aktif/nonaktif.
- Produk basic:
  - nama,
  - slug,
  - deskripsi,
  - harga,
  - stok,
  - gambar,
  - category,
  - variant,
  - status draft/published.
- Storefront:
  - homepage,
  - daftar produk,
  - detail produk,
  - halaman brand,
  - pencarian/filter dasar.
- Cart dan checkout basic.
- Order management basic.
- Role dasar:
  - Super Admin,
  - Brand Admin/Seller,
  - Customer/Guest.
- Seller tidak mendaftar sendiri; akun seller dibuat dan ditugaskan oleh Super Admin.
- Satu seller dapat memiliki akses ke banyak brand melalui assignment.
- Satu domain merepresentasikan satu marketplace.

### 8.3 Checkout MVP

Checkout awal menggunakan pembayaran manual dengan upload bukti transfer.

Fitur:

- Customer dapat menambahkan produk ke cart.
- Cart dapat berisi produk dari beberapa brand dalam satu order.
- Customer mengisi data kontak dan alamat.
- Sistem membuat order.
- Sistem menyimpan snapshot harga saat order dibuat.
- Sistem menampilkan nomor order dan instruksi pembayaran manual.
- Customer dapat upload bukti transfer setelah order dibuat.
- Admin dapat melihat dan memverifikasi bukti transfer.
- Admin/seller dapat mengubah status order.

Status order awal:

- `pending`
- `waiting_payment_verification`
- `confirmed`
- `processing`
- `shipped`
- `completed`
- `cancelled`

### 8.4 Admin Dashboard MVP

- Login admin/seller.
- Dashboard ringkasan:
  - total brand,
  - total produk,
  - total order,
  - order terbaru.
- Manajemen brand.
- Manajemen produk.
- Manajemen order.
- Manajemen setting marketplace.
- Status service/tunnel/storage.

### 8.5 Storage MVP

- Upload logo brand.
- Upload gambar produk.
- Upload bukti transfer customer.
- File tersimpan di local storage path.
- File dapat disajikan ke storefront.
- Konfigurasi S3-compatible disiapkan sebagai struktur konfigurasi untuk ekspansi setelah MVP:
  - endpoint,
  - bucket,
  - region,
  - access key,
  - secret key,
  - public base URL.
- Pada MVP, penyimpanan aktif hanya local storage.

### 8.6 Tunnel MVP

- `cloudflared` sidecar dibundel dan dapat dikonfigurasi.
- Admin dapat melihat status tunnel.
- Admin dapat melihat URL publik jika tunnel aktif.
- Jika tunnel gagal, sistem menampilkan error dan fallback local URL.
- Tunnel opsional; VPS dapat menggunakan reverse proxy/domain biasa.

### 8.7 Package/Module Awareness MVP

- Base open source harus memiliki tempat untuk mendeteksi dan menampilkan package yang tersedia/aktif.
- MVP tidak wajib memiliki marketplace package penuh.
- Admin settings harus dapat menampilkan status fitur:
  - base feature,
  - package available,
  - package installed,
  - package disabled.
- Fitur promo, online payment, delivery, advanced analytics, dan cloud backup tidak masuk base MVP.
- Core data model harus disiapkan agar package lanjutan dapat menambahkan tabel/migrasi tanpa merusak instalasi existing.

## 9. Key User Flows

### 9.1 Desktop Local Install Flow

1. Operator mengunduh dan membuka aplikasi desktop.
2. Tauri menampilkan setup wizard jika sistem belum dikonfigurasi.
3. App menjalankan preflight check.
4. Operator mengisi:
   - nama marketplace,
   - konfigurasi PostgreSQL lokal,
   - storage path,
   - admin email/password,
   - base URL lokal.
5. Sistem menjalankan migrasi database.
6. Sistem menyimpan konfigurasi.
7. Backend Axum berjalan lokal.
8. Admin masuk ke dashboard.
9. Opsional: admin mengaktifkan Cloudflare Tunnel.
10. Sistem menampilkan URL publik.

### 9.2 VPS Install Flow

1. Operator membuka URL install panel pada VPS.
2. Sistem menampilkan preflight check.
3. Operator mengisi:
   - nama marketplace,
   - domain/base URL,
   - konfigurasi PostgreSQL,
   - konfigurasi storage,
   - konfigurasi S3 opsional,
   - admin pertama.
4. Sistem memvalidasi konfigurasi.
5. Sistem menjalankan migrasi database.
6. Sistem menyimpan konfigurasi.
7. Install panel terkunci.
8. Operator diarahkan ke dashboard.

### 9.3 Super Admin Membuat Brand

1. Super Admin login.
2. Membuka menu Brand.
3. Membuat brand baru.
4. Mengisi nama, slug, logo, deskripsi, dan status.
5. Menugaskan akun seller jika diperlukan.
6. Brand aktif muncul di storefront.

### 9.3.1 Super Admin Membuat Seller dan Assignment Brand

1. Super Admin login.
2. Super Admin membuka menu User/Seller.
3. Super Admin membuat akun seller.
4. Super Admin memilih satu atau lebih brand yang boleh dikelola seller tersebut.
5. Sistem menyimpan assignment seller-brand.
6. Seller dapat login dan melihat semua brand yang ditugaskan.
7. Seller tidak dapat melihat atau mengelola brand di luar assignment.

### 9.4 Seller Mengelola Produk

1. Seller login.
2. Seller memilih brand yang dikelola.
3. Seller membuka menu Produk.
4. Seller membuat produk baru.
5. Seller mengisi data produk dan upload gambar.
6. Seller menyimpan sebagai draft atau publish.
7. Produk published tampil di storefront jika brand aktif.

### 9.5 Customer Membuat Order

1. Customer membuka storefront.
2. Customer mencari/memilih produk.
3. Customer membuka detail produk.
4. Customer menambahkan produk ke cart.
5. Customer checkout.
6. Customer mengisi kontak dan alamat.
7. Sistem membuat order.
8. Customer melihat nomor order dan instruksi pembayaran manual.
9. Admin/seller memproses order.

### 9.6 Admin/Seller Memproses Order

1. Admin/seller membuka daftar order.
2. Admin/seller membuka detail order.
3. Admin/seller mengubah status order.
4. Sistem menyimpan histori status.
5. Customer dapat mengecek status order.

## 10. Functional Requirements

### 10.1 Installation & Runtime

- Sistem harus mendukung mode `desktop` dan `vps`.
- Sistem harus memiliki konfigurasi runtime yang eksplisit.
- Sistem harus dapat membedakan state belum setup dan sudah setup.
- Setup pertama harus idempotent sejauh memungkinkan.
- Migrasi database harus versioned.
- Install panel harus terkunci setelah setup selesai.
- Desktop app harus dapat menjalankan dan menghentikan backend local service.
- Desktop app harus menampilkan status runtime.

### 10.2 Authentication & Authorization

- Super Admin dibuat saat setup awal.
- Seller dibuat oleh Super Admin, bukan melalui self-registration.
- Satu seller dapat memiliki assignment ke banyak brand.
- Password disimpan dengan hashing aman.
- Admin endpoint membutuhkan autentikasi.
- Seller hanya bisa mengakses brand yang ditugaskan.
- Customer checkout dapat berjalan sebagai guest pada MVP.
- Session/cookie harus aman untuk deployment HTTPS.

### 10.3 Brand Management

- Super Admin dapat membuat brand.
- Super Admin dapat mengedit brand.
- Super Admin dapat menonaktifkan brand.
- Brand inactive tidak menampilkan produk di storefront.
- Brand slug harus unik.
- Marketplace menggunakan satu domain utama; brand menggunakan halaman/slug di bawah domain marketplace, bukan custom domain per brand pada MVP.

### 10.4 Product Management

- Seller dapat membuat produk untuk brand yang dikelola.
- Seller dapat mengedit produk brand miliknya.
- Produk dapat memiliki category.
- Produk dapat memiliki variant.
- Produk memiliki status draft/published.
- Produk published tampil di storefront.
- Produk draft tidak tampil di storefront.
- Stok tidak boleh negatif.
- Produk dengan stok 0 tidak dapat dibeli.

### 10.5 Cart & Checkout

- Customer dapat menambah produk ke cart.
- Customer dapat mengubah kuantitas cart.
- Cart dapat berisi produk lintas brand.
- Checkout menolak cart kosong.
- Checkout menolak kuantitas melebihi stok.
- Order menyimpan snapshot harga dan item.
- Order lintas brand harus dapat dipecah tampilan/prosesnya per brand untuk seller.
- Order memiliki nomor unik.
- Order memiliki status awal `pending`.
- Customer dapat mengupload bukti transfer untuk order manual transfer.
- Bukti transfer harus tersimpan di local storage dan terhubung ke order.

### 10.6 Order Management

- Admin/seller dapat melihat order.
- Seller hanya melihat item/sub-order terkait brand miliknya, termasuk pada order lintas brand.
- Super Admin dapat melihat semua order.
- Admin/seller dapat mengubah status order.
- Perubahan status tercatat dalam histori.

### 10.7 Storage

- Sistem dapat menyimpan file di local storage.
- Sistem memvalidasi tipe file upload.
- Sistem membatasi ukuran file upload.
- Sistem menyediakan URL file untuk frontend.
- Sistem menyimpan konfigurasi S3-compatible secara aman untuk ekspansi, tetapi adapter aktif MVP adalah local storage.

### 10.8 Tunnel & Public Access

- Sistem dapat menyimpan konfigurasi `cloudflared`.
- Sistem dapat membaca status tunnel.
- Sistem dapat menampilkan URL publik tunnel.
- Kegagalan tunnel tidak boleh mematikan marketplace lokal.

### 10.9 Package/Module System

- Sistem harus membedakan fitur base open source dan fitur package berbayar.
- Sistem harus memiliki registry internal untuk package/module yang terpasang.
- Sistem harus dapat menampilkan status package di admin dashboard.
- Sistem harus menolak akses fitur package yang belum terpasang/aktif.
- Package harus memiliki versi.
- Package harus dapat membawa migrasi database tersendiri.
- Package harus menggunakan permission/scope yang eksplisit.
- Base open source tidak boleh bergantung pada package berbayar untuk flow inti marketplace MVP.

## 11. Non-Functional Requirements

### 11.1 Security

- Password harus di-hash dengan algoritma modern.
- Secret tidak boleh ditampilkan mentah di UI.
- Install panel terkunci setelah setup.
- Endpoint admin dilindungi autentikasi.
- RBAC diterapkan di backend, bukan hanya UI.
- Upload file dibatasi tipe dan ukuran.
- Path traversal harus dicegah pada local storage.
- CSRF harus dipertimbangkan jika menggunakan cookie auth.
- CORS harus dibatasi sesuai konfigurasi domain.

### 11.2 Reliability

- Service health check tersedia.
- Error konfigurasi harus mudah dipahami.
- Migrasi database harus aman dijalankan ulang.
- Desktop app harus memberi indikator jika backend/database/tunnel mati.
- VPS install harus dapat divalidasi sebelum setup final.

### 11.3 Performance

- Storefront harus responsive dan cepat untuk katalog kecil-menengah.
- Gambar produk harus memiliki ukuran dan format yang wajar.
- Pagination atau lazy loading diperlukan untuk daftar produk.
- Query dashboard dan katalog harus menggunakan index yang sesuai.

### 11.4 Portability

- Runtime desktop tidak boleh bergantung pada path spesifik perangkat.
- Konfigurasi storage harus dapat dipindahkan.
- Mode VPS dan desktop harus menggunakan core backend yang sama.
- Perbedaan mode deployment diisolasi pada adapter/runtime layer.

### 11.5 Backup & Recovery

- MVP minimal menyediakan dokumentasi backup PostgreSQL dan storage.
- Roadmap harus mencakup backup scheduler.
- Backup harus mencakup database dan file media.
- Restore harus dirancang sebelum payment otomatis masuk ke scope production.

### 11.6 Open-Core Package Governance

- Boundary antara base open source dan package berbayar harus terdokumentasi.
- Package berbayar tidak boleh memblokir penggunaan fitur base.
- Package harus memiliki compatibility version terhadap versi core.
- Upgrade core harus memvalidasi compatibility package.
- Migrasi package harus dapat dilacak terpisah dari migrasi core.
- Lisensi base open source dan lisensi package berbayar harus dijelaskan di dokumentasi distribusi.

## 12. Data Model Awal

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
- `file_objects`
- `marketplace_settings`
- `storage_settings`
- `tunnel_settings`
- `packages`
- `package_migrations`
- `installation_state`
- `audit_events`

Catatan:

- Semua akses brand harus melewati relasi membership/role.
- Order item harus menyimpan snapshot nama produk, harga, brand, dan kuantitas.
- Satu order dapat berisi item dari banyak brand; tampilan seller harus difilter pada item brand terkait.
- Order lintas brand dikelompokkan per brand melalui `order_brand_groups`.
- File media disimpan sebagai object di local storage dan metadata-nya dicatat di `file_objects`.
- Bukti transfer disimpan sebagai file private di local storage; metadata file dicatat di `file_objects`, sedangkan status/verifikasi dicatat di `payment_proofs`.
- Event sensitif seperti login, perubahan role, assignment brand, status order, verifikasi bukti transfer, dan aktivasi package dicatat di `audit_events`.
- Setting sensitif harus dienkripsi atau disimpan dengan mekanisme aman sesuai mode deployment.

Status teknis canonical:

- `orders.payment_status`: `pending`, `waiting_payment_verification`, `confirmed`, `rejected`, `cancelled`.
- `orders.global_status`: `open`, `confirmed`, `in_progress`, `partially_shipped`, `completed`, `cancelled`.
- `order_brand_groups.fulfillment_status`: `not_ready`, `ready_to_process`, `processing`, `shipped`, `completed`, `cancelled`.
- `payment_proofs.status`: `uploaded`, `verified`, `rejected`.

## 13. Acceptance Criteria MVP

### 13.1 Desktop Install

- Aplikasi Tauri dapat dibuka.
- Setup wizard muncul jika belum setup.
- Preflight check berjalan dan menampilkan hasil.
- PostgreSQL lokal yang dibundel dapat diinisialisasi dan dikonfigurasi.
- Migrasi database berhasil.
- Super Admin pertama berhasil dibuat.
- Backend lokal berjalan.
- Dashboard admin dapat dibuka.
- Storage lokal dapat menerima upload gambar.
- `cloudflared` sidecar yang dibundel dapat ditemukan dan dikonfigurasi.
- Status backend, database, storage, dan tunnel ditampilkan.

### 13.2 VPS Install Panel

- Install panel dapat dibuka pada instalasi baru.
- Panel memvalidasi database, storage, dan base URL.
- Panel dapat membuat super admin pertama.
- Panel menjalankan migrasi database.
- Panel terkunci setelah setup selesai.
- Marketplace dapat diakses setelah setup.

### 13.3 Multi-Brand

- Super Admin dapat membuat minimal dua brand.
- Super Admin dapat membuat seller dan menugaskan seller ke satu atau lebih brand.
- Satu seller dapat mengelola lebih dari satu brand yang ditugaskan.
- Brand aktif memiliki halaman publik.
- Brand nonaktif tidak menampilkan produk.
- Seller tidak dapat mengakses brand yang bukan miliknya.
- Satu domain hanya merepresentasikan satu marketplace.

### 13.4 Produk

- Seller dapat membuat produk dengan gambar, harga, stok, deskripsi, category, variant, dan status.
- Produk draft tidak tampil di storefront.
- Produk published tampil jika brand aktif.
- Produk stok 0 tidak dapat dibeli.
- Upload invalid menampilkan error jelas.

### 13.5 Checkout & Order

- Customer dapat membuat order dari cart.
- Customer dapat membuat satu order berisi produk lintas brand.
- Cart kosong tidak dapat checkout.
- Kuantitas melebihi stok ditolak.
- Order menyimpan snapshot item dan total.
- Customer dapat upload bukti transfer.
- Admin dapat melihat dan memverifikasi bukti transfer.
- Seller hanya melihat item order milik brand-nya pada order lintas brand.
- Admin/seller dapat mengubah status order.
- Histori status order tercatat.

### 13.6 Responsive Web

- Storefront dapat digunakan di desktop, tablet, dan mobile browser.
- Dashboard admin minimal usable di desktop dan tablet.
- Checkout dapat diselesaikan dari mobile browser.
- Navigasi, tombol, form, dan cart tetap jelas pada viewport kecil.

### 13.7 Security Minimum

- Password admin tidak tersimpan plaintext.
- Install panel tidak dapat digunakan ulang setelah setup.
- Admin API membutuhkan autentikasi.
- Seller isolation diterapkan di backend.
- Upload file membatasi tipe dan ukuran.
- Secret storage/tunnel tidak ditampilkan penuh di UI.

### 13.8 Open-Core Package Readiness

- Admin dapat melihat daftar/status package dasar di settings.
- Sistem dapat membedakan fitur base dan fitur package.
- Fitur package berbayar yang belum aktif tidak dapat diakses.
- Core marketplace tetap dapat berjalan tanpa package berbayar.
- Struktur migrasi mendukung migrasi core dan migrasi package secara terpisah.

## 14. UX Requirements

### 14.1 Storefront Customer

- Responsive-first.
- Navigasi sederhana.
- Pencarian produk terlihat jelas.
- Product card menampilkan brand, nama, harga, gambar, dan status stok.
- Detail produk menampilkan CTA add to cart yang jelas.
- Cart dan checkout minim langkah.
- Order success menampilkan nomor order dan instruksi pembayaran.

### 14.2 Admin Dashboard

- Struktur navigasi jelas:
  - Dashboard,
  - Brand,
  - Produk,
  - Order,
  - Settings,
  - System Status.
- Form harus memiliki validasi dan pesan error jelas.
- Empty state harus membantu user membuat data pertama.
- Status runtime harus mudah dipahami oleh operator non-teknis.

### 14.3 Install Panel

- Wizard bertahap.
- Menampilkan preflight check dengan status sukses/gagal.
- Error harus memberi instruksi perbaikan.
- Setup selesai harus mengarahkan user ke login dashboard.
- Setelah setup selesai, akses install panel harus menampilkan status terkunci.

## 15. Risiko & Mitigasi

| Risiko | Dampak | Mitigasi |
| --- | --- | --- |
| Desktop host mati | Marketplace offline | Tampilkan status, dokumentasi uptime, dorong VPS untuk production |
| PostgreSQL bundled gagal start | Setup gagal | Preflight check, log ringkas, retry/start-stop service, panduan recovery |
| Tunnel gagal | Tidak bisa diakses publik | Tunnel opsional, log ringkas, fallback local URL |
| Data hilang di mesin lokal | Kehilangan order/produk | Backup docs sejak MVP, backup scheduler di roadmap |
| RBAC multi-brand lemah | Kebocoran data brand | Authorization backend wajib, test isolation |
| Upload file berbahaya | Security risk | Validasi tipe/ukuran, path traversal protection |
| Payment manual dan bukti transfer membingungkan | Order tidak jelas | Instruksi pembayaran jelas, upload proof sederhana, status verifikasi transparan |
| Order lintas brand membingungkan seller | Seller melihat data tidak relevan | Seller view difilter per brand dan item order dikelompokkan per brand |
| Dua mode install menambah kompleksitas | Bug runtime | Shared setup core, adapter desktop/VPS terpisah |
| Storage lokal tidak scalable | Performa gambar buruk | Local storage dipakai untuk MVP, S3-compatible masuk roadmap setelah MVP |
| Boundary open source dan package berbayar kabur | Kebingungan user dan teknis | Dokumentasikan fitur base vs package, buat registry package dan compatibility version |
| Package berbayar merusak upgrade core | Instalasi production gagal | Pisahkan migrasi core/package, validasi compatibility sebelum upgrade |

## 16. Roadmap Bertahap

### Phase 0 — Foundation

- Struktur monorepo/app.
- Axum backend skeleton.
- SvelteKit storefront/admin skeleton.
- Tauri shell/control panel skeleton.
- PostgreSQL connection dan migration system.
- Bundled PostgreSQL runtime untuk desktop.
- Bundled `cloudflared` sidecar runtime.
- Installation state.
- Runtime config.
- Health check.
- Package/module registry basic.

### Phase 1 — Installable MVP

- Desktop setup wizard.
- VPS web install panel.
- Super Admin creation.
- Brand management.
- Product management.
- Storefront basic.
- Cart dan checkout manual.
- Upload bukti transfer.
- Order lintas brand.
- Order management.
- Local storage upload.
- cloudflared sidecar status/config.
- Package awareness di admin settings.

### Phase 2 — Operational Readiness

- Backup/restore flow.
- Log viewer.
- Diagnostics export.
- Import/export produk.
- Email/notification basic.
- Hardening RBAC.
- Update/migration safety.

### Phase 3 — Commerce Expansion

- Product variants.
- Payment gateway.
- Shipping config/integration.
- Promo package.
- Online payment package.
- Delivery package.
- Customer account.
- Voucher/promo basic.
- Order notification.
- S3-compatible storage production-ready.

### Phase 4 — Marketplace Platform

- Seller onboarding.
- Commission rules.
- Review/rating.
- Analytics.
- Theme customization.
- Audit log.
- Integration hooks.
- Package distribution/update mechanism.

### Phase 5 — Scale & Production Hardening

- Observability.
- Advanced backup scheduler.
- Performance tuning.
- Deployment guide untuk VPS production.
- Security audit checklist.
- Multi-instance guidance jika dibutuhkan.

## 17. Definisi Sukses MVP

MVP dianggap berhasil jika:

1. Operator dapat menginstal marketplace di desktop local-hosted.
2. Operator dapat menginstal marketplace di VPS melalui web install panel.
3. Super Admin dapat membuat lebih dari satu brand.
4. Seller dapat mengelola produk brand masing-masing.
5. Customer dapat membuka storefront responsive dan membuat order.
6. Customer dapat membuat order lintas brand dan upload bukti transfer.
7. Admin/seller dapat memproses order sesuai hak akses brand.
8. Sistem berjalan dengan PostgreSQL lokal yang dibundel dan local storage.
9. `cloudflared` sidecar yang dibundel dapat dikonfigurasi untuk akses publik.
10. Install panel terkunci setelah setup.
11. RBAC dasar mencegah akses data antar brand.
12. Base open source dapat berjalan tanpa package berbayar.
13. Admin dapat melihat status package/module di settings.

## 18. Keputusan Produk Terkunci

- Desktop installer membundel PostgreSQL lokal.
- Distribusi aplikasi membundel `cloudflared` sebagai sidecar.
- MVP checkout menggunakan manual transfer dengan upload bukti transfer.
- Satu order boleh berisi produk lintas brand.
- Order lintas brand memakai status global untuk pembayaran/order dan status fulfillment per brand/sub-order.
- Bukti transfer schema mendukung banyak file per order; UI MVP membatasi satu bukti transfer aktif per order.
- Akses bukti transfer dikendalikan oleh RBAC backend, bukan file public/static.
- Seller default hanya melihat status pembayaran/order brand assigned; seller boleh melihat file bukti transfer hanya jika Super Admin memberi permission eksplisit dan scope brand/order cocok.
- Nama permission payment proof canonical:
  - `payment_proof.view_assigned`: seller dapat melihat/download file proof hanya untuk order yang memiliki item dari brand assigned miliknya; akses wajib diaudit.
  - `payment_proof.verify`: Super Admin atau role eksplisit dapat menandai proof/order payment sebagai verified/confirmed.
  - `payment_proof.reject`: Super Admin atau role eksplisit dapat menolak proof dan mencatat alasan reject.
- VPS production baseline mendukung binary + systemd dan container sejak awal.
- VPS production menggunakan SvelteKit service terpisah dari Axum API service, biasanya di balik reverse proxy satu origin publik.
- Desktop installer menargetkan Windows, macOS, dan Linux sejak awal.
- Target awal desktop adalah all-platform release candidate; public v1.0 memerlukan kebijakan signing/notarization/distribution yang layak.
- Harga produk MVP bersumber dari `product_variants`; produk sederhana memakai default/internal variant.
- Package berbayar diarahkan ke remote entitlement; base open source tetap berjalan tanpa entitlement berbayar.
- Storage aktif MVP adalah local storage; S3-compatible disiapkan untuk roadmap setelah MVP.
- Seller tidak mendaftar sendiri; akun seller dibuat oleh Super Admin.
- Satu seller dapat mengelola banyak brand melalui assignment.
- Satu domain merepresentasikan satu marketplace; brand berada di bawah domain marketplace.
- Produk menggunakan model open-core: base open source, fitur lanjutan sebagai package/module berbayar.
- Promo, online payment, delivery, advanced analytics, cloud backup, dan theme customization masuk kandidat package berbayar.

## 19. Pertanyaan Terbuka

Keputusan yang masih perlu dikunci:

- Bagaimana strategi update aplikasi desktop dan migrasi database setelah user punya data production?
- Detail format distribusi desktop per OS: Windows `.msi`/`.exe`, macOS `.dmg`, Linux `.deb`/AppImage/RPM.
- Detail signing/notarization/distribution untuk public v1.0 desktop.
- Detail privacy/grace period remote entitlement untuk package berbayar.
