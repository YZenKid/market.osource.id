<svelte:head>
  <title>market.osource.id</title>
  <meta name="description" content="Self-hosted multi-brand marketplace platform." />
</svelte:head>

<main class="mx-auto flex min-h-screen max-w-6xl flex-col justify-center px-6 py-12">
  <section class="grid gap-8 lg:grid-cols-[1.15fr_0.85fr] lg:items-center">
    <div>
      <p class="mb-4 text-sm font-semibold uppercase tracking-[0.24em] text-primary">Operator-owned commerce</p>
      <h1 class="max-w-3xl text-4xl font-bold tracking-tight text-foreground sm:text-5xl">
        Marketplace multi-brand yang bisa di-host sendiri, dari desktop sampai VPS.
      </h1>
      <p class="mt-5 max-w-2xl text-base leading-7 text-muted-foreground">
        Self-hosted, open-core, dan sepenuhnya milik operator. Storefront responsive, admin dashboard, install panel, dan payment proof private — tanpa bergantung pada SaaS hosted.
      </p>
      <div class="mt-8 flex flex-wrap gap-3">
        <a class="rounded-xl bg-primary px-5 py-3 text-sm font-semibold text-primary-foreground" href="/install">
          Buka install panel
        </a>
        <a class="rounded-xl border border-border bg-surface px-5 py-3 text-sm font-semibold" href="/admin">
          Lihat admin
        </a>
        <a class="rounded-xl border border-border bg-surface px-5 py-3 text-sm font-semibold" href="/store">
          Lihat storefront
        </a>
      </div>
    </div>

    <div class="overflow-hidden rounded-2xl border border-border bg-surface shadow-panel">
      <img
        src="/assets/hero-landing.png"
        alt="Illustration of a self-hosted marketplace platform with interconnected brand storefronts, local server, and secure data vault"
        class="w-full object-cover"
        width="1216"
        height="640"
        loading="eager"
      />
    </div>
  </section>

  <section class="mt-12 grid gap-4 sm:grid-cols-3" aria-label="Product pillars">
    {#each [
      { title: 'Ownership', copy: 'Operator kontrol domain, data, storage, dan runtime sendiri. Tidak ada SaaS lock-in.' },
      { title: 'Reliability', copy: 'Status sistem, install state, tunnel, backup, dan order/payment state selalu terlihat jelas.' },
      { title: 'Commerce clarity', copy: 'Customer browse, cart, checkout, upload proof, dan track order tanpa perlu pengetahuan teknis.' }
    ] as pillar}
      <article class="panel p-5">
        <h2 class="font-semibold text-foreground">{pillar.title}</h2>
        <p class="mt-2 text-sm leading-6 text-muted-foreground">{pillar.copy}</p>
      </article>
    {/each}
  </section>

  <aside class="mt-6 panel p-6" aria-label="Status scaffold">
    <h2 class="text-lg font-semibold">Foundation status</h2>
    <dl class="mt-5 space-y-4 text-sm">
      <div class="flex items-center justify-between gap-4">
        <dt class="text-muted-foreground">Runtime</dt>
        <dd class="status-chip border-primary/30 bg-primary/10 text-primary">Axum + SvelteKit</dd>
      </div>
      <div class="flex items-center justify-between gap-4">
        <dt class="text-muted-foreground">Storage MVP</dt>
        <dd class="status-chip border-warning/30 bg-warning/10 text-warning">Local private-ready</dd>
      </div>
      <div class="flex items-center justify-between gap-4">
        <dt class="text-muted-foreground">Packages</dt>
        <dd class="status-chip border-border bg-muted text-muted-foreground">Registry boundary</dd>
      </div>
    </dl>
  </aside>
</main>
