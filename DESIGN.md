# market.osource.id Design System

> Category: Self-hosted open-core multi-brand marketplace for operators, sellers, and web customers across desktop-hosted and VPS-hosted deployments.
> Source: Generated from the current project structure, existing UI patterns, and user-provided hints.

## Visual Theme & Atmosphere

`market.osource.id` should feel like a trustworthy operator-owned commerce control center, not a playful consumer-only storefront and not a generic SaaS dashboard. The visual tone is practical, calm, technical enough for self-hosted operators, and still warm enough for small marketplace communities and local brands.

The UI should communicate three product promises:

- **Ownership** — operators control their marketplace, domain, data, storage, and runtime.
- **Reliability** — system status, install state, tunnel status, backup readiness, and order/payment state must be visibly understandable.
- **Commerce clarity** — customers should browse, cart, checkout, upload proof, and track orders without needing technical knowledge.

Use a clean, modern, utility-first SvelteKit interface built with Tailwind CSS and TweakCN. Prefer composed surfaces, legible hierarchy, precise state indicators, and strong form clarity over decorative gradients or overly animated marketing UI.

The product has three visual modes that must feel related:

- **Storefront:** approachable, product-first, responsive, brand-aware.
- **Admin dashboard:** dense but readable, operational, status-forward.
- **Install/control panel:** calm, guided, diagnostic, and explicit about risk.

Project-local `DESIGN.md` wins over generic preferences. UI/design work must read this file first, then `design-system/DESIGN.md` or any documented project-specific equivalent if one is added later.

## Color Palette & Roles

Use semantic roles first, then map them to Tailwind/TweakCN tokens during implementation. Do not introduce arbitrary one-off colors in components.

Recommended initial palette direction:

- **Background:** warm neutral off-white or slate-tinted white for storefront and admin app shells.
- **Surface:** white or near-white cards/panels with subtle borders.
- **Elevated surface:** slightly brighter surface with shadow only when layering is meaningful.
- **Text:** deep slate/charcoal for high readability.
- **Muted text:** gray/slate for metadata, descriptions, timestamps, helper text.
- **Border:** soft neutral border; visible enough for forms/tables but never heavy black lines.
- **Primary accent:** deep teal or blue-green for primary actions, ownership, and system trust.
- **Secondary accent:** amber or muted gold for commerce highlights, package upsells, and warnings that are not destructive.
- **Success:** green for confirmed payments, healthy runtime, completed setup, published state.
- **Warning:** amber for pending verification, tunnel warnings, backup reminders, stock low.
- **Destructive:** red for rejected payment, failed service, delete/archive, cancelled order.
- **Focus:** high-contrast ring using the primary accent with enough offset to remain visible on light and dark surfaces.
- **Data visualization:** use a restrained categorical set: teal, blue, amber, violet, green, red. Avoid rainbow dashboards unless analytics package specifically defines data-viz tokens.

State color semantics must stay consistent:

- `pending`, `waiting_payment_verification`, `not_ready` → warning/amber family.
- `confirmed`, `ready_to_process`, `completed`, healthy runtime → success/green family.
- `processing`, `in_progress`, `shipped`, active tunnel → primary/blue-teal family.
- `rejected`, `cancelled`, service failure → destructive/red family.
- Disabled package or inactive brand/product → muted neutral.

For package/module states:

- Base feature: neutral or primary-subtle badge.
- Package available: secondary-subtle badge.
- Package installed/enabled: success or primary badge.
- Package disabled/unlicensed: muted or warning badge depending on actionability.

## Typography Rules

Use a modern sans-serif stack compatible with SvelteKit/Tailwind and TweakCN. If a font is chosen later, prefer **Inter**, **Geist**, **Plus Jakarta Sans**, or **Public Sans** for strong dashboard and storefront readability.

Typography rules:

