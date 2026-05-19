<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl, getCsrfToken } from '$lib/api/base';
  import AdminShell from '$lib/ui/AdminShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  let password = '';
  let confirmed = false;
  let submitting = false;
  let error = '';
  let success = '';
  let roleCode = '';
  let loading = true;

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

  async function submitReset() {
    if (!confirmed || password.length < 12) return;
    submitting = true;
    error = '';
    success = '';
    try {
      const csrfToken = await getCsrfToken();
      const res = await fetch(apiUrl('/api/admin/install/reset'), {
        method: 'POST',
        credentials: 'include',
        headers: { 'content-type': 'application/json', 'x-csrf-token': csrfToken },
        body: JSON.stringify({ password, confirmed: true })
      });
      const payload = await res.json().catch(() => ({}));
      if (!res.ok) throw new Error(payload.message ?? 'Reset install gagal.');
      success = 'Install berhasil direset. Marketplace kembali ke state pre-install. Buka /install untuk setup ulang.';
      password = '';
      confirmed = false;
    } catch (err) {
      error = err instanceof Error ? err.message : 'Reset gagal.';
    } finally {
      submitting = false;
    }
  }

  $: isSuperAdmin = roleCode === 'super_admin';
  $: resetDisabled = submitting || !confirmed || password.length < 12;

  onMount(loadRole);
</script>

<svelte:head>
  <title>Install Reset — Admin Settings</title>
</svelte:head>

<AdminShell
  active="settings"
  title="Install Reset"
  description="Jalur recovery resmi: reset install menghapus semua data dan membuka kembali install panel."
  eyebrow="Settings → Install Reset"
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
      title="Akses ditolak"
      message="Install reset hanya dapat dilakukan oleh Super Admin. Role aktif: {roleCode || 'tidak terautentikasi'}."
      actionHref="/admin/login"
      actionLabel="Login sebagai Super Admin"
    />
  {:else}
    {#if success}
      <StateNotice
        tone="success"
        title="Reset berhasil"
        message={success}
        actionHref="/install"
        actionLabel="Buka install panel"
      />
    {:else}
      <div class="space-y-6">
        <StateNotice
          tone="destructive"
          title="Peringatan: tindakan tidak dapat dibatalkan"
          message="Reset install akan menghapus semua data marketplace termasuk brand, produk, order, user, dan session aktif. Pastikan Anda memiliki backup sebelum melanjutkan."
        />

        <section class="panel p-6 space-y-5">
          <div class="flex items-start justify-between gap-4">
            <div>
              <h2 class="text-lg font-semibold">Konfirmasi reset install</h2>
              <p class="mt-1 text-sm text-muted-foreground">Masukkan password Super Admin dan centang konfirmasi untuk melanjutkan.</p>
            </div>
            <StatusBadge tone="destructive" label="Destruktif" />
          </div>

          {#if error}
            <StateNotice tone="destructive" title="Reset gagal" message={error} />
          {/if}

          <form class="space-y-5" on:submit|preventDefault={submitReset} aria-label="Install reset form">
            <label class="block text-sm font-medium" for="reset-password">
              Password Super Admin
              <input
                id="reset-password"
                class="mt-2 min-h-11 w-full rounded-xl border border-border bg-surface px-4 py-3 text-sm transition-colors focus:border-destructive"
                type="password"
                bind:value={password}
                autocomplete="current-password"
                minlength={12}
                required
                disabled={submitting}
                placeholder="Password minimal 12 karakter"
              />
              <span class="mt-1 block text-xs text-muted-foreground">Minimal 12 karakter. Diverifikasi server-side sebelum reset dijalankan.</span>
            </label>

            <label class="flex cursor-pointer items-start gap-3 rounded-2xl border border-destructive/30 bg-destructive/5 p-4">
              <input
                class="mt-0.5 h-5 w-5 shrink-0 rounded border-border accent-destructive"
                type="checkbox"
                bind:checked={confirmed}
                disabled={submitting}
              />
              <span class="text-sm leading-6">
                <strong>Saya memahami bahwa semua data marketplace akan dihapus permanen</strong> dan tindakan ini tidak dapat dibatalkan. Saya sudah memiliki backup atau tidak memerlukan data yang ada.
              </span>
            </label>

            <button
              type="submit"
              class="inline-flex min-h-11 items-center rounded-xl border border-destructive bg-destructive px-5 py-3 text-sm font-semibold text-white disabled:cursor-not-allowed disabled:opacity-50"
              disabled={resetDisabled}
              aria-busy={submitting}
            >
              {submitting ? 'Mereset install…' : 'Reset install sekarang'}
            </button>
          </form>
        </section>
      </div>
    {/if}
  {/if}
</AdminShell>
