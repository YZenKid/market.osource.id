<script lang="ts">
  import { apiUrl, getCsrfToken } from '$lib/api/base';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  let email = '';
  let password = '';
  let submitting = false;
  let error = '';
  let success = '';

  async function login() {
    submitting = true;
    error = '';
    success = '';
    try {
      const csrfToken = await getCsrfToken();
      const response = await fetch(apiUrl('/api/auth/login'), {
        method: 'POST',
        credentials: 'include',
        headers: { 'content-type': 'application/json', 'x-csrf-token': csrfToken },
        body: JSON.stringify({ email, password })
      });
      const payload = await response.json().catch(() => ({}));
      if (!response.ok) throw new Error(payload.message ?? 'Login gagal. Pastikan kredensial benar.');
      success = 'Login berhasil. Lanjutkan ke admin dashboard.';
      password = '';
    } catch (loginError) {
      error = loginError instanceof Error ? loginError.message : 'Login gagal.';
    } finally {
      submitting = false;
    }
  }

  const roles = [
    { label: 'Super Admin', desc: 'Akses penuh: install, settings, semua brand.' },
    { label: 'Admin', desc: 'Kelola brand, produk, order. Tanpa settings sistem.' },
    { label: 'Seller', desc: 'Brand-scoped: produk dan order brand sendiri.' },
    { label: 'Karyawan', desc: 'Brand-scoped: fulfillment dan produk assigned.' }
  ];
</script>

<svelte:head>
  <title>Operator Login — market.osource.id</title>
</svelte:head>

<main class="mx-auto flex min-h-screen max-w-5xl flex-col justify-center px-4 py-10 sm:px-6">
  <div class="grid gap-8 lg:grid-cols-[1fr_420px] lg:items-center">

    <!-- Info panel (desktop left / mobile top strip) -->
    <section class="space-y-6">
      <div>
        <p class="eyebrow">Operator access</p>
        <h1 class="mt-2 text-3xl font-bold tracking-tight sm:text-4xl">Login ke admin panel</h1>
        <p class="mt-3 max-w-md text-sm leading-6 text-muted-foreground">
          Panel admin mendukung 4 role operasional. Akses disesuaikan per role secara server-side — UI hanya menyembunyikan menu, authz tetap di backend.
        </p>
      </div>

      <div class="grid gap-3 sm:grid-cols-2">
        {#each roles as role}
          <div class="rounded-2xl border border-border bg-surface p-4">
            <p class="text-sm font-semibold">{role.label}</p>
            <p class="mt-1 text-xs leading-5 text-muted-foreground">{role.desc}</p>
          </div>
        {/each}
      </div>

      <div class="rounded-2xl border border-border bg-muted/40 p-4 text-sm text-muted-foreground">
        <p class="font-semibold text-foreground">Keamanan login</p>
        <p class="mt-1 leading-6">Session cookie + CSRF protection. Password tidak pernah disimpan plain-text. Rate limit aktif per IP.</p>
      </div>

      <a class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-4 py-2.5 text-sm font-semibold" href="/store">
        ← Kembali ke storefront
      </a>
    </section>

    <!-- Login form -->
    <section class="panel p-6">
      <div class="flex items-start justify-between gap-4">
        <div>
          <p class="eyebrow">Operator login</p>
          <h2 class="mt-1 text-xl font-bold tracking-tight">Masuk ke admin</h2>
        </div>
        <StatusBadge tone="primary" label="CSRF protected" />
      </div>

      <form class="mt-6 space-y-5" on:submit|preventDefault={login} aria-label="Operator login form">
        {#if error}
          <StateNotice tone="destructive" title="Login gagal" message={error} />
        {/if}
        {#if success}
          <StateNotice tone="success" title="Login berhasil" message={success} actionHref="/admin" actionLabel="Buka dashboard" />
        {/if}

        <label class="block text-sm font-medium" for="login-email">
          Email
          <input
            id="login-email"
            class="mt-2 min-h-11 w-full rounded-xl border border-border bg-surface px-4 py-3 text-sm transition-colors focus:border-primary"
            bind:value={email}
            type="email"
            autocomplete="email"
            required
            disabled={submitting}
            placeholder="admin@example.com"
          />
        </label>

        <label class="block text-sm font-medium" for="login-password">
          Password
          <input
            id="login-password"
            class="mt-2 min-h-11 w-full rounded-xl border border-border bg-surface px-4 py-3 text-sm transition-colors focus:border-primary"
            bind:value={password}
            type="password"
            autocomplete="current-password"
            required
            disabled={submitting}
          />
        </label>

        <div class="flex flex-wrap gap-3 pt-1">
          <button
            type="submit"
            class="inline-flex min-h-11 items-center rounded-xl bg-primary px-5 py-3 text-sm font-semibold text-primary-foreground disabled:cursor-not-allowed disabled:opacity-50"
            disabled={submitting || email.trim().length === 0 || password.length === 0}
            aria-busy={submitting}
          >
            {submitting ? 'Memproses…' : 'Masuk ke admin'}
          </button>
          <a
            class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-4 py-2.5 text-sm font-semibold"
            href="/install"
          >Buka install panel</a>
        </div>
      </form>
    </section>
  </div>
</main>
