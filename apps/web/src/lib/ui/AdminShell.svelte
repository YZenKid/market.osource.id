<script lang="ts">
  export let active: 'dashboard' | 'brands' | 'products' | 'packages' | 'system' = 'dashboard';
  export let eyebrow = 'Operational control center';
  export let title: string;
  export let description: string;

  const navItems = [
    { key: 'dashboard', href: '/admin', label: 'Dashboard', helper: 'Status & next actions' },
    { key: 'brands', href: '/admin/brands', label: 'Brands', helper: 'Marketplace brand units' },
    { key: 'products', href: '/admin/products', label: 'Products', helper: 'Catalog & stock readiness' },
    { key: 'packages', href: '/admin/packages', label: 'Packages', helper: 'Open-core capability gate' },
    { key: 'system', href: '/admin/system', label: 'System', helper: 'Runtime, storage, tunnel' }
  ] as const;
</script>

<main class="min-h-screen bg-background">
  <div class="mx-auto grid max-w-7xl gap-6 px-4 py-6 sm:px-6 lg:grid-cols-[260px_1fr] lg:px-8">
    <aside class="panel h-fit p-4 lg:sticky lg:top-6" aria-label="Admin workspace">
      <a class="block rounded-xl px-2 py-2 focus-visible:outline-primary" href="/">
        <p class="text-xs font-semibold uppercase tracking-[0.2em] text-primary">Admin</p>
        <p class="mt-1 text-sm font-semibold text-foreground">market.osource.id</p>
      </a>
      <nav class="mt-4 grid gap-1 text-sm" aria-label="Admin navigation">
        {#each navItems as item}
          <a
            class={`rounded-xl px-3 py-3 transition-colors hover:bg-muted focus-visible:outline-primary ${
              active === item.key ? 'bg-muted font-semibold text-foreground' : 'text-muted-foreground'
            }`}
            aria-current={active === item.key ? 'page' : undefined}
            href={item.href}
          >
            <span class="block">{item.label}</span>
            <span class="mt-0.5 block text-xs font-normal text-muted-foreground">{item.helper}</span>
          </a>
        {/each}
      </nav>
    </aside>

    <section class="space-y-6">
      <header class="panel p-6">
        <p class="text-sm font-semibold uppercase tracking-[0.2em] text-primary">{eyebrow}</p>
        <div class="mt-2 flex flex-col gap-4 md:flex-row md:items-start md:justify-between">
          <div>
            <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">{title}</h1>
            <p class="mt-2 max-w-3xl text-sm leading-6 text-muted-foreground">{description}</p>
          </div>
          <slot name="header-action" />
        </div>
      </header>

      <slot />
    </section>
  </div>
</main>
