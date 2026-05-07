<svelte:head>
  <title>Install Panel — market.osource.id</title>
</svelte:head>

<main class="mx-auto flex min-h-screen max-w-4xl flex-col justify-center px-4 py-8">
  <section class="panel overflow-hidden">
    <div class="border-b border-border bg-muted/60 p-6">
      <p class="text-sm font-semibold uppercase tracking-[0.2em] text-primary">Install panel</p>
      <h1 class="mt-2 text-3xl font-bold tracking-tight">Setup marketplace pertama</h1>
      <p class="mt-2 max-w-2xl text-sm leading-6 text-muted-foreground">
        Panel ini akan mengunci setup server-side setelah Super Admin dibuat. Scaffold belum menjalankan mutation setup.
      </p>
    </div>

    <div class="grid gap-6 p-6 md:grid-cols-[1fr_280px]">
      <form class="space-y-5" aria-label="Install skeleton form">
        <label class="block text-sm font-medium">
          Nama marketplace
          <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" placeholder="Market Osource" disabled />
        </label>
        <label class="block text-sm font-medium">
          Email Super Admin
          <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" placeholder="admin@example.com" disabled />
        </label>
        <button class="rounded-xl bg-primary px-5 py-3 text-sm font-semibold text-primary-foreground" disabled>Setup belum aktif</button>
      </form>

      <aside class="rounded-2xl border border-border bg-background p-5">
        <h2 class="font-semibold">Preflight</h2>
        <ul class="mt-4 space-y-3 text-sm">
          <li class="flex items-center justify-between gap-3"><span>Backend</span><span class="status-chip border-success/30 bg-success/10 text-success">Ready</span></li>
          <li class="flex items-center justify-between gap-3"><span>Database</span><span class="status-chip border-warning/30 bg-warning/10 text-warning">Pending</span></li>
          <li class="flex items-center justify-between gap-3"><span>Storage</span><span class="status-chip border-primary/30 bg-primary/10 text-primary">Configured</span></li>
        </ul>
      </aside>
    </div>
  </section>
</main>
