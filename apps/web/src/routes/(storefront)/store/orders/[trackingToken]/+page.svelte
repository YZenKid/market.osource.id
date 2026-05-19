<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/state';
  import { apiUrl } from '$lib/api/base';
  import StoreShell from '$lib/ui/StoreShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  type OrderSummary = {
    order_number: string;
    payment_status: string;
    global_status: string;
    total_snapshot: string;
  };

  let loading = true;
  let error = '';
  let summary: OrderSummary | null = null;
  let uploading = false;
  let uploadError = '';
  let uploadSuccess = '';

  async function loadSummary() {
    loading = true;
    error = '';
    summary = null;
    try {
      const trackingToken = page.params.trackingToken;
      const response = await fetch(apiUrl(`/api/storefront/orders/${trackingToken}`), { credentials: 'include' });
      const payload = await response.json().catch(() => ({}));
      if (!response.ok) throw new Error(payload.message ?? 'Order tracking token tidak ditemukan.');
      summary = payload;
    } catch (loadError) {
      error = loadError instanceof Error ? loadError.message : 'Gagal memuat ringkasan order.';
    } finally {
      loading = false;
    }
  }

  async function uploadPaymentProof(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;

    uploading = true;
    uploadError = '';
    uploadSuccess = '';
    try {
      const formData = new FormData();
      formData.append('file', file);
      const trackingToken = page.params.trackingToken;
      const response = await fetch(apiUrl(`/api/storefront/orders/${trackingToken}/payment-proof`), {
        method: 'POST',
        body: formData,
        credentials: 'include'
      });
      const payload = await response.json().catch(() => ({}));
      if (!response.ok) throw new Error(payload.message ?? 'Upload bukti transfer gagal.');
      uploadSuccess = 'Bukti transfer berhasil diunggah dan status order menunggu verifikasi pembayaran.';
      await loadSummary();
      input.value = '';
    } catch (loadError) {
      uploadError = loadError instanceof Error ? loadError.message : 'Gagal upload bukti transfer.';
    } finally {
      uploading = false;
    }
  }

  onMount(loadSummary);
</script>

<svelte:head>
  <title>Order Tracking — market.osource.id</title>
</svelte:head>

<StoreShell active="checkout">
  <div class="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
    <section class="panel p-6">
      <p class="eyebrow">Order tracking</p>
      <h1 class="mt-2 text-2xl font-bold">Lacak status order</h1>
      <p class="mt-2 text-sm leading-6 text-muted-foreground">Halaman ini memakai tracking token publik untuk menampilkan ringkasan order tanpa membuka payment proof atau media private.</p>

      {#if loading}
        <div class="mt-6"><StateNotice tone="neutral" title="Memuat order" message="Mengambil ringkasan order dari endpoint storefront tracking." /></div>
      {:else if error}
        <div class="mt-6 flex flex-col items-center gap-4">
          <img
            src="/assets/empty-orders.png"
            alt="Ilustrasi order kosong: tidak ada data order yang ditemukan untuk token ini."
            class="w-48 rounded-xl object-contain aspect-[4/3]"
            loading="lazy"
            decoding="async"
          />
          <div class="w-full"><StateNotice tone="destructive" title="Order tidak dapat dimuat" message={error} actionHref="/store/checkout" actionLabel="Kembali ke checkout" /></div>
        </div>
      {:else if summary}
        {#if uploadError}
          <div class="mt-6"><StateNotice tone="destructive" title="Upload gagal" message={uploadError} /></div>
        {/if}
        {#if uploadSuccess}
          <div class="mt-6"><StateNotice tone="success" title="Bukti transfer diterima" message={uploadSuccess} /></div>
        {/if}
        <div class="mt-6 grid gap-4 md:grid-cols-2">
          <article class="metric-card">
            <p class="text-sm text-muted-foreground">Nomor order</p>
            <p class="mt-2 text-2xl font-bold tabular-nums">{summary.order_number}</p>
            <StatusBadge tone="primary" label="Tracking aktif" />
          </article>
          <article class="metric-card">
            <p class="text-sm text-muted-foreground">Total snapshot</p>
            <p class="mt-2 text-2xl font-bold tabular-nums">Rp{Number(summary.total_snapshot ?? 0).toLocaleString('id-ID')}</p>
            <StatusBadge tone="success" label="Snapshot tersimpan" />
          </article>
          <article class="metric-card">
            <p class="text-sm text-muted-foreground">Payment status</p>
            <p class="mt-2 text-xl font-bold">{summary.payment_status}</p>
            <StatusBadge tone={summary.payment_status === 'pending' ? 'warning' : 'success'} label="Manual transfer" />
          </article>
          <article class="metric-card">
            <p class="text-sm text-muted-foreground">Global status</p>
            <p class="mt-2 text-xl font-bold">{summary.global_status}</p>
            <StatusBadge tone="neutral" label="Order lifecycle" />
          </article>
        </div>

        <div class="mt-6 panel p-5">
          <div class="flex items-start justify-between gap-3">
            <div>
              <h2 class="text-lg font-semibold">Upload bukti transfer</h2>
              <p class="mt-2 text-sm leading-6 text-muted-foreground">File akan dikirim ke endpoint protected storefront dan tidak pernah dipublikasikan sebagai static media.</p>
            </div>
            <StatusBadge tone="warning" label="Private media" />
          </div>

          <label class="mt-4 block text-sm font-medium">
            File bukti transfer
            <input class="mt-2 block w-full rounded-xl border border-border bg-surface px-3 py-3 text-sm" type="file" accept="image/*,.pdf" disabled={uploading} on:change={uploadPaymentProof} />
          </label>
          <p class="mt-2 text-xs text-muted-foreground">Setelah upload berhasil, payment status akan berpindah ke waiting_payment_verification hingga diverifikasi admin/seller berizin.</p>
        </div>
      {/if}

      <div class="mt-6">
        <StateNotice tone="warning" title="Upload bukti transfer berikutnya tetap private" message="Tracking page hanya menampilkan status order. Upload bukti transfer dilakukan lewat endpoint protected dan tidak pernah menjadi static public media." />
      </div>
    </section>
  </div>
</StoreShell>
