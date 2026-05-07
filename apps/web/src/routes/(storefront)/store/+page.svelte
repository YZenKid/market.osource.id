<svelte:head>
  <title>Storefront — market.osource.id</title>
</svelte:head>

<main class="mx-auto max-w-6xl px-4 py-8 sm:px-6 lg:px-8">
  <header class="flex flex-col gap-4 border-b border-border pb-6 md:flex-row md:items-end md:justify-between">
    <div>
      <p class="text-sm font-semibold uppercase tracking-[0.2em] text-primary">Storefront MVP</p>
      <h1 class="mt-2 text-3xl font-bold tracking-tight">Belanja lintas brand dalam satu checkout</h1>
      <p class="mt-2 max-w-2xl text-sm leading-6 text-muted-foreground">
        Halaman ini adalah skeleton responsive untuk browse brand, kategori, cart, checkout manual transfer, dan upload bukti pembayaran pada fase berikutnya.
      </p>
    </div>
    <a class="rounded-xl bg-primary px-4 py-3 text-sm font-semibold text-primary-foreground" href="/store/cart">Lihat cart</a>
  </header>

  <section class="mt-8 grid gap-4 sm:grid-cols-2 lg:grid-cols-3" aria-label="Product skeleton cards">
    {#each ['Produk brand utama', 'Produk brand partner', 'Produk stok terbatas'] as name, index}
      <article class="panel overflow-hidden">
        <div class="flex aspect-[4/3] items-center justify-center bg-muted text-sm text-muted-foreground">
          Gambar produk {index + 1}
        </div>
        <div class="p-5">
          <p class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">Brand {index + 1}</p>
          <h2 class="mt-2 text-lg font-semibold">{name}</h2>
          <p class="mt-2 text-sm text-muted-foreground">Placeholder konten sampai katalog DB-backed tersedia.</p>
          <div class="mt-4 flex items-center justify-between">
            <span class="font-semibold tabular-nums">Rp{(75000 + index * 25000).toLocaleString('id-ID')}</span>
            <span class="status-chip border-success/30 bg-success/10 text-success">Published</span>
          </div>
        </div>
      </article>
    {/each}
  </section>
</main>
