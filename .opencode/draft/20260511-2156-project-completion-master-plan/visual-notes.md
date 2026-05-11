# Catatan Visual Draft — Project Completion Master Plan

Task ID: `20260511-2156-project-completion-master-plan`

## Surface Priority

User memilih semua core surfaces untuk prioritas visual mendalam:

1. Storefront customer
2. Checkout & post-checkout / order tracking
3. Admin dashboard operator
4. Admin brand / products / orders / system / packages
5. Install panel VPS
6. Desktop control panel Tauri

## Visual Direction yang Sudah Terkunci dari Repo

- Bukan gaya marketing-heavy.
- Bukan glassmorphism / gradient SaaS generik.
- Status-forward, operator-trust, calm operations UI.
- Surface utama memakai `.panel`, semantic status colors, dan hierarchy yang jelas.
- Motion harus subtle dan optional; hormati `prefers-reduced-motion`.

## Visual Density Rubric

- **Storefront**: medium density, content-first, image-supported, CTA jelas.
- **Checkout**: medium-high clarity, low confusion, sticky summary desktop.
- **Admin**: compact but breathable, high state visibility.
- **Install**: guided wizard density, strong success/error communication.
- **Desktop control panel**: operational dashboard density, high status clarity.

## Motion Storyboard Draft

- Storefront:
  - subtle fade/translate-in section entrance,
  - image skeleton → loaded state,
  - sticky cart summary on desktop,
  - no essential information hidden behind animation.
- Admin:
  - quick highlight transitions for status chips,
  - no large-motion card hover,
  - left rail sticky behavior only.
- Install:
  - step transition minimal,
  - validation state immediate,
  - progress/locking state explicit.
- Desktop:
  - service start/stop states via progress/status indicators only.

## Icon Matrix Draft

- Navigation icons: operational, outline/simple.
- Status icons: success, warning, destructive, neutral, primary.
- Commerce icons: cart, package, receipt, upload proof, order tracking.
- Runtime icons: database, backend, tunnel, storage, diagnostics.

## Browser Evidence Note

- Capture workflow wajib memakai viewport tetap dan wait-stabilize-scroll-settle.
- Existing repo evidence menunjukkan screenshot capture pernah terblokir oleh missing Chromium/Chrome runtime; implementasi harus menyelesaikan blocker ini sebelum mengklaim visual parity.