- **Display headings:** use sparingly for storefront hero, install welcome, and major landing sections. Use strong weight, tight but readable tracking, and avoid oversized generic hero typography.
- **Page headings:** clear, direct, 24–36px depending on viewport. Include supporting context when the page involves setup, payment verification, package status, or runtime state.
- **Section headings:** 16–20px, medium/semibold, paired with short helper text for admin/settings forms.
- **Body text:** 14–16px with comfortable line-height. Storefront product descriptions may use 16px; dense admin metadata may use 13–14px.
- **Labels:** 12–14px semibold/medium. Labels must be visible, not placeholder-only.
- **Captions/helper text:** 12–13px muted text. Use for file restrictions, tunnel risk notes, status explanations, and package compatibility notes.
- **Numeric values:** use tabular numerals where possible for prices, stock, order counts, dashboard metrics, and totals.
- **Code/mono:** use only for technical values: domain, local URL, tunnel URL, version, package capability, migration version, object key, service port.

Avoid decorative font pairing until a real brand/theme package exists. The base product should prioritize durable readability and easy white-labeling.

## Component Stylings

Prefer TweakCN-compatible component anatomy and Tailwind tokens. Components should be predictable, accessible, and state-rich.

- **Buttons:** clear hierarchy: primary, secondary, outline, ghost, destructive. Primary is for one main action per surface. Destructive actions require confirmation when data-impacting. Loading buttons must preserve width and show progress state.
- **Inputs/forms:** visible labels, helper text, validation text, and clear disabled/loading states. Use grouped sections for install setup, storage settings, tunnel credentials, and package configuration.
- **Cards:** use for product cards, brand cards, package cards, status panels, and setup steps. Cards should have subtle border and controlled padding; avoid floating card overload.
- **Navigation:** admin navigation should be functional and persistent on desktop/tablet. Storefront navigation should prioritize search, cart, brand/category discovery, and order lookup.
- **Tables/lists:** admin product/order/package views may use tables on desktop and card/list rows on mobile. Always include empty, loading, filtered-empty, and error states.
- **Dialogs/drawers:** use dialogs for confirmations and small forms; use drawers or dedicated pages for larger edit flows. Payment proof review should show image/file preview, order context, and verify/reject actions together.
- **Feedback surfaces:** use inline alerts for form errors and callouts for operational risk. Runtime failures need actionable copy, not just red badges.
- **Empty states:** should explain next action: create first brand, add product, configure tunnel, upload proof, install package, run setup.
- **Icons:** use a consistent outline icon style. Do not use random emoji as primary UI icons. Numeric-only service icons are not allowed.
- **Imagery:** product images and brand logos are content, not decoration. Missing images should use intentional neutral placeholders with clear alt strategy.
- **Charts:** reserved for future analytics/package surfaces. Use restrained colors, labeled axes, and avoid decorative 3D/chart junk.

Critical component states to design from the start:

- default, hover, focus, active, disabled
- loading, skeleton, empty
- validation error, system error, success
- unauthenticated, permission denied
- offline/unavailable local service
- tunnel disconnected/connected
- package disabled/enabled/unlicensed

## Layout Principles

Use a mobile-first responsive layout, but optimize admin density for desktop/tablet because operators and sellers will manage products/orders there.

Recommended structure:

- **Storefront max width:** centered content with generous product grid spacing; keep checkout narrow and focused.
- **Admin max width:** wider workspace with stable sidebar/header; avoid overly centered dashboard layouts.
- **Install panel max width:** guided wizard layout, 640–880px content width, with preflight/status panel clearly visible.
- **Spacing rhythm:** use a consistent 4px-based or Tailwind default spacing scale. Prefer 16/24/32px section rhythm; avoid inconsistent arbitrary spacing.
- **Grid:** product grids should adapt from 1 column mobile, 2 columns small/tablet, 3–4 columns desktop depending on content density.
- **Admin lists:** use filters/search/status chips above tables; keep bulk actions and primary action predictable.
- **Sticky/fixed behavior:** cart summary, admin action bars, and install wizard footer may be sticky when useful, but must not cover content on mobile.
- **Hierarchy:** status and next action should be visible above secondary configuration details.

For order lintas brand, group content visually by brand. Super Admin can see all groups; seller views must only show assigned brand groups.

## Depth & Elevation

Use depth to explain interaction and hierarchy, not decoration.

- **Default surfaces:** subtle border, no shadow or minimal shadow.
- **Interactive cards:** border change or background tint on hover; avoid heavy elevation jumps.
- **Dropdowns/popovers/dialogs:** use shadow + overlay/layering with clear focus management.
- **Runtime/status panels:** use border and semantic tint instead of heavy shadows.
- **Focus rings:** always visible, high contrast, and not removed for mouse users unless replaced with an accessible alternative.
- **Overlays:** dialogs and drawers need calm dim overlays. Avoid transparent overlays that reduce readability.
- **Hover/active depth:** use transform or shadow very lightly. Avoid `transition: all`.

