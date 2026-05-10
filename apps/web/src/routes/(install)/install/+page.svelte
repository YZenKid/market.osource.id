<script lang="ts">
  import { onMount } from 'svelte';

  type InstallState = {
    state: string;
    locked: boolean;
    installed: boolean;
    runtime_mode: string | null;
    core_version: string | null;
    database_connected: boolean;
  };

  type PreflightReport = {
    ok: boolean;
    checks: Array<{ name: string; ok: boolean; message: string | null }>;
  };

  let installState: InstallState | null = null;
  let preflight: PreflightReport | null = null;
  let loading = true;
  let submitting = false;
  let error = '';
  let success = '';

  let marketplaceName = '';
  let adminName = '';
  let adminEmail = '';
  let adminPassword = '';

  async function loadInstallStatus() {
    loading = true;
    error = '';
    try {
      const [stateResponse, preflightResponse] = await Promise.all([
        fetch('/api/install/state'),
        fetch('/api/install/preflight')
      ]);
      if (!stateResponse.ok || !preflightResponse.ok) {
        throw new Error('Install API belum tersedia. Pastikan reverse proxy mengarahkan /api ke Axum.');
      }
      installState = await stateResponse.json();
      preflight = await preflightResponse.json();
    } catch (loadError) {
      error = loadError instanceof Error ? loadError.message : 'Gagal memuat status install.';
    } finally {
      loading = false;
    }
  }

  async function submitSetup() {
    submitting = true;
    error = '';
    success = '';
    try {
      const response = await fetch('/api/install/setup', {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({
          marketplace_name: marketplaceName,
          admin_name: adminName,
          admin_email: adminEmail,
          admin_password: adminPassword
        })
      });
      const payload = await response.json();
      if (!response.ok) {
        throw new Error(payload.message ?? 'Setup gagal.');
      }
      success = 'Setup berhasil dan install panel sudah dikunci server-side.';
      adminPassword = '';
      await loadInstallStatus();
    } catch (submitError) {
      error = submitError instanceof Error ? submitError.message : 'Setup gagal.';
    } finally {
      submitting = false;
    }
  }

  $: setupDisabled =
    loading ||
    submitting ||
    installState?.locked ||
    !preflight?.ok ||
    marketplaceName.trim().length === 0 ||
    adminName.trim().length === 0 ||
    adminEmail.trim().length === 0 ||
    adminPassword.length < 12;

  onMount(loadInstallStatus);
</script>

<svelte:head>
  <title>Install Panel — market.osource.id</title>
</svelte:head>

<main class="mx-auto flex min-h-screen max-w-4xl flex-col justify-center px-4 py-8">
  <section class="panel overflow-hidden">
    <div class="border-b border-border bg-muted/60 p-6">
      <p class="text-sm font-semibold uppercase tracking-[0.2em] text-primary">Install panel</p>
      <h1 class="mt-2 text-3xl font-bold tracking-tight">Setup marketplace pertama</h1>
      <p class="mt-2 max-w-2xl text-sm leading-6 text-muted-foreground">
        Panel ini membuat Super Admin pertama dan mengunci setup server-side setelah berhasil.
      </p>
    </div>

    <div class="grid gap-6 p-6 md:grid-cols-[1fr_300px]">
      <form class="space-y-5" aria-label="Install setup form" on:submit|preventDefault={submitSetup}>
        {#if loading}
          <div class="rounded-xl border border-border bg-muted/40 p-4 text-sm text-muted-foreground">Memuat status install…</div>
        {/if}

        {#if error}
          <div class="rounded-xl border border-destructive/30 bg-destructive/10 p-4 text-sm text-destructive" role="alert">{error}</div>
        {/if}

        {#if success}
          <div class="rounded-xl border border-success/30 bg-success/10 p-4 text-sm text-success" role="status">{success}</div>
        {/if}

        {#if installState?.locked}
          <div class="rounded-xl border border-success/30 bg-success/10 p-4 text-sm text-success" role="status">
            Install sudah terkunci. Setup ulang ditolak oleh backend.
          </div>
        {/if}

        <label class="block text-sm font-medium">
          Nama marketplace
          <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={marketplaceName} placeholder="Market Osource" disabled={installState?.locked || submitting} required />
        </label>
        <label class="block text-sm font-medium">
          Nama Super Admin
          <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={adminName} placeholder="Admin Market" disabled={installState?.locked || submitting} required />
        </label>
        <label class="block text-sm font-medium">
          Email Super Admin
          <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={adminEmail} type="email" placeholder="admin@example.com" disabled={installState?.locked || submitting} required />
        </label>
        <label class="block text-sm font-medium">
          Password Super Admin
          <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={adminPassword} type="password" minlength="12" autocomplete="new-password" disabled={installState?.locked || submitting} required />
          <span class="mt-1 block text-xs text-muted-foreground">Minimal 12 karakter.</span>
        </label>
        <button class="rounded-xl bg-primary px-5 py-3 text-sm font-semibold text-primary-foreground disabled:cursor-not-allowed disabled:opacity-50" disabled={setupDisabled}>
          {submitting ? 'Mengunci setup…' : installState?.locked ? 'Setup sudah terkunci' : 'Buat Super Admin & kunci setup'}
        </button>
      </form>

      <aside class="rounded-2xl border border-border bg-background p-5">
        <h2 class="font-semibold">Preflight</h2>
        <dl class="mt-4 space-y-3 text-sm">
          <div class="flex items-center justify-between gap-3">
            <dt>Install state</dt>
            <dd class="status-chip border-primary/30 bg-primary/10 text-primary">{installState?.state ?? 'loading'}</dd>
          </div>
          <div class="flex items-center justify-between gap-3">
            <dt>Lock</dt>
            <dd class={`status-chip ${installState?.locked ? 'border-success/30 bg-success/10 text-success' : 'border-warning/30 bg-warning/10 text-warning'}`}>{installState?.locked ? 'Locked' : 'Open'}</dd>
          </div>
        </dl>
        <ul class="mt-5 space-y-3 text-sm">
          {#each preflight?.checks ?? [] as check}
            <li class="flex items-center justify-between gap-3">
              <span>{check.name.replaceAll('_', ' ')}</span>
              <span class={`status-chip ${check.ok ? 'border-success bg-success/10 text-success' : 'border-destructive bg-destructive/10 text-destructive'}`} title={check.message ?? ''}>
                {check.ok ? 'Ready' : 'Blocked'}
              </span>
            </li>
          {/each}
        </ul>
      </aside>
    </div>
  </section>
</main>
