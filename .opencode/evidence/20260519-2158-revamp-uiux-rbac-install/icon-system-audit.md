# Icon System Audit — Revamp UI/UX

Status: pending implementation.

## Kondisi sekarang

- Tidak ditemukan icon system khusus di `apps/web/src/lib/ui`.
- UI banyak mengandalkan text labels, badge, dan panel.

## Rekomendasi

- Reuse `StatusBadge` untuk status penting.
- Jika perlu icon, tambah satu library legal kecil seperti `lucide-svelte` hanya setelah audit bundle dan kebutuhan jelas.
- Jangan pakai icon sebagai satu-satunya pembawa makna.
- Semua interactive icon harus punya label atau `aria-label`.

## Area butuh icon opsional

- sidebar admin (Dashboard, Orders, Products, Brands, Settings, System);
- status cards dashboard;
- install preflight success/error;
- checkout stepper.

## Gate

Icon pass wajib diverifikasi oleh accessibility review: label, contrast, focus state, reduced ambiguity.
