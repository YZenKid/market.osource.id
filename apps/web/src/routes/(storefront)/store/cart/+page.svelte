<script lang="ts">
  import { onMount } from 'svelte';
  import { cart, type CartItem } from '$lib/stores/cart';
  import StoreShell from '$lib/ui/StoreShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  const demoImages = ['/assets/demo/product-1.png', '/assets/demo/product-2.png', '/assets/demo/product-3.png'];

  let items: CartItem[] = [];
  const unsubscribe = cart.subscribe((value) => {
    items = value;
  });

  const total = (entries: CartItem[]) => entries.reduce((sum, item) => sum + Number(item.price ?? 0) * item.quantity, 0);
  const grouped = (entries: CartItem[]) =>
    Object.entries(
      entries.reduce<Record<string, CartItem[]>>((groups, item) => {
        const key = item.brandName ?? 'Brand tidak diketahui';
        groups[key] = [...(groups[key] ?? []), item];
        return groups;
      }, {})
    );

  function imageFor(index: number) {
    return demoImages[index % demoImages.length];
  }

  async function increase(item: CartItem) {
    await cart.setQuantity(item.variantId, item.quantity + 1);
  }

  async function decrease(item: CartItem) {
    await cart.setQuantity(item.variantId, item.quantity - 1);
  }

  async function remove(item: CartItem) {
    await cart.remove(item.variantId);
  }

  onMount(() => {
    cart.hydrate();
    return () => unsubscribe();
  });
</script>

<svelte:head>
  <title>Cart — market.osource.id</title>
</svelte:head>

<StoreShell active="cart">
  <div class="mx-auto grid max-w-6xl gap-6 px-4 py-8 sm:px-6 lg:grid-cols-[1fr_360px] lg:px-8">
    <section class="space-y-4">
      <div class="panel p-6">
        <p class="eyebrow">Cart</p>
        <h1 class="mt-2 text-2xl font-bold tracking-tight sm:text-3xl">Review belanja per brand</h1>
        <p class="mt-2 text-sm leading-6 text-muted-foreground">
          Item tetap digabung dalam satu cart, lalu backend membagi fulfillment per brand saat order dibuat. Qty stepper dan subtotal selalu sinkron dengan API storefront.
        </p>
      </div>

      {#if items.length === 0}
        <StateNotice tone="primary" title="Cart masih kosong" message="Tambahkan produk dari storefront untuk memulai checkout lintas brand." actionHref="/store" actionLabel="Kembali ke katalog" />
      {/if}

      {#each grouped(items) as [brand, groupItems], brandIndex}
        <article class="panel p-5">
          <div class="flex items-center justify-between gap-4 border-b border-border pb-4">
            <div>
              <h2 class="font-semibold">{brand}</h2>
              <p class="mt-1 text-sm text-muted-foreground">{groupItems.length} item pada brand ini.</p>
            </div>
            <StatusBadge tone="primary" label="Brand group" />
          </div>
          <div class="divide-y divide-border">
            {#each groupItems as item, itemIndex}
              <div class="grid gap-4 py-4 sm:grid-cols-[88px_1fr_auto] sm:items-center">
                <img
                  src={imageFor(brandIndex + itemIndex)}
                  alt={item.name}
                  class="aspect-[4/5] w-20 rounded-xl border border-border object-cover"
                  loading="lazy"
                />
                <div>
                  <p class="font-medium">{item.name}</p>
                  <p class="mt-1 text-sm text-muted-foreground">{item.description ?? 'Produk marketplace'} </p>
                  <p class="mt-1 text-sm text-muted-foreground">{item.brandName ?? 'Brand'} · stock {item.stock}</p>
                </div>
                <div class="flex flex-col items-start gap-3 sm:items-end">
                  <p class="font-semibold tabular-nums">Rp{(Number(item.price) * item.quantity).toLocaleString('id-ID')}</p>
                  <div class="flex items-center gap-3">
                    <div class="flex items-center rounded-xl border border-border bg-surface">
                      <button class="min-h-11 min-w-11 px-3 text-lg font-semibold" on:click={() => decrease(item)} aria-label={`Kurangi ${item.name}`}>−</button>
                      <span class="min-w-10 text-center text-sm font-semibold tabular-nums">{item.quantity}</span>
                      <button class="min-h-11 min-w-11 px-3 text-lg font-semibold disabled:opacity-40" on:click={() => increase(item)} disabled={item.quantity >= item.stock} aria-label={`Tambah ${item.name}`}>+</button>
                    </div>
                    <button class="inline-flex min-h-11 items-center rounded-xl border border-border px-3 py-2 text-sm font-semibold text-destructive" on:click={() => remove(item)}>Hapus</button>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </article>
      {/each}
    </section>

    <aside class="space-y-4">
      <div class="panel h-fit p-6 lg:sticky lg:top-24">
        <h2 class="font-semibold">Ringkasan belanja</h2>
        <dl class="mt-4 space-y-3 text-sm">
          <div class="flex justify-between gap-4"><dt class="text-muted-foreground">Subtotal</dt><dd class="font-semibold tabular-nums">Rp{total(items).toLocaleString('id-ID')}</dd></div>
          <div class="flex justify-between gap-4"><dt class="text-muted-foreground">Brand aktif</dt><dd class="font-semibold tabular-nums">{grouped(items).length}</dd></div>
          <div class="flex justify-between gap-4"><dt class="text-muted-foreground">Pembayaran</dt><dd>Manual transfer</dd></div>
        </dl>
        <p class="mt-4 text-sm leading-6 text-muted-foreground">Upload bukti pembayaran dilakukan setelah order dibuat dan tetap private melalui endpoint protected.</p>
        <a class="mt-5 inline-flex min-h-11 w-full items-center justify-center rounded-xl bg-primary px-4 py-3 text-sm font-semibold text-primary-foreground" href="/store/checkout">Lanjut checkout</a>
      </div>

      {#if items.length > 0}
        <div class="fixed inset-x-4 bottom-4 z-20 rounded-2xl border border-border bg-surface p-4 shadow-panel lg:hidden">
          <div class="flex items-center justify-between gap-4">
            <div>
              <p class="text-sm text-muted-foreground">Subtotal</p>
              <p class="font-semibold tabular-nums">Rp{total(items).toLocaleString('id-ID')}</p>
            </div>
            <a class="inline-flex min-h-11 items-center rounded-xl bg-primary px-4 py-2.5 text-sm font-semibold text-primary-foreground" href="/store/checkout">Checkout</a>
          </div>
        </div>
      {/if}
    </aside>
  </div>
</StoreShell>
