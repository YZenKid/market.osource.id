# Release Artifact Manifest & Operations Notes

## Manifest schema

Machine-readable schema tersedia di `docs/release/manifest.schema.json`.

Manifest dipakai untuk mencatat:

- identitas release,
- artifact per platform/runtime,
- checksum,
- SBOM reference,
- health/smoke evidence reference,
- rollback dan upgrade notes reference.

## Example manifest shape

```json
{
  "schema_version": "1.0.0",
  "release": {
    "version": "0.1.0-rc.1",
    "channel": "internal-rc",
    "commit": "abcdef123456",
    "created_at": "2026-05-10T00:00:00Z"
  },
  "artifacts": [
    {
      "name": "market-backend-linux-x86_64",
      "kind": "backend-binary",
      "platform": "linux-x86_64",
      "path": "dist/market-backend-linux-x86_64.tar.gz",
      "sha256": "...",
      "sbom": "dist/sbom/market-backend-linux-x86_64.spdx.json"
    }
  ],
  "evidence": {
    "ci_workflow": "release-foundation",
    "smoke_test_notes": ".opencode/evidence/.../gate-i-j-ops-release-foundation.md"
  },
  "notes": {
    "known_limitations": "docs/release/release-notes-template.md",
    "rollback": "docs/release/release-notes-template.md",
    "upgrade": "docs/release/release-notes-template.md"
  }
}
```

## Checksums

Gunakan salah satu command berikut dari root repo:

```bash
sha256sum dist/* > dist/SHA256SUMS
```

atau untuk subset tertentu:

```bash
sha256sum dist/market-backend-* dist/web-* > dist/SHA256SUMS
```

## SBOM

SBOM **opsional tetapi direkomendasikan**. Jangan jadikan hard dependency CI bila tool belum tersedia.

Jika `syft` tersedia lokal/CI:

```bash
syft packages dir:. -o spdx-json=dist/sbom/repository.spdx.json
syft packages "docker-archive:dist/market-backend-image.tar" -o spdx-json=dist/sbom/backend-image.spdx.json
```

Jika `grype` tersedia untuk advisory scan tambahan:

```bash
grype sbom:dist/sbom/repository.spdx.json
```

## Known limitations / rollback / upgrade notes

Sebelum RC/public release, setiap manifest harus menautkan note yang menjawab:

- known limitations yang masih diterima,
- langkah rollback non-destruktif,
- langkah upgrade termasuk migration dan backup prerequisites.

Template siap pakai tersedia di `docs/release/release-notes-template.md`.
