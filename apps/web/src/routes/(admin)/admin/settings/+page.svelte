<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl } from '$lib/api/base';
  import AdminShell from '$lib/ui/AdminShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  type Tab = 'marketplace' | 'storage' | 'tunnel' | 'package' | 'demo' | 'reset';

  let activeTab: Tab = 'marketplace';
  let roleCode = '';
  let loading = true;
  let runtime: Record<string, any> | null = null;
  let error = '';

  const tabs: { key: Tab; label: string; superOnly?: boolean }[] = [
    { key: 'marketplace', label: 'Marketplace' },
    { key: 'storage', label: 'Storage', superOnly: true },
    { key: 'tunnel', label: 'Tunnel', superOnly: true },
    { key: 'package', label: 'Package', superOnly: true },
    { key: 'demo', label: 'Demo Data', superOnly: true },
    { key: 'reset', label: 'Install Reset', superOnly: true }
  ];

  async function loadSettings() {
    loading = true;
    error = '';
    try {
      const meRes = await fetch(apiUrl('/api/auth/me'), { credentials: 'include' });
      if (meRes.ok) {
        const me = await meRes.json().catch(() => ({}));
        roleCode = me?.user?.role?.code ?? '';
      }
      const runtimeRes = await fetch(apiUrl('/api/system/runtime'), { credentials: 'include' });
      if (runtimeRes.ok) {
        runtime = await runtimeRes.json().catch(() => null);
      }
    } catch (err) {
      error = err instanceof Error ? err.message : 'Gagal memuat settings.';
    } finally {
      loading = false;
    }
  }

  $: isSuperAdmin = roleCode === 'super_admin';
  $: visibleTabs = tabs.filter(t => !t.superOnly || isSuperAdmin);

  onMount(loadSettings);
</script>

<svelte:head>
  <title>Settings — Admin</title>
</svelte:head>

<AdminShell
  active="settings"
  title="Settings"
  description="Konfigurasi marketplace, storage, tunnel, package, demo data, dan install reset."
