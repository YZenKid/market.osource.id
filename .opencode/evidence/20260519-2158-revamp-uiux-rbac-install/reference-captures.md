# Reference Captures — Revamp UI/UX

Status: reference URL tersedia dari user intent: dashboard harus mengikuti gaya template dashboard TweakCN. Referensi utama: `https://tweakcn.com/editor/theme` dan/atau preview dashboard TweakCN yang tersedia di situs. Gunakan sebagai style-equivalent reference saja; jangan salin asset/kode proprietary.

## Capture workflow wajib saat implementasi

Gunakan workflow sama untuk current dan final:
1. set viewport `1440x1200`, `768x1024`, `390x844`;
2. navigate dengan `waitUntil: "networkidle"` jika server mendukung;
3. tunggu loading/preloader hilang;
4. tunggu 300–700ms untuk settle animation;
5. scroll bertahap sampai bawah untuk trigger lazy/reveal;
6. tunggu singkat per scroll;
7. kembali ke posisi target;
8. screenshot setelah stabil.

## Reference requirement

Referensi dashboard TweakCN yang harus dicapture:
- URL utama: `https://tweakcn.com/editor/theme`
- Jika situs menyediakan dashboard preview/template spesifik, capture preview dashboard itu juga.
- Dashboard target harus mengadopsi anatomi style-equivalent: sidebar kiri persistent, grouped navigation, top bar, cards modern, recent activity/table, secondary panels, token-based theme surface.
- Jangan menyalin asset, logo, layout exact pixel, atau kode dari TweakCN. Gunakan tokens internal repo.

Referensi internal tambahan:
- storefront root setelah install punya hero katalog clothing, filters, featured brands, product grid;
- install panel hanya muncul first-run, wizard step-by-step.

## Screenshot paths target

- `.opencode/evidence/20260519-2158-revamp-uiux-rbac-install/current-captures.md`
- `.opencode/evidence/20260519-2158-revamp-uiux-rbac-install/visual-comparison.md`

## Limitasi

Belum menjalankan browser/dev server pada fase planning. Screenshot menjadi gate implementasi, bukan bukti final saat ini.
