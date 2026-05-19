# Animation Audit — Revamp UI/UX

Status: pending implementation.

## Prinsip

- Motion hanya untuk feedback ringan, bukan untuk memahami flow.
- Respect `prefers-reduced-motion` dari `apps/web/src/lib/styles/app.css`.
- Hindari preloaders panjang pada VPS install dan checkout.

## Allowed motion

- hover border/background tint;
- skeleton shimmer hanya jika reduced-motion nonaktif;
- stepper progress instant/simple;
- drawer/sidebar mobile slide ringan jika ada fallback reduced-motion.

## Not allowed

- entrance animation berat yang menunda install/setup;
- status change hanya lewat animasi;
- scroll-reveal wajib untuk konten penting.

## Capture notes

Browser capture final harus menunggu settle period dan scroll pass sebelum screenshot.
