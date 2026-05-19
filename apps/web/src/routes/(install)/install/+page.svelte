<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl } from '$lib/api/base';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

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

  type SetupResponse = {
    state: string;
    locked: boolean;
    authenticated?: boolean;
  };

  type SessionState = 'checking' | 'authenticated' | 'anonymous' | 'unavailable';

  let installState: InstallState | null = null;
  let preflight: PreflightReport | null = null;
  let loading = true;
  let submitting = false;
  let error = '';
  let success = '';
  let sessionState: SessionState = 'checking';

  let marketplaceName = '';
  let adminName = '';
  let adminEmail = '';
  let adminPassword = '';

  async function fetchCsrfToken() {
    const response = await fetch(apiUrl('/api/auth/csrf'), { credentials: 'include' });
    const payload = await response.json();
    if (!response.ok || typeof payload.token !== 'string') {
      throw new Error(payload.message ?? 'Gagal menyiapkan token keamanan.');
    }
    return payload.token;
  }

  async function loadInstallStatus() {
    loading = true;
    error = '';
    try {
      const [stateResponse, preflightResponse, sessionResponse] = await Promise.all([
        fetch(apiUrl('/api/install/state'), { credentials: 'include' }),
        fetch(apiUrl('/api/install/preflight'), { credentials: 'include' }),
        fetch(apiUrl('/api/admin/brands'), { credentials: 'include' })
      ]);
      if (!stateResponse.ok || !preflightResponse.ok) {
        throw new Error('Install API belum tersedia. Pastikan reverse proxy mengarahkan /api ke Axum.');
      }
      installState = await stateResponse.json();
      preflight = await preflightResponse.json();
      sessionState = sessionResponse.ok ? 'authenticated' : sessionResponse.status === 401 || sessionResponse.status === 403 ? 'anonymous' : 'unavailable';
    } catch (loadError) {
      error = loadError instanceof Error ? loadError.message : 'Gagal memuat status install.';
      sessionState = 'unavailable';
    } finally {
      loading = false;
    }
  }

  async function submitSetup() {
    submitting = true;
    error = '';
    success = '';
    try {
      const csrfToken = await fetchCsrfToken();
      const response = await fetch(apiUrl('/api/install/setup'), {
        method: 'POST',
        credentials: 'include',
        headers: { 'content-type': 'application/json', 'x-csrf-token': csrfToken },
        body: JSON.stringify({
          marketplace_name: marketplaceName,
          admin_name: adminName,
          admin_email: adminEmail,
          admin_password: adminPassword
        })
      });
      const payload: SetupResponse & { message?: string } = await response.json();
      if (!response.ok) {
        throw new Error(payload.message ?? 'Setup gagal.');
      }
      success = payload.authenticated
        ? 'Setup berhasil, install panel terkunci, dan sesi Super Admin sudah aktif.'
        : 'Setup berhasil dan install panel sudah dikunci server-side.';
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
        <div class="flex flex-col gap-4 md:flex-row md:items-start md:justify-between">
          <div>
            <p class="text-sm font-semibold uppercase tracking-[0.2em] text-primary">Install panel</p>
            <h1 class="mt-2 text-3xl font-bold tracking-tight">Setup marketplace pertama</h1>
            <p class="mt-2 max-w-2xl text-sm leading-6 text-muted-foreground">
              Panel ini membuat Super Admin pertama, memakai CSRF untuk setup mutation, dan mengunci setup server-side setelah berhasil.
            </p>
          </div>
          <StatusBadge tone={sessionState === 'authenticated' ? 'success' : sessionState === 'checking' ? 'neutral' : 'warning'} label={sessionState === 'authenticated' ? 'Admin session active' : sessionState === 'checking' ? 'Checking session' : 'No admin session'} />
        </div>
      </div>

    <div class="grid gap-6 p-6 md:grid-cols-[1fr_300px]">
      <form class="space-y-5" aria-label="Install setup form" on:submit|preventDefault={submitSetup}>
        {#if !installState?.locked}
          <div class="hidden md:block">
            <img
              src="/assets/install-onboarding.png"
              alt="Ilustrasi langkah onboarding: buat Super Admin, kunci setup, dan buka admin dashboard."
              class="w-full rounded-xl border border-border object-contain aspect-[4/3]"
              loading="lazy"
              decoding="async"
            />
          </div>
        {/if}
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
          <StateNotice tone="success" title="Install sudah terkunci" message="Setup ulang ditolak oleh backend. Gunakan sesi Super Admin yang dibuat saat setup untuk membuka admin dashboard." actionHref="/admin" actionLabel="Buka admin" />
        {/if}

        {#if !installState?.locked && sessionState === 'anonymous'}
          <StateNotice tone="warning" title="Belum ada sesi admin" message="Setup yang berhasil akan membuat sesi Super Admin jika backend mengembalikan cookie market_session." />
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
            <dd><StatusBadge tone="primary" label={installState?.state ?? 'loading'} /></dd>
          </div>
          <div class="flex items-center justify-between gap-3">
            <dt>Lock</dt>
            <dd><StatusBadge tone={installState?.locked ? 'success' : 'warning'} label={installState?.locked ? 'Locked' : 'Open'} /></dd>
          </div>
          <div class="flex items-center justify-between gap-3">
            <dt>Admin session</dt>
            <dd><StatusBadge tone={sessionState === 'authenticated' ? 'success' : 'warning'} label={sessionState === 'authenticated' ? 'Authenticated' : 'Required'} /></dd>
          </div>
        </dl>
        <ul class="mt-5 space-y-3 text-sm">
          {#each preflight?.checks ?? [] as check}
            <li class="flex items-center justify-between gap-3">
              <span>{check.name.replaceAll('_', ' ')}</span>
              <StatusBadge tone={check.ok ? 'success' : 'destructive'} label={check.ok ? 'Ready' : 'Blocked'} />
            </li>
          {/each}
        </ul>
      </aside>
    </div>
  </section>
</main>