Flat surfaces are preferred for admin density. Elevated surfaces are reserved for modals, popovers, selected product cards, checkout summary, and critical status callouts.

## Do's and Don'ts

Do:

- Read `DESIGN.md` first before any UI/design work, then read `design-system/DESIGN.md` or another documented local equivalent if it exists.
- Reuse project tokens, TweakCN components, and existing SvelteKit route patterns once implementation exists.
- Keep storefront customer flows simple and mobile-friendly.
- Make admin/system states explicit: backend, database, storage, tunnel, install lock, package status.
- Use brand grouping for multi-brand orders.
- Use meaningful empty states and action-oriented error messages.
- Treat payment proof, tunnel credentials, package license keys, and DB config as sensitive in UI copy and screenshots.
- Prefer semantic badges over raw status strings.

Don't:

- Do not introduce generic purple/blue gradient SaaS visuals unless explicitly approved.
- Do not use emoji as system icons or commerce category icons.
- Do not create bland centered-card-only admin pages for complex operational workflows.
- Do not hide labels and rely only on placeholders.
- Do not expose payment proof as public imagery.
- Do not use color as the only way to communicate status.
- Do not create arbitrary custom colors outside the semantic palette.
- Do not add chart-heavy analytics to base MVP; advanced analytics belongs to a package.
- Do not make the UI imply SaaS hosted behavior or self-service seller registration.
- Do not introduce native mobile design patterns as a primary requirement; mobile web responsive is the target.

Never introduce new product assumptions, status names, package capabilities, auth models, or deployment modes without updating PRD/TRD/ERD as appropriate.

## Responsive Behavior

Mobile web is required for customers; admin must be usable at least on desktop/tablet and not broken on mobile.

- **Mobile storefront:** bottom or top navigation must keep search/cart/order lookup reachable. Product cards should be scannable with image, brand, name, price, and stock state.
- **Mobile checkout:** single-column, minimal steps, large touch targets, persistent order summary when appropriate, and clear upload proof interaction.
- **Tablet admin:** sidebar may collapse but primary navigation and filters must remain accessible.
- **Desktop admin:** use sidebar + content workspace; keep tables and status panels dense but readable.
- **Data display:** tables become stacked rows/cards on narrow screens. Preserve status, action, and key metadata.
- **Forms:** group fields into readable sections; use full-width fields on mobile and two-column only when labels and errors remain clear.
- **Images:** product imagery should use stable aspect ratios. Avoid layout shift; use skeleton/placeholder while loading.
- **Touch targets:** minimum 44px target for customer-facing mobile actions and critical admin actions.
- **Motion:** use subtle CSS/native transitions for hover/focus/state changes. Respect reduced-motion. Avoid generic entrance animations and unbounded loops.

For substantial UI work, visual validation should include desktop, tablet, and mobile screenshots once the app is runnable.

## Agent Prompt Guide

Future coding/design agents must follow this order:

1. Read `DESIGN.md` first for UI/design direction.
2. Read `PRD.md` for product scope and locked decisions.
3. Read `TRD.md` for architecture, runtime, security, and project structure.
4. Read `ERD.md` for data model, relations, and status canonicalization.
5. If `design-system/DESIGN.md` or another project-specific equivalent exists later, read it after root `DESIGN.md` and reconcile conflicts by project-local guidance.

Application rules:

- Reuse existing tokens/components/styles first once implementation exists.
- Extend TweakCN/Tailwind tokens when a repeated semantic need appears.
- Create new components only when reuse would harm clarity or accessibility.
- Ask targeted questions before inventing a major visual direction, especially for theme customization, storefront branding, package upsell UI, payment proof review, or install wizard UX.
- For substantial UI/design work, produce page-level and state-level plans before implementation.
- Validate UI with screenshots when runnable; include desktop/tablet/mobile evidence for substantial changes.
- Report any deviation from this design system in the final summary with reason and affected files.

If the project is missing additional design guidance for a substantial design change, suggest `/init-design` and ask targeted follow-up questions before inventing a new visual direction.
