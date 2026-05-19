<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/state';
  import { cart } from '$lib/stores/cart';

  export let active: 'store' | 'cart' | 'checkout' = 'store';
  export let marketplaceName = 'Market Osource';

  let cartCount = 0;
  const unsubscribe = cart.subscribe((items) => {
    cartCount = items.reduce((sum, item) => sum + item.quantity, 0);
  });

  onMount(() => {
    cart.hydrate();
    return () => unsubscribe();
  });

  const navItems = [
    { key: 'store', href: '/store', label: 'Produk' },
    { key: 'cart', href: '/store/cart', label: 'Cart' },
    { key: 'checkout', href: '/store/checkout', label: 'Checkout' }
  ] as const;

  const categories = [
    { label: 'Semua', value: '' },
    { label: 'Batik', value: 'batik' },
    { label: 'Casual', value: 'casual' },
    { label: 'Modest', value: 'modest' },
    { label: 'Outerwear', value: 'outerwear' },
    { label: 'Aksesori', value: 'aksesori' }
  ];

  $: activeCategory = page.url.searchParams.get('category') ?? '';
</script>

<main class="min-h-screen bg-background">
  <header class="sticky top-0 z-20 border-b border-border bg-surface/90 backdrop-blur">
    <div class="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
      <!-- Row 1: brand + nav + cart -->
      <div class="flex items-center justify-between gap-4 py-3">
        <a class="shrink-0 rounded-xl focus-visible:outline-primary" href="/store">
          <p class="text-xs font-semibold uppercase tracking-[0.2em] text-primary">Storefront</p>
          <p class="text-base font-bold tracking-tight">{marketplaceName}</p>
        </a>

        <!-- Desktop nav -->
        <nav class="hidden items-center gap-2 md:flex" aria-label="Storefront navigation">
          {#each navItems as item}
            <a
              class={`relative inline-flex min-h-11 items-center rounded-xl border px-4 py-2 text-sm font-semibold transition-colors hover:bg-muted ${
                active === item.key
                  ? 'border-primary/30 bg-primary/10 text-primary'
                  : 'border-border bg-surface text-muted-foreground'
              }`}
              aria-current={active === item.key ? 'page' : undefined}
              href={item.href}
            >
              {item.label}
              {#if item.key === 'cart' && cartCount > 0}
                <span class="ml-2 inline-flex h-5 min-w-5 items-center justify-center rounded-full bg-primary px-1.5 text-xs font-bold text-primary-foreground tabular-nums">
                  {cartCount}
                </span>
              {/if}
            </a>
          {/each}
        </nav>

        <!-- Mobile: cart icon + count -->
        <div class="flex items-center gap-2 md:hidden">
          <a
            class="relative inline-flex min-h-11 min-w-11 items-center justify-center rounded-xl border border-border bg-surface"
            href="/store/cart"
            aria-label={`Cart, ${cartCount} item`}
          >
            <svg class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="1.8" viewBox="0 0 24 24" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 3h1.386c.51 0 .955.343 1.087.835l.383 1.437M7.5 14.25a3 3 0 0 0-3 3h15.75m-12.75-3h11.218c1.121-2.3 2.1-4.684 2.924-7.138a60.114 60.114 0 0 0-16.536-1.84M7.5 14.25 5.106 5.272M6 20.25a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0Zm12.75 0a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0Z" />
            </svg>
            {#if cartCount > 0}
              <span class="absolute -right-1 -top-1 inline-flex h-5 min-w-5 items-center justify-center rounded-full bg-primary px-1 text-xs font-bold text-primary-foreground tabular-nums">
                {cartCount}
              </span>
            {/if}
          </a>
        </div>
      </div>

      <!-- Row 2: category quick links -->
      <div class="flex gap-2 overflow-x-auto pb-3 scrollbar-none" role="navigation" aria-label="Kategori produk">
        {#each categories as cat}
          <a
            class={`inline-flex shrink-0 min-h-9 items-center rounded-xl border px-3 py-1.5 text-xs font-semibold transition-colors ${
              activeCategory === cat.value
                ? 'border-primary/30 bg-primary/10 text-primary'
                : 'border-border bg-surface text-muted-foreground hover:bg-muted hover:text-foreground'
            }`}
            aria-current={activeCategory === cat.value ? 'page' : undefined}
            href={cat.value ? `/store?category=${cat.value}` : '/store'}
          >
            {cat.label}
          </a>
        {/each}
      </div>
    </div>
  </header>

  <slot />
</main>
