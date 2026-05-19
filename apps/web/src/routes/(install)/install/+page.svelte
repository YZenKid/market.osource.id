<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl, getCsrfToken } from '$lib/api/base';
  import type { HealthItem } from '$lib/ui/types';
  import AsyncState from '$lib/ui/AsyncState.svelte';
  import FormSection from '$lib/ui/forms/FormSection.svelte';
  import InputField from '$lib/ui/forms/InputField.svelte';
  import SubmitButton from '$lib/ui/forms/SubmitButton.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';
  import SystemHealthPanel from '$lib/ui/SystemHealthPanel.svelte';

  export let data: { gatingUnavailable?: boolean; installState?: any };

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

  type SetupResponse = { state: string; locked: boolean; authenticated?: boolean; message?: string };
  type SessionState = 'checking' | 'authenticated' | 'anonymous' | 'unavailable';

  let installState: InstallState | null = data.installState ?? null;
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
  let seedDemo = true;

  let healthItems: HealthItem[] = [];

  async function loadInstallStatus() {
    loading = true;
    error = '';
    try {
      const [stateResponse, preflightResponse, sessionResponse] = await Promise.all([
        fetch(apiUrl('/api/install/state'), { credentials: 'include' }),
        fetch(apiUrl('/api/install/preflight'), { credentials: 'include' }),
        fetch(apiUrl('/api/auth/me'), { credentials: 'include' }).catch(() => null)
      ]);

      if (!stateResponse.ok || !preflightResponse.ok) {
        throw new Error('Install API belum tersedia. Pastikan reverse proxy mengarahkan /api ke Axum.');
      }

      installState = await stateResponse.json();
      preflight = await preflightResponse.json();
      sessionState = sessionResponse?.ok ? 'authenticated' : sessionResponse?.status === 401 || sessionResponse?.status === 403 ? 'anonymous' : 'unavailable';

      healthItems = [
        {
          label: 'Install state',
          value: installState?.state ?? 'loading',
          helper: 'State lifecycle install backend.',
          tone: installState?.locked ? 'success' : 'warning'
        },
        {
          label: 'Database',
          value: installState?.database_connected ? 'Connected' : 'Unavailable',
          helper: 'Koneksi database untuk bootstrap setup.',
          tone: installState?.database_connected ? 'success' : 'destructive'
        },
        {
          label: 'Session admin',
          value: sessionState === 'authenticated' ? 'Active' : 'Anonymous',
          helper: 'Setup yang berhasil akan membuat session Super Admin bila backend siap.',
          tone: sessionState === 'authenticated' ? 'success' : 'warning'
        },
        ...((preflight?.checks ?? []).slice(0, 2).map((check) => ({
          label: check.name.replaceAll('_', ' '),
          value: check.ok ? 'Ready' : 'Blocked',
          helper: check.message ?? 'Preflight check',
          tone: check.ok ? 'success' as const : 'destructive' as const
        })))
      ];
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
      const csrfToken = await getCsrfToken();
      const response = await fetch(apiUrl('/api/install/setup'), {
        method: 'POST',
        credentials: 'include',
        headers: { 'content-type': 'application/json', 'x-csrf-token': csrfToken },
        body: JSON.stringify({
          marketplace_name: marketplaceName,
          admin_name: adminName,
          admin_email: adminEmail,
          admin_password: adminPassword,
          seed_demo: seedDemo
        })
      });
      const payload: SetupResponse = await response.json().catch(() => ({} as SetupResponse));
      if (!response.ok) throw new Error(payload.message ?? 'Setup gagal.');
      success = payload.authenticated
        ? 'Setup berhasil, install panel terkunci, sesi Super Admin aktif, dan marketplace siap dipakai.'
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

<main class="mx-auto flex min-h-screen max-w-6xl flex-col justify-center px-4 py-8 sm:px-6 lg:px-8">
  <section class="grid gap-6 lg:grid-cols-[1fr_320px] lg:items-start">
    <div class="panel overflow-hidden">
      <div class="border-b border-border bg-muted/50 p-6">
        <div class="flex flex-col gap-4 md:flex-row md:items-start md:justify-between">
          <div>
            <p class="eyebrow">Install panel</p>
            <h1 class="mt-2 text-3xl font-bold tracking-tight sm:text-4xl">Setup marketplace pertama</h1>
            <p class="mt-2 max-w-2xl text-sm leading-6 text-muted-foreground">
              Buat Super Admin pertama, kunci setup server-side, dan opsional isi data demo clothing company untuk langsung melihat storefront bekerja.
            </p>
          </div>
          <StatusBadge tone={sessionState === 'authenticated' ? 'success' : sessionState === 'checking' ? 'neutral' : 'warning'} label={sessionState === 'authenticated' ? 'Admin session active' : sessionState === 'checking' ? 'Checking session' : 'No admin session'} />
        </div>
      </div>

      <div class="grid gap-6 p-6 lg:grid-cols-[1fr_240px]">
        <div class="space-y-6">
          {#if data.gatingUnavailable}
            <StateNotice tone="warning" title="Server gate belum aktif" message="Route server tidak dapat memverifikasi install state. UI ini tetap mencoba membaca state dari API di browser." />
          {/if}
          {#if error}
            <StateNotice tone="destructive" title="Install belum tersedia" message={error} />
          {/if}
          {#if success}
            <StateNotice tone="success" title="Setup berhasil" message={success} actionHref="/store" actionLabel="Buka storefront" />
          {/if}

          <AsyncState
            state={loading ? 'loading' : installState?.locked ? 'locked' : 'ready'}
            title={loading ? 'Memuat status install' : 'Install sudah terkunci'}
            message={loading ? 'Menghubungi API install dan preflight.' : 'Marketplace sudah ter-install. Lanjut ke storefront atau admin.'}
            actionHref={installState?.locked ? '/store' : undefined}
            actionLabel={installState?.locked ? 'Buka storefront' : undefined}
          >
            <form class="space-y-6" aria-label="Install setup form" on:submit|preventDefault={submitSetup}>
              <div class="overflow-hidden rounded-2xl border border-border bg-background">
                <img
                  src="/assets/install-onboarding.png"
                  alt="Ilustrasi onboarding: buat Super Admin, kunci setup, dan isi demo data clothing."
                  class="aspect-[4/3] w-full object-contain"
                  loading="eager"
                  decoding="async"
                />
              </div>

              <FormSection title="Informasi marketplace" description="Data ini dipakai untuk bootstrap marketplace dan account Super Admin pertama." eyebrow="Step 1">
                <div class="space-y-4">
                  <InputField id="marketplace-name" label="Nama marketplace" bind:value={marketplaceName} placeholder="Market Osource" required disabled={Boolean(installState?.locked) || submitting} />
                  <div class="field-grid">
                    <InputField id="admin-name" label="Nama Super Admin" bind:value={adminName} placeholder="Admin Market" required disabled={Boolean(installState?.locked) || submitting} />
                    <InputField id="admin-email" label="Email Super Admin" bind:value={adminEmail} type="email" placeholder="admin@example.com" required disabled={Boolean(installState?.locked) || submitting} autocomplete="email" />
                  </div>
                  <InputField id="admin-password" label="Password Super Admin" bind:value={adminPassword} type="password" minlength={12} autocomplete="new-password" required disabled={Boolean(installState?.locked) || submitting} helper="Minimal 12 karakter sesuai policy backend." />
                </div>
              </FormSection>

              <FormSection title="Demo data clothing company" description="Opsional. Cocok untuk mencoba dashboard, storefront, dan order flow tanpa input manual satu per satu." eyebrow="Step 2">
                <label class="flex cursor-pointer items-start gap-3 rounded-2xl border border-border bg-background p-4">
                  <input class="mt-0.5 h-5 w-5 shrink-0 rounded border-border accent-primary" type="checkbox" bind:checked={seedDemo} disabled={Boolean(installState?.locked) || submitting} />
                  <span class="text-sm leading-6">
                    <strong>Isi data demo clothing company</strong><br />
                    Seed 3 brand demo, seller demo, dan produk pakaian dengan varian ukuran/warna agar storefront langsung terisi.
                  </span>
                </label>
                <div class="grid gap-3 sm:grid-cols-3">
                  <div class="rounded-2xl border border-border bg-background p-4"><p class="font-semibold">Batik Nusantara</p><p class="mt-1 text-xs text-muted-foreground">Batik premium lokal</p></div>
                  <div class="rounded-2xl border border-border bg-background p-4"><p class="font-semibold">Urban Threads</p><p class="mt-1 text-xs text-muted-foreground">Casual urban modern</p></div>
                  <div class="rounded-2xl border border-border bg-background p-4"><p class="font-semibold">Modest Wear ID</p><p class="mt-1 text-xs text-muted-foreground">Busana modest kontemporer</p></div>
                </div>
              </FormSection>

              <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
                <p class="text-sm leading-6 text-muted-foreground">Setup hanya bisa dijalankan sekali. Setelah berhasil, root <code>/</code> akan diarahkan ke storefront.</p>
                <SubmitButton type="submit" tone="primary" loading={submitting} disabled={setupDisabled}>
                  {submitting ? 'Mengunci setup…' : installState?.locked ? 'Setup sudah terkunci' : 'Buat Super Admin & kunci setup'}
                </SubmitButton>
              </div>
            </form>
          </AsyncState>
        </div>

        <aside class="space-y-4">
          <SystemHealthPanel items={healthItems} />
          {#if !installState?.locked && sessionState === 'anonymous'}
            <StateNotice tone="warning" title="Belum ada sesi admin" message="Setup yang berhasil akan membuat sesi Super Admin bila backend mengembalikan cookie market_session." />
          {/if}
        </aside>
      </div>
    </div>
  </section>
</main>
