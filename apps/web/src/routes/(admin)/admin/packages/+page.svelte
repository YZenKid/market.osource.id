<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl } from '$lib/api/base';
  import AdminShell from '$lib/ui/AdminShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  type PackageStatus = {
    package_id: string;
    name: string;
    capabilities: string[];
    paid?: boolean;
    enabled?: boolean;
    status?: string;
  };

  let packages: PackageStatus[] = [];
  let loading = true;
  let permission = false;
  let degraded = false;
  let error = '';

  async function loadPackages() {
    loading = true;
    permission = false;
    degraded = false;
    error = '';
    try {
      let response = await fetch(apiUrl('/api/admin/packages'), { credentials: 'include' });
      if (response.status === 401 || response.status === 403) {
        permission = true;
        response = await fetch(apiUrl('/api/packages'), { credentials: 'include' });
      }
      if (!response.ok) throw new Error('Package API belum tersedia.');
      const payload = await response.json();
      packages = Array.isArray(payload.packages) ? payload.packages : [];
      degraded = Boolean(payload.degraded || permission);
    } catch (loadError) {
      error = loadError instanceof Error ? loadError.message : 'Gagal memuat package.';
    } finally {
      loading = false;
    }
  }

  onMount(loadPackages);
</script>

<svelte:head><title>Packages — Admin</title></svelte:head>

<AdminShell active="packages" title="Packages" description="Open-core package status foundation. Base marketplace tetap berjalan tanpa paid entitlement; paid features harus feature-gated server-side.">
  {#if loading}
    <StateNotice tone="neutral" title="Memuat package registry" message="Mengecek admin package status dan fallback public registry bila sesi admin belum tersedia." />
  {:else if error}
    <StateNotice tone="destructive" title="Package belum dapat dimuat" message={error} />
  {:else if permission}
    <StateNotice tone="warning" title="Mode public registry" message="Sesi Super Admin tidak tersedia. Daftar ini memakai endpoint public package registry dan tidak mengklaim status entitlement aktual." />
  {/if}

  {#if !loading && packages.length === 0 && !error}
    <StateNotice tone="primary" title="Registry kosong" message="Static package registry belum mengembalikan item pada runtime ini." />
  {:else if !loading && packages.length > 0}
    <section class="grid gap-4 md:grid-cols-2" aria-label="Package cards">
      {#each packages as pkg}
        <article class="panel p-5">
          <div class="flex items-start justify-between gap-4">
            <div>
              <h2 class="text-lg font-semibold">{pkg.name}</h2>
              <p class="mt-1 font-mono text-xs text-muted-foreground">{pkg.package_id}</p>
            </div>
            <StatusBadge tone={pkg.enabled ? 'success' : pkg.paid ? 'secondary' : 'primary'} label={pkg.enabled ? 'Enabled' : pkg.paid ? 'Paid package' : 'Base'} />
          </div>
          <div class="mt-4 flex flex-wrap gap-2" aria-label="Capabilities">
            {#each pkg.capabilities ?? [] as capability}
              <span class="rounded-lg border border-border bg-muted px-2.5 py-1 font-mono text-xs text-muted-foreground">{capability}</span>
            {/each}
          </div>
        </article>
      {/each}
    </section>
    {#if degraded}
      <p class="text-sm text-muted-foreground">Ditandai degraded/fallback karena admin status penuh memerlukan database dan sesi Super Admin.</p>
    {/if}
  {/if}
</AdminShell>
