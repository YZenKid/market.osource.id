<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl } from '$lib/api/base';
  import AdminShell from '$lib/ui/AdminShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  type Brand = { id: string; name: string; slug: string; status?: string | null; description?: string | null };
  let brands: Brand[] = [];
  let loading = true;
  let permission = false;
  let error = '';

  async function loadBrands() {
    loading = true;
    permission = false;
    error = '';
    try {
      const response = await fetch(apiUrl('/api/admin/brands'), { credentials: 'include' });
      if (response.status === 401 || response.status === 403) {
        permission = true;
        return;
      }
      if (!response.ok) throw new Error('Brand API belum tersedia.');
      const payload = await response.json();
      brands = Array.isArray(payload.brands) ? payload.brands : [];
    } catch (loadError) {
      error = loadError instanceof Error ? loadError.message : 'Gagal memuat brand.';
    } finally {
      loading = false;
    }
  }

  onMount(loadBrands);
</script>

<svelte:head><title>Brands — Admin</title></svelte:head>

<AdminShell active="brands" title="Brands" description="Foundation untuk brand marketplace. Seller tidak mendaftar sendiri; Super Admin mengatur brand dan assignment.">
  <span slot="header-action" class="inline-flex min-h-11 items-center rounded-xl border border-border bg-muted px-4 py-2.5 text-sm font-semibold text-muted-foreground">Create form pending</span>

  {#if loading}
    <StateNotice tone="neutral" title="Memuat brand" message="Mengambil daftar brand dari endpoint admin terproteksi." />
  {:else if permission}
    <StateNotice tone="warning" title="Butuh sesi Super Admin" message="Brand admin hanya boleh diakses oleh Super Admin. Backend tetap menjadi sumber otorisasi." actionHref="/install" actionLabel="Cek install/login" />
  {:else if error}
    <StateNotice tone="destructive" title="Brand belum dapat dimuat" message={error} />
  {:else if brands.length === 0}
    <StateNotice tone="primary" title="Belum ada brand" message="Buat brand pertama melalui API/admin form pada slice berikutnya. Brand akan menjadi scope produk dan seller assignment." />
  {:else}
    <section class="grid gap-4 md:grid-cols-2" aria-label="Brand list">
      {#each brands as brand}
        <article class="panel p-5">
          <div class="flex items-start justify-between gap-4">
            <div>
              <h2 class="text-lg font-semibold">{brand.name}</h2>
              <p class="mt-1 font-mono text-xs text-muted-foreground">/{brand.slug}</p>
            </div>
            <StatusBadge tone={brand.status === 'active' ? 'success' : 'neutral'} label={brand.status ?? 'status unknown'} />
          </div>
          <p class="mt-4 text-sm leading-6 text-muted-foreground">{brand.description ?? 'Belum ada deskripsi brand.'}</p>
        </article>
      {/each}
    </section>
  {/if}
</AdminShell>
