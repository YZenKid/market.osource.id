<script lang="ts">
  export let active: 'store' | 'cart' | 'checkout' = 'store';

  const navItems = [
    { key: 'store', href: '/store', label: 'Produk' },
    { key: 'cart', href: '/store/cart', label: 'Cart' },
    { key: 'checkout', href: '/store/checkout', label: 'Checkout' }
  ] as const;
</script>

<main class="min-h-screen bg-background">
  <header class="border-b border-border bg-surface/85 backdrop-blur">
    <div class="mx-auto flex max-w-6xl flex-col gap-4 px-4 py-4 sm:px-6 md:flex-row md:items-center md:justify-between lg:px-8">
      <a class="rounded-xl focus-visible:outline-primary" href="/store">
        <p class="text-xs font-semibold uppercase tracking-[0.2em] text-primary">Storefront</p>
        <p class="mt-1 text-lg font-bold tracking-tight">Market Osource</p>
      </a>
      <nav class="flex flex-wrap gap-2 text-sm" aria-label="Storefront navigation">
        {#each navItems as item}
          <a
            class={`min-h-11 rounded-xl border px-4 py-2.5 font-semibold transition-colors hover:bg-muted ${
              active === item.key ? 'border-primary/30 bg-primary/10 text-primary' : 'border-border bg-surface text-muted-foreground'
            }`}
            aria-current={active === item.key ? 'page' : undefined}
            href={item.href}
          >
            {item.label}
          </a>
        {/each}
      </nav>
    </div>
  </header>

  <slot />
</main>
