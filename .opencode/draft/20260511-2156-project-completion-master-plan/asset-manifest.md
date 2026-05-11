# Draft Asset Manifest — Project Completion Master Plan

Task ID: `20260511-2156-project-completion-master-plan`

## Section-by-Section Asset Planning

### 1. Storefront Home / Catalog

- Hero / masthead visual: `generate`
- Brand card imagery: `use-provided-assets` jika operator sudah upload logo; fallback `generate` untuk demo/dev placeholders legal style-equivalent
- Product thumbnails: `use-provided-assets`
- Empty-state illustrations: `generate`

### 2. Product Detail

- Product gallery: `use-provided-assets`
- Variant/attribute visual support icons: `licensed-existing-assets` atau icon library

### 3. Cart / Checkout / Order Tracking

- No decorative hero required
- Small semantic illustrations for success/empty/error: `generate`

### 4. Admin Dashboard

- No decorative imagery required
- Runtime/system icons: `licensed-existing-assets`
- Empty-state/support illustrations: `generate` only if needed

### 5. Install Panel

- No heavy imagery required
- Guided setup illustration / trust visual: `generate` optional

### 6. Desktop Control Panel

- No heavy imagery required
- Runtime status visuals and diagrams: icon/system driven, `licensed-existing-assets`

## Legal Handling

- Jangan menyalin reference asset yang restricted.
- Gunakan legal style-equivalent generation untuk hero/illustration assets.
- Product media nyata tetap berasal dari operator upload, bukan generated replacement untuk production catalog.

## Operational Notes

- Asset generation baru boleh dijalankan setelah visual spec matrix per surface disetujui.
- Semua generated asset harus dicatat di `.opencode/evidence/<task-id>/generated-assets.md` saat execution lane berjalan.
