# market.osource.id Design System

> Category: Self-hosted multi-brand marketplace for operators (desktop/VPS) with responsive web storefront + admin + install flows.
> Source: Generated from current project structure, existing UI patterns, and repository docs.

## Visual Theme & Atmosphere
Use a **status-forward, operator-trust** visual language: clear hierarchy, low ornament, readable density, and explicit state messaging. The UI should feel operational and calm, not marketing-heavy. Favor practical surfaces (`.panel`) and concise guidance text over decorative blocks. This project-local `DESIGN.md` is the first design authority for UI/design work; read it first, then `design-system/DESIGN.md` (if later added) or another documented project-specific equivalent.

## Color Palette & Roles
Use existing CSS variables from `apps/web/src/lib/styles/app.css` and Tailwind aliases in `apps/web/tailwind.config.ts`.
- Background: `--background` / `bg-background` for page canvas.
- Surface: `--surface` / `bg-surface` for cards, nav, forms.
- Text: `--foreground` / `text-foreground` for primary content.
- Muted text: `--muted-foreground` / `text-muted-foreground` for helper and secondary copy.
- Border: `--border` / `border-border` as default separators.
- Primary accent: `--primary` for primary CTA, active state, key badges.
- Secondary accent: `--secondary` for non-critical highlight (`secondary` badge tone only).
- Success/Warning/Destructive: use `--success`, `--warning`, `--destructive` only for semantic status.
- Focus: `:focus-visible` uses 3px primary outline with offset; do not remove.
- Data-viz colors: keep charts minimal and semantic-first; until chart tokens exist, limit palette to `primary`, `secondary`, `success`, `warning`, `destructive`, plus muted neutral for baselines.

## Typography Rules
Use configured stacks only:
- Sans: `Inter, Geist, Plus Jakarta Sans, Public Sans, system-ui`.
- Mono/numeric: `JetBrains Mono` fallback stack and `tabular-nums` for operational numbers.

Scale and rhythm (match existing pages):
- H1: `text-3xl` to `sm:text-5xl`, `font-bold`, `tracking-tight`.
- Section headings: `text-lg`/`text-2xl`, `font-semibold` or `font-bold` based on emphasis.
- Body: `text-sm` or `text-base`, `leading-6` to `leading-7`.
- Labels/eyebrows: uppercase small labels with wide tracking (`tracking-[0.2em]` range).
- Captions/help: `text-xs` to `text-sm`, muted color.
- Code/log snippets: mono only when technical strings are shown.
Do not introduce display typography styles unrelated to operational readability.

## Component Stylings
- Buttons: rounded-xl, min height 44px (`min-h-11`) for touchability; primary = solid `bg-primary`, secondary = border/surface.
- Inputs/Textareas: rounded-xl, `border-border`, `bg-surface`, generous padding; keep clear error/success messaging nearby.
- Cards/Panels: use `.panel` (rounded-2xl + border + surface + `shadow-panel`) as base surface.
- Navigation:
  - Admin uses left rail with helper text (`AdminShell.svelte`).
  - Storefront uses top nav pills (`StoreShell.svelte`).
  - Active states must be visible through border/background + text change, not color alone.
- Tables/Lists: prefer grouped list rows with clear separators (`divide-border`) and tabular numerics for price/count.
- Dialogs/Drawers: not yet standardized; if added, mirror panel tokens, status tones, and focus behavior from existing surfaces.
- Feedback surfaces: use `StateNotice` tones (`neutral|primary|success|warning|destructive`) and `StatusBadge` tones for compact state chips.
- Empty states: explicit and actionable, with route/API context (example: fallback/static-mode notices).
- Icons: keep simple, status-supportive, never decorative-only for critical meaning.
- Imagery: storefront placeholders are acceptable in scaffold; for production visuals, use real product assets without weakening privacy boundaries.
- Charts: only introduce when product data requires it; style as low-noise, semantic-status compatible.

## Layout Principles
- Grid containers follow current max widths: `max-w-4xl`, `max-w-5xl`, `max-w-6xl`, `max-w-7xl` by screen purpose.
- Spacing rhythm: 4/6/8 scale with section blocks around `p-5`/`p-6` and `gap-4`/`gap-6`.
- Density: operationally compact but breathable; avoid crowded forms and cards.
- Admin layout: sticky contextual side rail on large screens.
- Storefront layout: top nav + content-first sections; checkout/cart summary may be sticky sidebar on desktop.
- Content hierarchy adapts by reducing columns first, then reducing non-critical side content.

## Depth & Elevation
- Default surfaces are mostly flat with border definition; depth is subtle.
- Use `shadow-panel` for primary content panels only.
- Hover/active depth should be mostly border/background tint changes (avoid heavy motion or large shadow jumps).
- Overlays/modals should preserve strong contrast and keep focus ring visible.
- Maintain explicit layering order for sticky header/aside and notices; do not obscure alerts or form actions.

## Do's and Don'ts
Do:
- Keep UI copy explicit about system state, install lock, auth, and protected media constraints.
- Reuse `AdminShell`, `StoreShell`, `StateNotice`, `StatusBadge`, `.panel`, and tokenized colors first.
- Use `tabular-nums` for prices, counts, runtime metrics.
- Preserve reduced-motion behavior already defined in `app.css`.

Don’t:
- Don’t introduce generic SaaS-style gradients/glassy visuals that hide operational clarity.
- Don’t show payment-proof files as public/static media or imply public access in UI.
- Don’t add new ad-hoc color tokens when existing semantic roles cover the need.
- Don’t make color the only carrier of status meaning; pair with labels/text.
- Don’t add major visual direction changes without discussing via `/init-design` for substantial redesigns.

## Responsive Behavior
- Mobile-first implementation; collapse multi-column admin/store blocks to one column before shrinking typography.
- Keep main CTAs reachable and visible on mobile (especially install and checkout flows).
- Nav behavior:
  - Storefront nav remains wrap-friendly pill buttons.
  - Admin rail can stack above content on smaller screens.
- Forms: full-width controls on mobile, minimum touch targets (`min-h-11`), clear inline status/error notices.
- Data display: switch from dense table-like layouts to grouped cards/lists on narrow screens.
- Images/placeholders: maintain stable aspect ratio blocks (e.g., `aspect-[4/3]`) to avoid layout jumps.
- Respect `prefers-reduced-motion`; never require animation for comprehension.

## Agent Prompt Guide
- Read in order before UI work: `DESIGN.md` (this file) → `apps/web/src/lib/styles/app.css` → `apps/web/tailwind.config.ts` → `apps/web/src/lib/ui/*` → relevant route files in `apps/web/src/routes/*`.
- Project-local `DESIGN.md` overrides generic style preferences and default model taste.
- Reuse-first policy: reuse existing tokens/components; extend them second; create new primitives only when reuse/extension cannot satisfy product requirements.
- For substantial UI/design work, if guidance is still insufficient or direction changes materially, run `/init-design` and ask targeted questions (user roles, priority screens, density preference, accessibility requirements, and brand constraints) before inventing a new direction.
- Validate with real evidence: run `npm run check` and `npm run build` in `apps/web`, then capture viewport screenshots for affected flows (store/admin/install) when UI changes are non-trivial.
- Report deviations explicitly: what changed, why existing tokens/components were insufficient, and what new conventions were introduced.
- Ownership chain for substantial UI work: `@designer` owns direction and implementation guidance first; `@visual-parity-auditor`, `@motion-specialist`, `@accessibility-reviewer`, and `@ui-system-architect` are conditional specialist/review lanes; `@quality-gate` is the final cross-cutting signoff lane for non-trivial work.
