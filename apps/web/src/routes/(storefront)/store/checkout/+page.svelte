<script lang="ts">
  import { apiUrl } from '$lib/api/base';
  import StoreShell from '$lib/ui/StoreShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';

  let customerName = '';
  let customerContact = '';
  let shippingAddress = '';
  let submitting = false;
  let error = '';
  let success = '';

  async function submitCheckout() {
    submitting = true;
    error = '';
    success = '';
    try {
      const response = await fetch(apiUrl('/api/storefront/checkout'), {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({
          customer_name: customerName,
          customer_contact: customerContact,
          shipping_address: shippingAddress,
          items: []
        })
      });
      const payload = await response.json().catch(() => ({}));
      if (!response.ok) throw new Error(payload.message ?? 'Checkout membutuhkan item cart yang valid dari backend.');
      success = 'Order berhasil dibuat. Instruksi upload bukti pembayaran akan ditampilkan melalui flow protected.';
    } catch (checkoutError) {
      error = checkoutError instanceof Error ? checkoutError.message : 'Checkout gagal.';
    } finally {
      submitting = false;
    }
  }
</script>

<svelte:head><title>Checkout — Storefront</title></svelte:head>

<StoreShell active="checkout">
  <div class="mx-auto grid max-w-5xl gap-6 px-4 py-8 sm:px-6 lg:grid-cols-[1fr_340px] lg:px-8">
    <section class="panel p-6">
      <p class="text-sm font-semibold uppercase tracking-[0.2em] text-primary">Checkout foundation</p>
      <h1 class="mt-2 text-2xl font-bold">Manual transfer checkout</h1>
      <p class="mt-2 text-sm leading-6 text-muted-foreground">Form ini fetch-ready terhadap /api/storefront/checkout, tetapi belum mengklaim persistent cart. Error empty checkout ditampilkan sebagai state aman.</p>

      <form class="mt-6 space-y-5" aria-label="Checkout form" on:submit|preventDefault={submitCheckout}>
        {#if error}<StateNotice tone="destructive" title="Checkout belum selesai" message={error} />{/if}
        {#if success}<StateNotice tone="success" title="Checkout berhasil" message={success} />{/if}
        <label class="block text-sm font-medium">
          Nama penerima
          <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={customerName} required autocomplete="name" />
        </label>
        <label class="block text-sm font-medium">
          Kontak
          <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={customerContact} required autocomplete="tel" />
        </label>
        <label class="block text-sm font-medium">
          Alamat pengiriman
          <textarea class="mt-2 min-h-28 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={shippingAddress} required></textarea>
        </label>
        <button class="min-h-11 rounded-xl bg-primary px-5 py-3 text-sm font-semibold text-primary-foreground disabled:cursor-not-allowed disabled:opacity-50" disabled={submitting}>
          {submitting ? 'Mengirim checkout…' : 'Kirim checkout'}
        </button>
      </form>
    </section>

    <aside class="space-y-4">
      <StateNotice tone="warning" title="Payment proof tetap private" message="Gate H tidak membuat preview bukti transfer sebagai static media. Upload proof harus memakai endpoint protected dan authorization backend." />
      <div class="panel p-5">
        <h2 class="font-semibold">State yang dicakup</h2>
        <ul class="mt-3 list-disc space-y-2 pl-5 text-sm leading-6 text-muted-foreground">
          <li>Loading/submitting button state</li>
          <li>Error backend atau empty checkout</li>
          <li>Success message tanpa mengekspos proof file</li>
        </ul>
      </div>
    </aside>
  </div>
</StoreShell>
