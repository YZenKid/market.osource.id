<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl, getCsrfToken } from '$lib/api/base';
  import AdminShell from '$lib/ui/AdminShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  let roleCode = '';
  let loading = true;
  let seeding = false;
  let clearing = false;
  let error = '';
  let success = '';

  async function loadRole() {
    loading = true;
    try {
      const res = await fetch(apiUrl('/api/auth/me'), { credentials: 'include' });
      if (res.ok) {
        const me = await res.json().catch(() => ({}));
        roleCode = me?.user?.role?.code ?? '';
      }
    } finally {
      loading = false;
    }
  }

  async function seedDemo() {
    seeding = true;
    error = '';
    success = '';
    try {
      const csrfToken = await getCsrfToken();
      const res = await fetch(apiUrl('/api/admin/demo/seed'), {
        method: 'POST',
        credentials: 'include',
        headers: { 'x-csrf-token': csrfToken }
      });
      const payload = await res.json().catch(() => ({}));
      if (!res.ok) throw new Error(payload.message ?? 'Seed demo gagal. Endpoint mungkin belum tersedia.');
      success = 'Data demo berhasil di-seed. Brand, produk, dan seller demo sudah tersedia di storefront.';
    } catch (err) {
      error = err instanceof Error ? err.message : 'Seed gagal.';
    } finally {
      seeding = false;
    }
  }

  async function clearDemo() {
    clearing = true;
    error = '';
    success = '';
    try {
      const csrfToken = await getCsrfToken();
      const res = await fetch(apiUrl('/api/admin/demo/clear'), {
        method: 'DELETE',
        credentials: 'include',
        headers: { 'x-csrf-token': csrfToken }
      });
      const payload = await res.json().catch(() => ({}));
      if (!res.ok) throw new Error(payload.message ?? 'Clear demo gagal. Endpoint mungkin belum tersedia.');
      success = 'Data demo berhasil dihapus. Storefront kembali ke state kosong.';
    } catch (err) {
      error = err instanceof Error ? err.message : 'Clear gagal.';
    } finally {
      clearing = false;
    }
  }

  $: isSuperAdmin = roleCode === 'super_admin';

  onMount(loadRole);
</script>

<svelte:head>
  <title>Demo Data — Admin Settings</title>
</svelte:head>

<AdminShell
  active="settings"
  title="Demo Data"
  description="Seed atau hapus data demo clothing company untuk keperluan testing dan demonstrasi."
  eyebrow="Settings → Demo Data"
>
  <div slot="header-action">
    <a class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-4 py-2.5 text-sm font-semibold" href="/admin/settings">
      ← Kembali ke Settings
    </a>
  </div>

  {#if loading}
    <StateNotice tone="neutral" title="Memuat" message="Memverifikasi sesi operator." />
  {:else if !isSuperAdmin}
    <StateNotice
      tone="warning"
      title="Akses terbatas"
      message="Demo data management hanya untuk Super Admin. Role aktif: {roleCode || 'tidak terautentikasi'}."
      actionHref="/admin/login"
      actionLabel="Login sebagai Super Admin"
    />
  {:else}
    {#if error}
      <StateNotice tone="destructive" title="Operasi gagal" message={error} />
    {/if}
    {#if success}
      <StateNotice tone="success" title="Berhasil" message={success} actionHref="/store" actionLabel="Lihat storefront" />
    {/if}

    <section class="panel p-6 space-y-5">
      <div>
        <h2 class="text-lg font-semibold">Clothing demo brands</h2>
        <p class="mt-1 text-sm text-muted-foreground">Tiga brand demo dengan produk, variant, dan seller masing-masing. Data ditandai <code>is_demo = true</code> agar dapat dihapus bersih.</p>
      </div>

      <div class="grid gap-3 sm:grid-cols-3">
        {#each [
          { slug: 'batik-nusantara', name: 'Batik Nusantara', desc: 'Produk batik premium lokal.' },
          { slug: 'urban-threads', name: 'Urban Threads', desc: 'Pakaian casual urban modern.' },
          { slug: 'modest-wear-id', name: 'Modest Wear ID', desc: 'Busana modest kontemporer.' }
        ] as brand}
          <div class="rounded-2xl border border-border bg-background p-4">
            <p class="font-semibold">{brand.name}</p>
            <p class="mt-1 font-mono text-xs text-muted-foreground">{brand.slug}</p>
            <p class="mt-2 text-sm text-muted-foreground">{brand.desc}</p>
          </div>
        {/each}
      </div>

      <div class="flex flex-wrap gap-3 pt-2">
        <button
          class="inline-flex min-h-11 items-center rounded-xl bg-primary px-5 py-3 text-sm font-semibold text-primary-foreground disabled:cursor-not-allowed disabled:opacity-50"
          on:click={seedDemo}
          disabled={seeding || clearing}
          aria-busy={seeding}
        >
          {seeding ? 'Menyeed data…' : 'Seed demo data'}
        </button>
        <button
          class="inline-flex min-h-11 items-center rounded-xl border border-destructive/30 bg-destructive/10 px-5 py-3 text-sm font-semibold text-destructive disabled:cursor-not-allowed disabled:opacity-50"
          on:click={clearDemo}
          disabled={seeding || clearing}
          aria-busy={clearing}
        >
          {clearing ? 'Menghapus data…' : 'Hapus demo data'}
        </button>
      </div>

      <StateNotice
        tone="warning"
        title="Idempotent"
        message="Seed aman dijalankan berulang kali — data yang sudah ada tidak akan diduplikasi. Clear hanya menghapus data bertanda is_demo."
      />
    </section>
  {/if}
</AdminShell>