>
  {#if loading}
    <StateNotice tone="neutral" title="Memuat settings" message="Mengambil status runtime dan sesi operator." />
  {:else if error}
    <StateNotice tone="destructive" title="Settings tidak dapat dimuat" message={error} />
  {:else if !isSuperAdmin && roleCode !== 'admin' && roleCode !== ''}
    <StateNotice
      tone="warning"
      title="Akses terbatas"
      message="Role {roleCode} hanya dapat melihat tab Marketplace. Tab sistem dan install reset memerlukan Super Admin."
    />
  {/if}

  <!-- Tab bar -->
  <div class="flex flex-wrap gap-2 border-b border-border pb-4" role="tablist" aria-label="Settings navigation">
    {#each visibleTabs as tab}
      <button
        class={`inline-flex min-h-11 items-center rounded-xl border px-4 py-2 text-sm font-semibold transition-colors ${
          activeTab === tab.key
            ? 'border-primary/30 bg-primary/10 text-primary'
            : 'border-border bg-surface text-muted-foreground hover:bg-muted'
        }`}
        on:click={() => (activeTab = tab.key)}
        role="tab"
        aria-selected={activeTab === tab.key}
        aria-controls={`settings-panel-${tab.key}`}
        id={`settings-tab-${tab.key}`}
      >
        {tab.label}
      </button>
    {/each}
  </div>

  <!-- Tab content -->
  {#if activeTab === 'marketplace'}
    <div class="panel space-y-4 p-6" role="tabpanel" id="settings-panel-marketplace" aria-labelledby="settings-tab-marketplace">
      <div>
        <h2 class="text-lg font-semibold">Marketplace</h2>
        <p class="mt-1 text-sm text-muted-foreground">Nama dan konfigurasi dasar marketplace. Perubahan memerlukan restart backend.</p>
      </div>
      {#if runtime?.config}
        <dl class="space-y-3 text-sm">
          <div class="flex items-center justify-between gap-4 rounded-xl border border-border bg-background p-4">
            <dt class="text-muted-foreground">Base URL</dt>
            <dd class="font-mono font-semibold">{runtime.config.base_url ?? '—'}</dd>
          </div>
          <div class="flex items-center justify-between gap-4 rounded-xl border border-border bg-background p-4">
            <dt class="text-muted-foreground">Runtime mode</dt>
            <dd><StatusBadge tone="primary" label={runtime.config.runtime_mode ?? '—'} /></dd>
          </div>
        </dl>
      {:else}
        <StateNotice tone="neutral" title="Data marketplace" message="Konfigurasi marketplace diambil dari runtime backend. Login sebagai Super Admin untuk melihat detail." />
      {/if}
    </div>

  {:else if activeTab === 'storage'}
    <div class="panel space-y-4 p-6" role="tabpanel" id="settings-panel-storage" aria-labelledby="settings-tab-storage">
      <div>
        <h2 class="text-lg font-semibold">Storage</h2>
        <p class="mt-1 text-sm text-muted-foreground">Konfigurasi storage lokal untuk media private (payment proof, dll).</p>
      </div>
      {#if runtime?.storage}
        <dl class="space-y-3 text-sm">
          <div class="flex items-center justify-between gap-4 rounded-xl border border-border bg-background p-4">
            <dt class="text-muted-foreground">Provider</dt>
            <dd><StatusBadge tone="primary" label={runtime.storage.provider ?? 'local'} /></dd>
          </div>
          <div class="flex items-center justify-between gap-4 rounded-xl border border-border bg-background p-4">
            <dt class="text-muted-foreground">Status</dt>
            <dd><StatusBadge tone={runtime.storage.configured ? 'success' : 'warning'} label={runtime.storage.configured ? 'Configured' : 'Needs setup'} /></dd>
          </div>
        </dl>
      {:else}
        <StateNotice tone="neutral" title="Storage" message="Data storage diambil dari /api/system/runtime. Pastikan sesi Super Admin aktif." />
      {/if}
    </div>

  {:else if activeTab === 'tunnel'}
    <div class="panel space-y-4 p-6" role="tabpanel" id="settings-panel-tunnel" aria-labelledby="settings-tab-tunnel">
      <div>
        <h2 class="text-lg font-semibold">Tunnel</h2>
        <p class="mt-1 text-sm text-muted-foreground">Status Cloudflare tunnel opt-in. Start/stop tunnel dilakukan dari server, bukan dari UI ini.</p>
      </div>
      {#if runtime?.tunnel}
        <dl class="space-y-3 text-sm">
          <div class="flex items-center justify-between gap-4 rounded-xl border border-border bg-background p-4">
            <dt class="text-muted-foreground">State</dt>
            <dd><StatusBadge tone="neutral" label={runtime.tunnel.state ?? 'Off'} /></dd>
          </div>
        </dl>
      {:else}
        <StateNotice tone="neutral" title="Tunnel" message="Status tunnel diambil dari runtime backend." />
      {/if}
      <StateNotice tone="warning" title="Opt-in only" message="Tunnel tidak pernah diaktifkan otomatis. Jangan expose port PostgreSQL atau debug melalui tunnel." />
    </div>

  {:else if activeTab === 'package'}
    <div class="panel space-y-4 p-6" role="tabpanel" id="settings-panel-package" aria-labelledby="settings-tab-package">
      <div>
        <h2 class="text-lg font-semibold">Package</h2>
        <p class="mt-1 text-sm text-muted-foreground">Status open-core package registry. Lihat detail di halaman Packages lama atau endpoint /api/packages.</p>
      </div>
      {#if runtime?.packages}
        <dl class="space-y-3 text-sm">
          <div class="flex items-center justify-between gap-4 rounded-xl border border-border bg-background p-4">
            <dt class="text-muted-foreground">Registered</dt>
            <dd class="font-semibold tabular-nums">{runtime.packages.registered ?? '—'}</dd>
          </div>
          <div class="flex items-center justify-between gap-4 rounded-xl border border-border bg-background p-4">
            <dt class="text-muted-foreground">Enabled capabilities</dt>
            <dd class="font-semibold tabular-nums">{runtime.packages.enabled_capabilities ?? '—'}</dd>
          </div>
        </dl>
      {:else}
        <StateNotice tone="neutral" title="Package" message="Data package diambil dari runtime backend." />
      {/if}
      <a class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-4 py-2.5 text-sm font-semibold" href="/admin/packages">
        Lihat package registry lengkap
      </a>
    </div>

  {:else if activeTab === 'demo'}
    <div class="panel space-y-4 p-6" role="tabpanel" id="settings-panel-demo" aria-labelledby="settings-tab-demo">
      <div>
        <h2 class="text-lg font-semibold">Demo Data</h2>
        <p class="mt-1 text-sm text-muted-foreground">Seed atau hapus data demo clothing company (Batik Nusantara, Urban Threads, Modest Wear ID). Hanya Super Admin.</p>
      </div>
      <StateNotice
        tone="primary"
        title="Kelola demo data"
        message="Gunakan halaman Demo Data untuk seed atau hapus data demo clothing company. Endpoint tersedia dan siap dipakai."
        actionHref="/admin/settings/demo"
        actionLabel="Buka Demo Data"
      />
      <div class="rounded-2xl border border-border bg-background p-5 text-sm">
        <p class="font-semibold">Brand demo yang akan di-seed:</p>
        <ul class="mt-3 space-y-2 text-muted-foreground">
          <li class="flex items-center gap-2"><StatusBadge tone="primary" label="batik-nusantara" /> Batik Nusantara — produk batik premium</li>
          <li class="flex items-center gap-2"><StatusBadge tone="secondary" label="urban-threads" /> Urban Threads — pakaian casual urban</li>
          <li class="flex items-center gap-2"><StatusBadge tone="success" label="modest-wear-id" /> Modest Wear ID — busana modest modern</li>
        </ul>
      </div>
    </div>

  {:else if activeTab === 'reset'}
    <div class="panel space-y-4 p-6" role="tabpanel" id="settings-panel-reset" aria-labelledby="settings-tab-reset">
      <div class="flex items-start justify-between gap-4">
        <div>
          <h2 class="text-lg font-semibold text-destructive">Install Reset</h2>
          <p class="mt-1 text-sm text-muted-foreground">Reset install menghapus semua data marketplace dan membuka kembali install panel. Tidak dapat dibatalkan.</p>
        </div>
        <StatusBadge tone="destructive" label="Super Admin only" />
      </div>
      <StateNotice
        tone="destructive"
        title="Tindakan destruktif"
        message="Reset install akan menghapus semua data: brand, produk, order, user, dan session. Gunakan hanya sebagai jalur recovery resmi."
      />
      <a
        class="inline-flex min-h-11 items-center rounded-xl border border-destructive/30 bg-destructive/10 px-4 py-2.5 text-sm font-semibold text-destructive"
        href="/admin/settings/install"
      >
        Lanjut ke konfirmasi reset →
      </a>
    </div>
  {/if}
</AdminShell>
