# Threat Model — Entitlement Privacy

## Asset utama

- status entitlement package berbayar,
- identitas instalasi/operator,
- capability state,
- telemetry/cache status untuk verifikasi entitlement.

## Ancaman utama

1. Payload entitlement memuat PII/operator metadata berlebih.
2. Offline cache menyimpan secret/token tanpa redaksi yang cukup.
3. Diagnostics export membocorkan token/verdict/cache internals.
4. Fitur paid aktif walau status entitlement invalid/unlicensed.

## Kontrol wajib

- Payload entitlement minim: kirim hanya metadata yang diperlukan untuk verifikasi lisensi.
- Cache/verdict disimpan dengan prinsip least data dan tanpa secret tampil di UI/log/diagnostics.
- Server-side feature gate wajib menentukan capability aktif/tidak aktif.
- Status diagnostik entitlement cukup berupa state ringkas (`active`, `grace`, `unreachable`, dll.), bukan token mentah.

## Review checklist

- [ ] docs/privacy entitlement menyebut field minimum yang boleh dikirim;
- [ ] feature gate server-side ada untuk endpoint paid;
- [ ] diagnostics tidak menampilkan bearer token, signed blob, atau installation fingerprint mentah;
- [ ] fallback `unreachable/grace` policy terdokumentasi.

## Residual risk

- Desain final privacy payload dan grace-period masih pertanyaan terbuka lintas release gate.
