<script lang="ts">
  import StateNotice from '$lib/ui/StateNotice.svelte';
</script>

<svelte:head>
  <title>market.osource.id</title>
  <meta name="description" content="Install/storefront route gate." />
</svelte:head>

<main class="flex min-h-screen items-center justify-center px-4 py-10 sm:px-6">
  <section class="panel w-full max-w-lg p-6 sm:p-8">
    <p class="eyebrow">Route gate</p>
    <h1 class="mt-2 text-3xl font-bold tracking-tight">Status marketplace belum dapat diverifikasi</h1>
    <p class="mt-3 text-sm leading-6 text-muted-foreground">
      Root route ini hanya berfungsi sebagai gate. Saat install state tersedia, server akan mengarahkan ke <code>/install</code> atau <code>/store</code>.
    </p>

    <div class="mt-6">
      <StateNotice
        tone="warning"
        title="Coba lagi"
        message="API install state belum merespons atau reverse proxy belum mengarahkan /api ke backend."
        actionHref="/install"
        actionLabel="Buka /install"
      />
    </div>

    <div class="mt-4 flex flex-wrap gap-3">
      <a class="inline-flex min-h-11 items-center rounded-xl bg-primary px-4 py-2.5 text-sm font-semibold text-primary-foreground" href="/">Coba lagi</a>
      <a class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-4 py-2.5 text-sm font-semibold" href="/store">Buka /store</a>
    </div>
  </section>
</main>
