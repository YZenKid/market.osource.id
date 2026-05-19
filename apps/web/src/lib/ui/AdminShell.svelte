<script lang="ts">
  type NavKey = 'dashboard' | 'brands' | 'products' | 'orders' | 'settings' | 'packages' | 'system';

  export let active: NavKey = 'dashboard';
  export let title: string;
  export let description: string;
  export let eyebrow = 'Operational control center';
  export let scopeLabel: string | undefined = undefined;
  export let roleLabel: string | undefined = undefined;
  export let userName: string | undefined = undefined;
  export let userEmail: string | undefined = undefined;
  export let healthBadge: 'ok' | 'warn' | 'error' | undefined = undefined;

  // Mobile drawer state
  let drawerOpen = false;
  function toggleDrawer() { drawerOpen = !drawerOpen; }
  function closeDrawer() { drawerOpen = false; }

  $: currentActive = active === 'packages' || active === 'system' ? 'settings' : active;

  const navGroups = [
    {
      label: 'Overview',
      items: [
        { key: 'dashboard', href: '/admin', label: 'Dashboard', helper: 'Status & next actions' }
      ]
    },
    {
      label: 'Commerce',
      items: [
        { key: 'brands', href: '/admin/brands', label: 'Brands', helper: 'Marketplace brand units' },
        { key: 'products', href: '/admin/products', label: 'Products', helper: 'Catalog & stock readiness' },
        { key: 'orders', href: '/admin/orders', label: 'Orders', helper: 'Payment proof & fulfillment' }
      ]
    },
    {
      label: 'Control',
      items: [
        { key: 'settings', href: '/admin/settings', label: 'Settings', helper: 'Packages, system, install reset' }
      ]
    }
  ] as const;

  const healthTone = {
    ok: 'border-success/30 bg-success/10 text-success',
    warn: 'border-warning/30 bg-warning/10 text-warning',
    error: 'border-destructive/30 bg-destructive/10 text-destructive'
  };
  const healthLabel = { ok: 'Healthy', warn: 'Warning', error: 'Error' };
</script>

