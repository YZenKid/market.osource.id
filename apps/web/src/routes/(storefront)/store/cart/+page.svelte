<script lang="ts">
  import { onMount } from 'svelte';
  import { cart, type CartItem } from '$lib/stores/cart';
  import StoreShell from '$lib/ui/StoreShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  let items: CartItem[] = [];

  const unsubscribe = cart.subscribe((value) => {
    items = value;
  });

  const total = (entries: CartItem[]) =>
    entries.reduce((sum, item) => sum + Number(item.price ?? 0) * item.quantity, 0);

  const grouped = (entries: CartItem[]) =>
    Object.entries(
      entries.reduce<Record<string, CartItem[]>>((groups, item) => {
        const key = item.brandName ?? 'Brand tidak diketahui';
        groups[key] = [...(groups[key] ?? []), item];
        return groups;
      }, {})
    );

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
  <div class="mx-auto grid max-w-5xl gap-6 px-4 py-8 sm:px-6 lg:grid-cols-[1fr_340px] lg:px-8">
    <section class="space-y-4">
      <div class="panel p-6">
        <p class="text-sm font-semibold uppercase tracking-[0.2em] text-primary">Cart foundation</p>
        <h1 class="mt-2 text-2xl font-bold">Cart lintas brand</h1>
        <p class="mt-2 text-sm leading-6 text-muted-foreground">Cart guest sekarang dipersist lewat API storefront agar halaman cart dan checkout memakai sumber data yang sama sebelum order dikirim ke backend checkout.</p>
      </div>

      {#if items.length === 0}
        <StateNotice tone="primary" title="Cart masih kosong" message="Tambahkan produk dari storefront untuk memulai checkout lintas brand." actionHref="/store" actionLabel="Kembali ke katalog" />
      {/if}

      {#each grouped(items) as [brand, groupItems]}
        <article class="panel p-5">
          <div class="flex items-center justify-between gap-4 border-b border-border pb-4">
            <h2 class="font-semibold">{brand}</h2>
            <StatusBadge tone="primary" label="Brand group" />
          </div>
          <div class="divide-y divide-border">
            {#each groupItems as item}
              <div class="flex flex-col gap-4 py-4 sm:flex-row sm:items-center sm:justify-between">
                <div>
                  <p class="font-medium">{item.name}</p>
                  <p class="mt-1 text-sm text-muted-foreground">Rp{Number(item.price).toLocaleString('id-ID')} · stock {item.stock}</p>
                </div>
                <div class="flex items-center gap-3">
                  <div class="flex items-center rounded-xl border border-border bg-surface">
                    <button class="min-h-11 min-w-11 px-3 text-lg font-semibold" on:click={() => decrease(item)} aria-label={`Kurangi ${item.name}`}>−</button>
                    <span class="min-w-10 text-center text-sm font-semibold tabular-nums">{item.quantity}</span>
                    <button class="min-h-11 min-w-11 px-3 text-lg font-semibold disabled:opacity-40" on:click={() => increase(item)} disabled={item.quantity >= item.stock} aria-label={`Tambah ${item.name}`}>+</button>
                  </div>
                  <p class="min-w-24 text-right font-semibold tabular-nums">Rp{(Number(item.price) * item.quantity).toLocaleString('id-ID')}</p>
                  <button class="rounded-xl border border-border px-3 py-2 text-sm font-semibold text-destructive" on:click={() => remove(item)}>Hapus</button>
                </div>
              </div>
            {/each}
          </div>
        </article>
      {/each}
    </section>

    <aside class="panel h-fit p-6 lg:sticky lg:top-6">
      <h2 class="font-semibold">Ringkasan</h2>
      <dl class="mt-4 space-y-3 text-sm">
        <div class="flex justify-between gap-4"><dt class="text-muted-foreground">Subtotal</dt><dd class="font-semibold tabular-nums">Rp{total(items).toLocaleString('id-ID')}</dd></div>
        <div class="flex justify-between gap-4"><dt class="text-muted-foreground">Pembayaran</dt><dd>Manual transfer</dd></div>
      </dl>
      <p class="mt-4 text-sm leading-6 text-muted-foreground">Upload bukti pembayaran dilakukan setelah order dibuat dan tetap melalui endpoint protected, bukan public static media.</p>
      <a class="mt-5 inline-flex min-h-11 w-full items-center justify-center rounded-xl bg-primary px-4 py-3 text-sm font-semibold text-primary-foreground" href="/store/checkout">Lanjut checkout</a>
    </aside>
  </div>
</StoreShell>
