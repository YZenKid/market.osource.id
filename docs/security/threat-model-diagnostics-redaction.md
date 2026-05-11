# Threat Model — Diagnostics Redaction

## Asset utama

- runtime/config summary,
- readiness/install/package status,
- logs/evidence exports yang mungkin dibagikan operator ke support.

## Ancaman utama

1. Secret DSN/token/path bocor lewat export.
2. Proof object key, customer contact, atau alamat terkirim ke file diagnostics.
3. Endpoint diagnostics dapat diakses user non-admin.
4. Endpoint read-only berubah menjadi helper yang menulis state tambahan.

## Kontrol wajib

- Endpoint diagnostics harus admin-only.
- Secret dan path lokal direpresentasikan sebagai redacted presence flags, bukan raw string.
- Export tidak boleh mengandung customer/order payload, proof object key, atau filename sensitif.
- Ada unit/integration test yang memverifikasi substrings sensitif tidak muncul pada JSON export.
- Export harus read-only: tidak boleh sync entitlement, mutate package state, atau menulis file.

## Review checklist

- [ ] route memerlukan `require_super_admin`;
- [ ] test memastikan DSN/path/object-key/PII tidak muncul;
- [ ] summary cukup untuk support triage tanpa membuka rahasia operator;
- [ ] docs operator menjelaskan export aman untuk support sharing dengan tetap review manual.

## Residual risk

- Dukungan support tetap harus menghindari meminta upload file proof/database dump mentah tanpa kanal aman.