<!-- Mobile overlay -->
{#if drawerOpen}
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
  <div
    class="fixed inset-0 z-30 bg-foreground/20 backdrop-blur-sm lg:hidden"
    on:click={closeDrawer}
    aria-hidden="true"
  ></div>
{/if}

<main class="min-h-screen bg-background">
  <!-- Mobile top bar -->
  <div class="sticky top-0 z-20 flex items-center justify-between border-b border-border bg-surface/90 px-4 py-3 backdrop-blur lg:hidden">
    <a class="rounded-xl focus-visible:outline-primary" href="/admin">
      <p class="text-xs font-semibold uppercase tracking-[0.2em] text-primary">Admin</p>
      <p class="text-sm font-bold tracking-tight">market.osource.id</p>
    </a>
    <div class="flex items-center gap-2">
      {#if healthBadge}
        <span class={`status-chip ${healthTone[healthBadge]}`}>{healthLabel[healthBadge]}</span>
      {/if}
      <button
        class="inline-flex min-h-11 min-w-11 items-center justify-center rounded-xl border border-border bg-surface"
        on:click={toggleDrawer}
        aria-label="Toggle navigation"
        aria-expanded={drawerOpen}
      >
        <span class="block h-4 w-5 space-y-1" aria-hidden="true">
          <span class="block h-0.5 w-full bg-foreground transition-transform" class:rotate-45={drawerOpen} class:translate-y-1.5={drawerOpen}></span>
          <span class="block h-0.5 w-full bg-foreground transition-opacity" class:opacity-0={drawerOpen}></span>
          <span class="block h-0.5 w-full bg-foreground transition-transform" class:-rotate-45={drawerOpen} class:-translate-y-1.5={drawerOpen}></span>
        </span>
      </button>
    </div>
  </div>

  <div class="mx-auto grid max-w-7xl gap-0 lg:grid-cols-[280px_1fr]">
    <!-- Left rail / drawer -->
    <aside
      class={`fixed inset-y-0 left-0 z-40 flex w-72 flex-col border-r border-border bg-surface transition-transform duration-[220ms] lg:static lg:z-auto lg:w-auto lg:translate-x-0 lg:transition-none ${drawerOpen ? 'translate-x-0' : '-translate-x-full'}`}
      aria-label="Admin workspace"
    >
      <!-- Brand block -->
      <div class="border-b border-border p-5">
        <a class="block rounded-xl focus-visible:outline-primary" href="/admin">
          <p class="text-xs font-semibold uppercase tracking-[0.2em] text-primary">Admin</p>
          <p class="mt-1 text-base font-bold tracking-tight">market.osource.id</p>
        </a>
        {#if scopeLabel}
          <p class="mt-2 text-xs text-muted-foreground">{scopeLabel}</p>
        {/if}
      </div>

      <!-- Nav groups -->
      <nav class="flex-1 overflow-y-auto p-3" aria-label="Admin navigation">
        {#each navGroups as group}
          <div class="mb-4">
            <p class="mb-1 px-3 text-xs font-semibold uppercase tracking-[0.2em] text-muted-foreground">{group.label}</p>
            {#each group.items as item}
              <a
                class={`flex flex-col rounded-xl px-3 py-2.5 transition-colors hover:bg-muted focus-visible:outline-primary ${currentActive === item.key ? 'bg-muted font-semibold text-foreground' : 'text-muted-foreground'}`}
                aria-current={currentActive === item.key ? 'page' : undefined}
                href={item.href}
                on:click={closeDrawer}
              >
                <span class="text-sm">{item.label}</span>
                <span class="mt-0.5 text-xs font-normal text-muted-foreground">{item.helper}</span>
              </a>
            {/each}
          </div>
        {/each}
      </nav>

      <!-- Footer user card -->
      <div class="border-t border-border p-4">
        {#if userName}
          <div class="flex items-start justify-between gap-3">
            <div class="min-w-0">
              <p class="truncate text-sm font-semibold">{userName}</p>
              {#if userEmail}
                <p class="truncate text-xs text-muted-foreground">{userEmail}</p>
              {/if}
            </div>
            {#if roleLabel}
              <span class="status-chip shrink-0 border-primary/30 bg-primary/10 text-primary">{roleLabel}</span>
            {/if}
          </div>
          <a
            class="mt-3 inline-flex min-h-11 w-full items-center justify-center rounded-xl border border-border bg-background px-3 py-2 text-sm font-semibold"
            href="/admin/login"
          >Logout / ganti akun</a>
        {:else}
          <a
            class="inline-flex min-h-11 w-full items-center justify-center rounded-xl bg-primary px-3 py-2 text-sm font-semibold text-primary-foreground"
            href="/admin/login"
          >Login operator</a>
        {/if}
      </div>
    </aside>

    <!-- Main content -->
    <section class="min-w-0 px-4 py-6 sm:px-6 lg:px-8">
      <!-- Sticky top bar (desktop) -->
      <div class="mb-6 hidden items-center justify-between gap-4 lg:flex">
        <div class="flex items-center gap-2 text-sm text-muted-foreground">
          <a class="hover:text-foreground" href="/admin">Admin</a>
          {#if active !== 'dashboard'}
            <span aria-hidden="true">/</span>
            <span class="capitalize text-foreground">{active}</span>
          {/if}
        </div>
        <div class="flex items-center gap-3">
          {#if healthBadge}
            <span class={`status-chip ${healthTone[healthBadge]}`}>{healthLabel[healthBadge]}</span>
          {/if}
          {#if roleLabel}
            <span class="status-chip border-primary/30 bg-primary/10 text-primary">{roleLabel}</span>
          {/if}
          <slot name="topbar-action" />
          {#if userName}
            <span class="text-sm font-semibold">{userName}</span>
          {:else}
            <a class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-3 py-2 text-sm font-semibold" href="/admin/login">Login</a>
          {/if}
        </div>
      </div>

      <!-- Page header panel -->
      <header class="panel mb-6 p-6">
        <p class="eyebrow">{eyebrow}</p>
        <div class="mt-2 flex flex-col gap-4 md:flex-row md:items-start md:justify-between">
          <div>
            <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">{title}</h1>
            <p class="mt-2 max-w-3xl text-sm leading-6 text-muted-foreground">{description}</p>
          </div>
          <slot name="header-action" />
        </div>
      </header>

      <div class="space-y-6">
        <slot />
      </div>
    </section>
  </div>
</main>
