<script lang="ts">
  import { apiUrl } from '$lib/api/base';
  import { onMount } from 'svelte';
  import { cart, type CartItem } from '$lib/stores/cart';
  import StoreShell from '$lib/ui/StoreShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';

  let items: CartItem[] = [];
  let customerName = '';
  let customerContact = '';
  let shippingAddress = '';
  let submitting = false;
  let error = '';
  let success = '';
  let trackingToken = '';
  let orderNumber = '';

  const unsubscribe = cart.subscribe((value) => {
    items = value;
  });

  const total = (entries: CartItem[]) =>
    entries.reduce((sum, item) => sum + Number(item.price ?? 0) * item.quantity, 0);

  async function submitCheckout() {
    submitting = true;
    error = '';
    success = '';
    trackingToken = '';
    orderNumber = '';
    try {
      const response = await fetch(apiUrl('/api/storefront/checkout'), {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({
          customer_name: customerName,
          customer_contact: customerContact,
          shipping_address: shippingAddress,
          items: items.map((item) => ({
            variant_id: item.variantId,
            quantity: item.quantity
          }))
        })
      });
      const payload = await response.json().catch(() => ({}));
      if (!response.ok) throw new Error(payload.message ?? 'Checkout membutuhkan item cart yang valid dari backend.');
      trackingToken = payload.order?.public_tracking_token ?? '';
      orderNumber = payload.order?.order_number ?? '';
      success = 'Order berhasil dibuat. Simpan nomor order dan lanjutkan ke pelacakan order untuk upload bukti pembayaran private.';
      // Clear backend cart, then reset local store only if backend clear succeeded.
      const clearRes = await fetch(apiUrl('/api/storefront/cart'), {
        method: 'DELETE',
        credentials: 'include'
      }).catch(() => null);
      if (clearRes?.ok) {
        cart.clear();
      } else {
        // Backend clear failed — re-hydrate so local state matches backend truth.
        await cart.hydrate();
      }
    } catch (checkoutError) {
      error = checkoutError instanceof Error ? checkoutError.message : 'Checkout gagal.';
    } finally {
      submitting = false;
    }
  }

  onMount(() => {
    cart.hydrate();
    return () => unsubscribe();
  });
</script>

<svelte:head><title>Checkout — Storefront</title></svelte:head>

<StoreShell active="checkout">
  <div class="mx-auto grid max-w-5xl gap-6 px-4 py-8 sm:px-6 lg:grid-cols-[1fr_340px] lg:px-8">
    <section class="panel p-6">
      <p class="eyebrow">Checkout foundation</p>
      <h1 class="mt-2 text-2xl font-bold">Manual transfer checkout</h1>
      <p class="mt-2 text-sm leading-6 text-muted-foreground">Form ini mengirim variant IDs dan quantity nyata dari cart browser ke backend checkout. Backend akan membuat satu order lintas brand dengan `order_brand_groups` dan tracking token publik.</p>

      <form class="mt-6 space-y-5" aria-label="Checkout form" on:submit|preventDefault={submitCheckout}>
        {#if error}<StateNotice tone="destructive" title="Checkout belum selesai" message={error} />{/if}
        {#if success}<StateNotice tone="success" title="Checkout berhasil" message={success} actionHref={trackingToken ? `/store/orders/${trackingToken}` : undefined} actionLabel={trackingToken ? 'Buka pelacakan order' : undefined} />{/if}
        {#if items.length === 0}
          <StateNotice tone="warning" title="Cart kosong" message="Tambahkan produk dari storefront sebelum checkout. Backend akan menolak empty checkout sebagai boundary aman." actionHref="/store" actionLabel="Kembali ke katalog" />
        {/if}
        <div class="panel bg-background p-5">
          <div class="section-heading">
            <div>
              <h2 class="text-lg font-semibold">Informasi penerima</h2>
              <p class="text-sm text-muted-foreground">Dipakai sebagai snapshot order saat checkout berhasil.</p>
            </div>
            <StateNotice tone="neutral" title="Step 1" message="Isi data customer dengan jelas." />
          </div>
          <div class="field-grid mt-4">
            <label class="block text-sm font-medium">
              Nama penerima
              <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={customerName} required autocomplete="name" />
            </label>
            <label class="block text-sm font-medium">
              Kontak
              <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={customerContact} required autocomplete="tel" />
            </label>
          </div>
        </div>

        <div class="panel bg-background p-5">
          <div class="section-heading">
            <div>
              <h2 class="text-lg font-semibold">Alamat pengiriman</h2>
              <p class="text-sm text-muted-foreground">Alamat ini ikut tersimpan sebagai snapshot order untuk fulfillment lintas brand.</p>
            </div>
            <StateNotice tone="neutral" title="Step 2" message="Pastikan alamat lengkap dan mudah diverifikasi seller." />
          </div>
          <label class="mt-4 block text-sm font-medium">
            Alamat pengiriman
            <textarea class="mt-2 min-h-28 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={shippingAddress} required></textarea>
          </label>
        </div>

        <button class="min-h-11 rounded-xl bg-primary px-5 py-3 text-sm font-semibold text-primary-foreground disabled:cursor-not-allowed disabled:opacity-50" disabled={submitting || items.length === 0}>
          {submitting ? 'Mengirim checkout…' : 'Kirim checkout'}
        </button>
      </form>
    </section>

    <aside class="space-y-4">
      <StateNotice tone="warning" title="Payment proof tetap private" message="Gate H tidak membuat preview bukti transfer sebagai static media. Upload proof harus memakai endpoint protected dan authorization backend." />
      <div class="panel p-5">
        <h2 class="font-semibold">Ringkasan order draft</h2>
        <dl class="mt-3 space-y-3 text-sm">
          <div class="flex justify-between gap-4"><dt class="text-muted-foreground">Item</dt><dd class="font-semibold tabular-nums">{items.length}</dd></div>
          <div class="flex justify-between gap-4"><dt class="text-muted-foreground">Subtotal</dt><dd class="font-semibold tabular-nums">Rp{total(items).toLocaleString('id-ID')}</dd></div>
        </dl>
        {#if orderNumber}
          <div class="mt-4 rounded-xl border border-success/30 bg-success/10 p-4 text-sm">
            <p class="font-semibold text-success">Order {orderNumber}</p>
            <p class="mt-1 text-muted-foreground">Tracking token publik sudah dibuat dan bisa dipakai customer untuk melihat status order tanpa membuka media private.</p>
          </div>
        {/if}
      </div>
    </aside>
  </div>
</StoreShell>
