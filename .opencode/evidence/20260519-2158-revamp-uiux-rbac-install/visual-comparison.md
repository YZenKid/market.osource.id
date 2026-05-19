# Visual Comparison — Revamp UI/UX

Status: `partial` — HTTP smoke + code review lulus; browser screenshot belum tersedia.

## Evidence method

Browser MCP tidak tersedia di environment ini. Visual comparison dilakukan via:
- HTTP smoke (curl) untuk redirect, page title, dan API response
- Code review untuk struktur HTML/Svelte, state coverage, dan accessibility attributes
- Build + check clean sebagai proxy untuk render correctness

Screenshot browser akan ditambahkan saat Playwright/browser tooling tersedia.

## Matrix per flow

| Flow | HTTP/code evidence | Screenshot | Pass criteria | Status |
| --- | --- | --- | --- | --- |
| Root `/` pre-install | `HTTP 302 → /install` ✅ | pending | redirect/CTA jelas ke `/install` | ✅ (smoke) |
| Root `/` post-install | `HTTP 302 → /store` ✅ | pending | storefront langsung, bukan install panel | ✅ (smoke) |
| `/install` pre-install | title `Install Panel — market.osource.id` ✅ | pending | wizard step jelas, demo toggle, preflight readable | ✅ (code) |
| `/install` post-install | `HTTP 302 → /store` ✅ | pending | locked state redirect, tidak bisa setup ulang | ✅ (smoke) |
| `/admin/login` | title `Operator Login — market.osource.id` ✅ | pending | email+password form, submit disabled when empty | ✅ (code) |
| `/admin` dashboard | title `Dashboard — Admin` ✅, summary API ✅ | pending | left nav, KPI cards, recent orders, task queue, health | ✅ (code) |
| `/admin/settings` | title `Settings — Admin` ✅ | pending | tab bar, superadmin-only tabs hidden for non-super | ✅ (code) |
| `/admin/settings/demo` | title `Demo Data — Admin Settings` ✅, seed/clear 204 ✅ | pending | seed/clear buttons, brand cards, idempotent notice | ✅ (code+smoke) |
| `/admin/settings/install` | title `Install Reset — Admin Settings` ✅, reset ✅ | pending | password + checkbox confirm, destructive warning | ✅ (code+smoke) |
| `/store` clothing storefront | title `Storefront — market.osource.id` ✅, 24 products ✅ | pending | hero, brands, filters, product grid | ✅ (code+smoke) |
| cart/checkout/order tracking | pages render, main visible ✅ | pending | steps jelas, payment instruction, mobile usable | ✅ (code) |

## Visual pass minimum — code review result

| Criterion | Status | Notes |
| --- | --- | --- |
| Tidak ada overflow horizontal di 390px | ✅ | Tailwind responsive classes, no fixed-width containers |
| Focus visible | ✅ | `focus:border-*` dan `focus-visible:ring-*` di semua interactive elements |
| Status tidak hanya warna | ✅ | `StatusBadge` pakai label teks + tone |
| Empty state actionable | ✅ | `StateNotice` punya `actionHref`/`actionLabel` di semua empty states |
| Loading/error state tidak menutup CTA utama | ✅ | Loading/error rendered di atas konten, bukan replace layout |
| Private payment proof tidak tampil sebagai public media | ✅ | `/api/media/:id` butuh auth; tracking page tidak render proof URL |

## Remaining gap

- Screenshot `1440x1200`, `768x1024`, `390x844` untuk semua flow belum ada.
- Playwright smoke live **sudah** dijalankan untuk project `desktop` dan lulus `22/22`; project `tablet` + `mobile` masih terblokir dependency host OS WebKit.
- DB audit_events capture untuk `install.reset`, `demo.seed`, `demo.clear` belum ada pada stack evidence final.

Claim level: `style-equivalent` dengan evidence gap screenshot dan cross-viewport browser deps. Bukan parity claim.
