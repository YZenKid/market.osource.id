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

  function statusSteps(paymentStatus: string, globalStatus: string) {
    return [
      { label: 'Order dibuat', active: true },
      { label: 'Menunggu pembayaran', active: paymentStatus === 'pending' || paymentStatus === 'waiting_payment_verification' || paymentStatus === 'verified' },
      { label: 'Verifikasi pembayaran', active: paymentStatus === 'waiting_payment_verification' || paymentStatus === 'verified' },
      { label: 'Diproses brand', active: globalStatus === 'processing' || globalStatus === 'completed' },
      { label: 'Selesai', active: globalStatus === 'completed' }
    ];
  }

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
      uploadSuccess = `Bukti transfer ${file.name} berhasil diunggah dan status order menunggu verifikasi pembayaran.`;
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

<StoreShell active="store">
  <div class="mx-auto max-w-6xl px-4 py-8 sm:px-6 lg:px-8">
    <section class="grid gap-6 lg:grid-cols-[1fr_360px]">
      <div class="space-y-6">
        <div class="panel p-6">
          <p class="eyebrow">Order tracking</p>
          <h1 class="mt-2 text-2xl font-bold tracking-tight sm:text-3xl">Lacak status order</h1>
          <p class="mt-2 text-sm leading-6 text-muted-foreground">Tracking token publik hanya menampilkan ringkasan order. Bukti transfer tetap private dan diakses lewat endpoint protected.</p>
        </div>

        {#if loading}
          <StateNotice tone="neutral" title="Memuat order" message="Mengambil ringkasan order dari endpoint storefront tracking." />
        {:else if error}
          <div class="flex flex-col items-center gap-4">
            <img src="/assets/empty-orders.png" alt="Ilustrasi order kosong" class="aspect-[4/3] w-56 rounded-xl object-contain" loading="lazy" />
            <div class="w-full">
              <StateNotice tone="destructive" title="Order tidak dapat dimuat" message={error} actionHref="/store/checkout" actionLabel="Kembali ke checkout" />
            </div>
          </div>
        {:else if summary}
          {#if uploadError}
            <StateNotice tone="destructive" title="Upload gagal" message={uploadError} />
          {/if}
          {#if uploadSuccess}
            <StateNotice tone="success" title="Bukti transfer diterima" message={uploadSuccess} />
          {/if}

          <div class="grid gap-4 sm:grid-cols-2">
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
              <StatusBadge tone={summary.payment_status === 'pending' ? 'warning' : summary.payment_status === 'verified' ? 'success' : 'primary'} label="Manual transfer" />
            </article>
            <article class="metric-card">
              <p class="text-sm text-muted-foreground">Global status</p>
              <p class="mt-2 text-xl font-bold">{summary.global_status}</p>
              <StatusBadge tone="neutral" label="Order lifecycle" />
            </article>
          </div>

          <div class="panel p-5">
            <h2 class="text-lg font-semibold">Alur status order</h2>
            <div class="mt-4 space-y-3">
              {#each statusSteps(summary.payment_status, summary.global_status) as step}
                <div class="flex items-center gap-3 rounded-xl border border-border bg-background px-4 py-3">
                  <span aria-hidden="true" class={`inline-flex h-6 w-6 items-center justify-center rounded-full text-xs font-bold ${step.active ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}`}>•</span>
                  <span class={step.active ? 'font-semibold' : 'text-muted-foreground'}>{step.label}</span>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <aside class="space-y-4">
        <div class="panel p-5 lg:sticky lg:top-24">
          <div class="flex items-start justify-between gap-3">
            <div>
              <h2 class="text-lg font-semibold">Upload bukti transfer</h2>
              <p class="mt-2 text-sm leading-6 text-muted-foreground">Hanya metadata file yang ditampilkan di UI ini. Tidak ada preview gambar public.</p>
            </div>
            <StatusBadge tone="warning" label="Private media" />
          </div>

          <label class="mt-4 block text-sm font-medium">
            File bukti transfer
            <input class="mt-2 block min-h-11 w-full rounded-xl border border-border bg-surface px-4 py-3 text-sm" type="file" accept="image/*,.pdf" disabled={uploading} on:change={uploadPaymentProof} />
          </label>
          <p class="mt-2 text-xs text-muted-foreground">Format disarankan JPG, PNG, atau PDF. Setelah upload, admin/seller berizin akan memverifikasi file di area order admin.</p>
        </div>

        <StateNotice tone="warning" title="No public preview" message="Tracking page tidak pernah menampilkan raw image atau URL static untuk bukti transfer. Status order tetap dapat dilihat tanpa membuka media private." />
      </aside>
    </section>
  </div>
</StoreShell>
