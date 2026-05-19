# Generated Assets — Revamp UI/UX

Status: no-generation-needed untuk planning awal.

## Keputusan asset

- Product images clothing demo: `use-provided-assets` jika `apps/web/static/assets/demo/product-*.png` cukup; jika kualitas tidak sesuai, gunakan `generate` legal style-equivalent pada fase implementation asset pass.
- Brand logos demo: `generate` legal style-equivalent jika tidak ada logo internal. Jangan pakai logo brand nyata tanpa lisensi.
- Icons dashboard: `licensed-existing-assets` bila menambah icon library legal seperti `lucide-svelte`; jika tidak ingin dependency, gunakan text/status badge saja.
- Rich backgrounds: `no-generation-needed`; desain harus operational trust, bukan dekoratif berat.

## Target jika generate diperlukan

- path relatif target app: `apps/web/static/assets/demo/brands/*.png` atau `apps/web/static/assets/demo/products/*.png`;
- semua prompt harus style-equivalent legal, tidak meniru brand spesifik seperti Zara/Nike/Uniqlo;
- alt text wajib.

## Catatan legal

Tidak boleh menyalin restricted reference assets. Demo data boleh menggunakan nama fiktif: `Batik Nusantara`, `Urban Threads`, `Modest Wear ID`.
