<script lang="ts">
  import { apiUrl } from '$lib/api/base';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  let email = '';
  let password = '';
  let submitting = false;
  let error = '';
  let success = '';

  async function fetchCsrfToken() {
    const response = await fetch(apiUrl('/api/auth/csrf'), { credentials: 'include' });
    const payload = await response.json().catch(() => ({}));
    if (!response.ok || typeof payload.token !== 'string') {
      throw new Error(payload.message ?? 'Gagal menyiapkan CSRF token untuk login.');
    }
    return payload.token as string;
  }

  async function login() {
    submitting = true;
    error = '';
    success = '';
    try {
      const csrfToken = await fetchCsrfToken();
      const response = await fetch(apiUrl('/api/auth/login'), {
        method: 'POST',
        credentials: 'include',
        headers: {
          'content-type': 'application/json',
          'x-csrf-token': csrfToken
        },
        body: JSON.stringify({
          email,
          password
        })
      });
      const payload = await response.json().catch(() => ({}));
      if (!response.ok) {
        throw new Error(payload.message ?? 'Login gagal. Pastikan kredensial Super Admin benar.');
      }
      success = 'Login berhasil. Lanjutkan ke admin dashboard.';
      password = '';
    } catch (loginError) {
      error = loginError instanceof Error ? loginError.message : 'Login gagal.';
    } finally {
      submitting = false;
    }
  }
</script>

<svelte:head>
  <title>Admin Login — market.osource.id</title>
</svelte:head>

<main class="mx-auto flex min-h-screen w-full max-w-xl flex-col justify-center px-4 py-10">
  <section class="panel p-6">
    <div class="flex items-start justify-between gap-4">
      <div>
        <p class="text-xs font-semibold uppercase tracking-[0.2em] text-primary">Admin access</p>
        <h1 class="mt-2 text-2xl font-bold tracking-tight">Login Super Admin</h1>
      </div>
      <StatusBadge tone="primary" label="/api/auth/login" />
    </div>

    <p class="mt-3 text-sm leading-6 text-muted-foreground">Menggunakan session cookie + CSRF protection backend. Setelah login berhasil, buka dashboard admin.</p>

    <form class="mt-6 space-y-5" on:submit|preventDefault={login}>
      {#if error}
        <StateNotice tone="destructive" title="Login gagal" message={error} />
      {/if}
      {#if success}
        <StateNotice tone="success" title="Login berhasil" message={success} actionHref="/admin" actionLabel="Buka dashboard" />
      {/if}

      <label class="block text-sm font-medium">
        Email
        <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={email} type="email" autocomplete="email" required />
      </label>

      <label class="block text-sm font-medium">
        Password
        <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={password} type="password" autocomplete="current-password" required />
      </label>

      <div class="flex flex-wrap gap-3">
        <button class="min-h-11 rounded-xl bg-primary px-5 py-3 text-sm font-semibold text-primary-foreground disabled:cursor-not-allowed disabled:opacity-50" disabled={submitting}>
          {submitting ? 'Memproses login…' : 'Login'}
        </button>
        <a class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-4 py-2.5 text-sm font-semibold" href="/install">Buka install panel</a>
      </div>
    </form>
  </section>
</main>
