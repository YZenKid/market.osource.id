# Threat Model — Payment Proof

## Asset utama

- file bukti transfer,
- metadata `payment_proofs` dan `file_objects`,
- status verifikasi pembayaran,
- audit akses proof.

## Ancaman utama

1. Seller brand A mengakses proof brand B.
2. File proof tersaji sebagai static/public asset.
3. Upload berbahaya: file palsu, oversized, MIME spoofing, malware carrier.
4. Link/object key bocor lewat diagnostics, logs, error page, atau export.
5. Verifikasi/reject dilakukan tanpa permission scoped.

## Kontrol wajib

- Semua akses proof file harus lewat backend Axum, bukan direct static file serving.
- Seller hanya boleh melihat proof bila punya permission canonical yang brand/order-scoped dan diaudit.
- Upload wajib validasi size, MIME, extension, magic bytes, dan menolak SVG pada MVP.
- `file_objects.object_key`, original filename sensitif, dan path lokal tidak boleh muncul di diagnostics export.
- Semua event view/verify/reject perlu audit trail dengan metadata non-PII.

## Review checklist

- [ ] proof route memerlukan session + RBAC backend;
- [ ] seller cross-brand negative test ada;
- [ ] proof tidak dapat diakses lewat URL static/container volume langsung;
- [ ] logs/error/diagnostics tidak menampilkan object key atau customer contact;
- [ ] admin vs seller visibility rule terdokumentasi.

## Residual risk

- Malware scanning belum ada pada MVP.
- Manual transfer tetap memiliki risiko human error verifikasi.
