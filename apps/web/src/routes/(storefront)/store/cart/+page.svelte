<script lang="ts">
  import StoreShell from '$lib/ui/StoreShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  const cartGroups = [
    { brand: 'Brand Utama', items: [{ name: 'Kopi Arabika Lokal', qty: 1, price: 75000 }] },
    { brand: 'Brand Partner', items: [{ name: 'Paket Hampers Komunitas', qty: 1, price: 125000 }] }
  ];

  const total = cartGroups.flatMap((group) => group.items).reduce((sum, item) => sum + item.price * item.qty, 0);
</script>

<svelte:head>
  <title>Cart — market.osource.id</title>
</svelte:head>

<StoreShell active="cart">
  <div class="mx-auto grid max-w-5xl gap-6 px-4 py-8 sm:px-6 lg:grid-cols-[1fr_340px] lg:px-8">
    <section class="space-y-4">
      <div class="panel p-6">
        <p class="text-sm font-semibold uppercase tracking-[0.2em] text-primary">Cart foundation</p>
        <h1 class="mt-2 text-2xl font-bold">Cart lintas brand</h1>
        <p class="mt-2 text-sm leading-6 text-muted-foreground">Contoh state cart mock-safe untuk memvalidasi grouping brand dan ringkasan checkout. Backend saat ini menyediakan checkout submission, bukan persistent cart UI.</p>
      </div>

      <StateNotice tone="warning" title="Mock-safe cart state" message="Item di halaman ini adalah fallback statis. Integrasi cart persistent harus mengikuti endpoint /api/storefront/cart saat tersedia." />

      {#each cartGroups as group}
        <article class="panel p-5">
          <div class="flex items-center justify-between gap-4 border-b border-border pb-4">
            <h2 class="font-semibold">{group.brand}</h2>
            <StatusBadge tone="primary" label="Brand group" />
          </div>
          <div class="divide-y divide-border">
            {#each group.items as item}
              <div class="flex items-center justify-between gap-4 py-4">
                <div>
                  <p class="font-medium">{item.name}</p>
                  <p class="mt-1 text-sm text-muted-foreground">Qty {item.qty}</p>
                </div>
                <p class="font-semibold tabular-nums">Rp{(item.price * item.qty).toLocaleString('id-ID')}</p>
              </div>
            {/each}
          </div>
        </article>
      {/each}
    </section>

    <aside class="panel h-fit p-6 lg:sticky lg:top-6">
      <h2 class="font-semibold">Ringkasan</h2>
      <dl class="mt-4 space-y-3 text-sm">
        <div class="flex justify-between gap-4"><dt class="text-muted-foreground">Subtotal</dt><dd class="font-semibold tabular-nums">Rp{total.toLocaleString('id-ID')}</dd></div>
        <div class="flex justify-between gap-4"><dt class="text-muted-foreground">Pembayaran</dt><dd>Manual transfer</dd></div>
      </dl>
      <p class="mt-4 text-sm leading-6 text-muted-foreground">Upload bukti pembayaran dilakukan setelah order dibuat dan tetap melalui endpoint protected, bukan public static media.</p>
      <a class="mt-5 inline-flex min-h-11 w-full items-center justify-center rounded-xl bg-primary px-4 py-3 text-sm font-semibold text-primary-foreground" href="/store/checkout">Lanjut checkout</a>
    </aside>
  </div>
</StoreShell>
