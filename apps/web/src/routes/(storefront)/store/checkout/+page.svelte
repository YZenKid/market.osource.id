<script lang="ts">
  import { apiUrl } from '$lib/api/base';
  import { onMount } from 'svelte';
  import { cart, type CartItem } from '$lib/stores/cart';
  import StoreShell from '$lib/ui/StoreShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import FormSection from '$lib/ui/forms/FormSection.svelte';
  import InputField from '$lib/ui/forms/InputField.svelte';
  import SubmitButton from '$lib/ui/forms/SubmitButton.svelte';

  let items: CartItem[] = [];
  let customerName = '';
  let customerContact = '';
  let shippingAddress = '';
  let note = '';
  let submitting = false;
  let error = '';
  let success = '';
  let trackingToken = '';
  let orderNumber = '';

  const unsubscribe = cart.subscribe((value) => {
    items = value;
  });

  const total = (entries: CartItem[]) => entries.reduce((sum, item) => sum + Number(item.price ?? 0) * item.quantity, 0);

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
          shipping_address: note ? `${shippingAddress}\n\nCatatan:\n${note}` : shippingAddress,
          items: items.map((item) => ({ variant_id: item.variantId, quantity: item.quantity }))
        })
      });
      const payload = await response.json().catch(() => ({}));
      if (!response.ok) throw new Error(payload.message ?? 'Checkout membutuhkan item cart yang valid dari backend.');
      trackingToken = payload.order?.public_tracking_token ?? '';
      orderNumber = payload.order?.order_number ?? '';
      success = 'Order berhasil dibuat. Simpan nomor order dan lanjutkan ke pelacakan order untuk upload bukti pembayaran private.';

      const clearRes = await fetch(apiUrl('/api/storefront/cart'), {
        method: 'DELETE',
        credentials: 'include'
      }).catch(() => null);
      if (clearRes?.ok) {
        cart.clear();
      } else {
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
  <div class="mx-auto grid max-w-6xl gap-6 px-4 py-8 sm:px-6 lg:grid-cols-[1fr_400px] lg:px-8">
    <section class="space-y-6">
      <div class="panel p-6">
        <p class="eyebrow">Checkout</p>
        <h1 class="mt-2 text-2xl font-bold tracking-tight sm:text-3xl">Selesaikan order manual transfer</h1>
        <p class="mt-2 text-sm leading-6 text-muted-foreground">Isi data pengiriman dan backend akan membuat satu order dengan tracking token publik. Upload bukti transfer dilakukan di halaman tracking, bukan di checkout.</p>
      </div>

      {#if error}<StateNotice tone="destructive" title="Checkout belum selesai" message={error} />{/if}
      {#if success}<StateNotice tone="success" title="Checkout berhasil" message={success} actionHref={trackingToken ? `/store/orders/${trackingToken}` : undefined} actionLabel={trackingToken ? 'Buka pelacakan order' : undefined} />{/if}
      {#if items.length === 0}
        <StateNotice tone="warning" title="Cart kosong" message="Tambahkan produk dari storefront sebelum checkout." actionHref="/store" actionLabel="Kembali ke katalog" />
      {/if}

      <form class="space-y-6" aria-label="Checkout form" on:submit|preventDefault={submitCheckout}>
        <FormSection title="Informasi penerima" description="Dipakai sebagai snapshot order saat checkout berhasil." eyebrow="Step 1">
          <div class="field-grid">
            <InputField id="customer-name" label="Nama penerima" bind:value={customerName} required autocomplete="name" />
            <InputField id="customer-contact" label="Kontak" bind:value={customerContact} required autocomplete="tel" helper="Email atau nomor telepon yang aktif." />
          </div>
        </FormSection>

        <FormSection title="Alamat pengiriman" description="Alamat ini tersimpan sebagai snapshot order untuk fulfillment lintas brand." eyebrow="Step 2">
          <InputField id="shipping-address" label="Alamat pengiriman" bind:value={shippingAddress} required as="textarea" rows={5} />
        </FormSection>

        <FormSection title="Catatan tambahan" description="Opsional. Misal: preferensi pengiriman, catatan warna, atau jam penerimaan." eyebrow="Step 3">
          <InputField id="checkout-note" label="Catatan" bind:value={note} as="textarea" rows={4} />
        </FormSection>

        <FormSection title="Pembayaran" description="Setelah order dibuat, sistem akan menampilkan tracking token dan instruksi upload bukti transfer private." eyebrow="Step 4">
          <div class="rounded-2xl border border-warning/30 bg-warning/10 p-4 text-sm leading-6 text-warning">
            <p class="font-semibold">Manual transfer only</p>
            <p class="mt-1">Customer menyelesaikan transfer di luar sistem, lalu upload bukti pembayaran pada halaman tracking order. Tidak ada public preview untuk bukti transfer.</p>
          </div>
        </FormSection>

        <SubmitButton type="submit" tone="primary" loading={submitting} disabled={submitting || items.length === 0}>
          {submitting ? 'Mengirim checkout…' : 'Buat order sekarang'}
        </SubmitButton>
      </form>
    </section>

    <aside class="space-y-4">
      <div class="panel p-6 lg:sticky lg:top-24">
        <h2 class="font-semibold">Ringkasan order</h2>
        <dl class="mt-4 space-y-3 text-sm">
          <div class="flex justify-between gap-4"><dt class="text-muted-foreground">Item</dt><dd class="font-semibold tabular-nums">{items.length}</dd></div>
          <div class="flex justify-between gap-4"><dt class="text-muted-foreground">Subtotal</dt><dd class="font-semibold tabular-nums">Rp{total(items).toLocaleString('id-ID')}</dd></div>
          <div class="flex justify-between gap-4"><dt class="text-muted-foreground">Payment method</dt><dd>Manual transfer</dd></div>
        </dl>
        {#if orderNumber}
          <div class="mt-4 rounded-xl border border-success/30 bg-success/10 p-4 text-sm">
            <p class="font-semibold text-success">Order {orderNumber}</p>
            <p class="mt-1 text-muted-foreground">Tracking token publik sudah dibuat. Lanjutkan ke halaman tracking untuk upload bukti pembayaran private.</p>
          </div>
        {/if}
        <div class="mt-4 space-y-2 text-sm text-muted-foreground">
          {#each items as item}
            <div class="flex items-center justify-between gap-3 rounded-xl border border-border bg-background px-3 py-2">
              <span class="line-clamp-1">{item.name} × {item.quantity}</span>
              <span class="font-semibold tabular-nums">Rp{(Number(item.price) * item.quantity).toLocaleString('id-ID')}</span>
            </div>
          {/each}
        </div>
      </div>
    </aside>
  </div>
</StoreShell>
