<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl } from '$lib/api/base';
  import AdminShell from '$lib/ui/AdminShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  let runtime: Record<string, any> | null = null;
  let loading = true;
  let permission = false;
  let error = '';

  async function loadRuntime() {
    loading = true;
    permission = false;
    error = '';
    try {
      const response = await fetch(apiUrl('/api/system/runtime'), { credentials: 'include' });
      if (response.status === 401 || response.status === 403) {
        permission = true;
        return;
      }
      if (!response.ok) throw new Error('System runtime API belum tersedia.');
      runtime = await response.json();
    } catch (loadError) {
      error = loadError instanceof Error ? loadError.message : 'Gagal memuat system runtime.';
    } finally {
      loading = false;
    }
  }

  onMount(loadRuntime);
</script>

<svelte:head><title>System — Admin</title></svelte:head>

<AdminShell active="system" title="System" description="Status-forward runtime foundation: database, storage local, install lock, packages, dan tunnel opt-in tanpa menampilkan secret atau path sensitif.">
  {#if loading}
    <StateNotice tone="neutral" title="Memuat runtime" message="Mengambil /api/system/runtime yang dijaga sesi Super Admin." />
  {:else if permission}
    <StateNotice tone="warning" title="System status terproteksi" message="Runtime detail memerlukan sesi Super Admin agar secret, konfigurasi, dan status operasional tidak bocor." actionHref="/install" actionLabel="Buka install panel" />
  {:else if error}
    <StateNotice tone="destructive" title="Runtime belum dapat dimuat" message={error} />
  {:else if runtime}
    <section class="grid gap-4 md:grid-cols-2 xl:grid-cols-3" aria-label="System status cards">
      <article class="panel p-5">
        <p class="text-sm text-muted-foreground">Backend</p>
        <p class="mt-2 text-2xl font-bold">{runtime.backend?.ok ? 'Online' : 'Unknown'}</p>
        <StatusBadge tone={runtime.backend?.ok ? 'success' : 'warning'} label={runtime.backend?.ok ? 'Healthy' : 'Check needed'} />
      </article>
      <article class="panel p-5">
        <p class="text-sm text-muted-foreground">Database</p>
        <p class="mt-2 text-2xl font-bold">{runtime.database?.ok ? 'Connected' : 'Unavailable'}</p>
        <StatusBadge tone={runtime.database?.ok ? 'success' : 'destructive'} label="PostgreSQL" />
      </article>
      <article class="panel p-5">
        <p class="text-sm text-muted-foreground">Storage</p>
        <p class="mt-2 text-2xl font-bold">{runtime.storage?.provider ?? 'local'}</p>
        <StatusBadge tone={runtime.storage?.configured ? 'success' : 'warning'} label={runtime.storage?.configured ? 'Configured' : 'Needs setup'} />
      </article>
      <article class="panel p-5">
        <p class="text-sm text-muted-foreground">Install lock</p>
        <p class="mt-2 text-2xl font-bold">{runtime.install_state?.locked ? 'Locked' : 'Open'}</p>
        <StatusBadge tone={runtime.install_state?.locked ? 'success' : 'warning'} label={runtime.install_state?.state ?? 'unknown'} />
      </article>
      <article class="panel p-5">
        <p class="text-sm text-muted-foreground">Tunnel</p>
        <p class="mt-2 text-2xl font-bold">{runtime.tunnel?.state ?? 'Off'}</p>
        <StatusBadge tone="neutral" label="Opt-in" />
      </article>
      <article class="panel p-5">
        <p class="text-sm text-muted-foreground">Packages</p>
        <p class="mt-2 text-2xl font-bold tabular-nums">{runtime.packages?.registered ?? '—'}</p>
        <StatusBadge tone="primary" label="Registered" />
      </article>
    </section>
    <StateNotice tone="warning" title="Operational caution" message="Tunnel start/stop dan secret-bearing configuration tidak ditambahkan di Gate H. Tindakan runtime harus tetap opt-in dan server-side guarded." />
  {/if}
</AdminShell>
