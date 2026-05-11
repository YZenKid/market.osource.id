# Reference Capture Requirements

Task ID: `20260511-2156-project-completion-master-plan`

Status: **Belum ditangkap**

## Required Workflow

Untuk setiap reference/current/final capture:

1. Set viewport tepat:
   - desktop `1440x1200`
   - tablet `768x1024`
   - mobile `390x844`
2. Navigate dengan `waitUntil: "networkidle"` bila memungkinkan.
3. Tunggu preloader/loading overlay hilang bila selector diketahui.
4. Tunggu settle singkat untuk entrance animation.
5. Scroll halaman bertahap untuk trigger lazy-loaded media / reveal animation.
6. Tunggu singkat setelah tiap scroll pass.
7. Kembali ke top/intended section sebelum hero screenshot.
8. Catat console/network error yang memengaruhi rendering.

## Required Pages / Sections

- Storefront home
- Product list/grid
- Product detail
- Cart
- Checkout
- Order tracking / success state
- Admin dashboard
- Admin brands
- Admin products
- Admin orders
- Admin system
- Admin packages
- Install panel
- Desktop control panel

## Current Blocker

- Existing repo evidence menunjukkan browser runtime capture pernah gagal karena Chromium/Chrome belum tersedia.
- Jika blocker masih sama saat implementation, lane designer/browser harus menyelesaikan runtime browser dulu sebelum claim visual parity.
