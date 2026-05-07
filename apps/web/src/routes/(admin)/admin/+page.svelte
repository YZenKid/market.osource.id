<svelte:head>
  <title>Admin — market.osource.id</title>
</svelte:head>

<main class="min-h-screen bg-background">
  <div class="mx-auto grid max-w-7xl gap-6 px-4 py-6 lg:grid-cols-[240px_1fr]">
    <aside class="panel h-fit p-4">
      <p class="px-2 text-xs font-semibold uppercase tracking-[0.2em] text-primary">Admin</p>
      <nav class="mt-4 grid gap-1 text-sm" aria-label="Admin navigation">
        <a class="rounded-lg bg-muted px-3 py-2 font-medium" href="/admin">Dashboard</a>
        <a class="rounded-lg px-3 py-2 text-muted-foreground" href="/admin/orders">Orders</a>
        <a class="rounded-lg px-3 py-2 text-muted-foreground" href="/admin/packages">Packages</a>
      </nav>
    </aside>

    <section class="space-y-6">
      <header class="panel p-6">
        <p class="text-sm font-semibold uppercase tracking-[0.2em] text-primary">Operational control center</p>
        <h1 class="mt-2 text-3xl font-bold tracking-tight">Status marketplace</h1>
        <p class="mt-2 max-w-2xl text-sm leading-6 text-muted-foreground">
          Admin dashboard awal menekankan health, install lock, payment verification, brand assignment, dan package status.
        </p>
      </header>

      <div class="grid gap-4 md:grid-cols-3">
        <article class="panel p-5">
          <p class="text-sm text-muted-foreground">Payment proof</p>
          <p class="mt-2 text-2xl font-bold tabular-nums">0</p>
          <span class="status-chip mt-4 border-warning/30 bg-warning/10 text-warning">Waiting verification</span>
        </article>
        <article class="panel p-5">
          <p class="text-sm text-muted-foreground">Tunnel</p>
          <p class="mt-2 text-2xl font-bold">Off</p>
          <span class="status-chip mt-4 border-border bg-muted text-muted-foreground">Opt-in</span>
        </article>
        <article class="panel p-5">
          <p class="text-sm text-muted-foreground">Package gate</p>
          <p class="mt-2 text-2xl font-bold">Base</p>
          <span class="status-chip mt-4 border-primary/30 bg-primary/10 text-primary">Core enabled</span>
        </article>
      </div>
    </section>
  </div>
</main>
