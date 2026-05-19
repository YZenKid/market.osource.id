<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl } from '$lib/api/base';
  import AdminShell from '$lib/ui/AdminShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  type Brand = { id: string; name: string; slug: string; status?: string | null; description?: string | null };
  type UserOption = { id: string; name: string; email: string; role_code?: string; role_name?: string; status?: string };

  let brands: Brand[] = [];
  let users: UserOption[] = [];

  let loading = true;
  let permission = false;
  let error = '';

  let creatingBrand = false;
  let creatingSeller = false;
  let creatingMember = false;
  let brandSuccess = '';
  let sellerSuccess = '';
  let memberSuccess = '';

  let brandName = '';
  let brandSlug = '';
  let brandDescription = '';
  let brandStatus = 'active';

  let assignBrandId = '';
  let assignUserId = '';
  let assignMemberRole = 'seller';

  let sellerName = '';
  let sellerEmail = '';
  let sellerPassword = '';

  async function loadBrands() {
    loading = true;
    permission = false;
    error = '';
    brandSuccess = '';
    sellerSuccess = '';
    memberSuccess = '';
    try {
      const [brandsResponse, usersResponse] = await Promise.all([
        fetch(apiUrl('/api/admin/brands'), { credentials: 'include' }),
        fetch(apiUrl('/api/admin/users'), { credentials: 'include' })
      ]);

      if (brandsResponse.status === 401 || brandsResponse.status === 403) {
        permission = true;
        return;
      }
      if (!brandsResponse.ok) throw new Error('Brand API belum tersedia.');

      const brandsPayload = await brandsResponse.json();
      brands = Array.isArray(brandsPayload.brands) ? brandsPayload.brands : [];
      if (!assignBrandId && brands.length > 0) assignBrandId = brands[0].id;

      if (usersResponse.ok) {
        const usersPayload = await usersResponse.json();
        users = Array.isArray(usersPayload.users) ? usersPayload.users : [];
        const firstSeller = users.find((user) => user.role_code === 'seller') ?? users[0];
        if (!assignUserId && firstSeller?.id) assignUserId = firstSeller.id;
      }
    } catch (loadError) {
      error = loadError instanceof Error ? loadError.message : 'Gagal memuat brand.';
    } finally {
      loading = false;
    }
  }

  async function fetchCsrfToken() {
    const response = await fetch(apiUrl('/api/auth/csrf'), { credentials: 'include' });
    const payload = await response.json().catch(() => ({}));
    if (!response.ok || typeof payload.token !== 'string') {
      throw new Error(payload.message ?? 'Gagal menyiapkan token keamanan.');
    }
    return payload.token as string;
  }

  async function createBrand() {
    creatingBrand = true;
    error = '';
    brandSuccess = '';
    try {
      const csrfToken = await fetchCsrfToken();
      const response = await fetch(apiUrl('/api/admin/brands'), {
        method: 'POST',
        credentials: 'include',
        headers: {
          'content-type': 'application/json',
          'x-csrf-token': csrfToken
        },
        body: JSON.stringify({
          name: brandName,
          slug: brandSlug,
          description: brandDescription || null,
          status: brandStatus
        })
      });
      const payload = await response.json().catch(() => ({}));
      if (!response.ok) throw new Error(payload.message ?? 'Gagal membuat brand.');
      brandSuccess = `Brand ${payload.name ?? brandName} berhasil dibuat.`;
      brandName = '';
      brandSlug = '';
      brandDescription = '';
      await loadBrands();
    } catch (submitError) {
      error = submitError instanceof Error ? submitError.message : 'Gagal membuat brand.';
    } finally {
      creatingBrand = false;
    }
  }

  async function createSeller() {
    creatingSeller = true;
    error = '';
    sellerSuccess = '';
    try {
      const csrfToken = await fetchCsrfToken();
      const response = await fetch(apiUrl('/api/admin/users'), {
        method: 'POST',
        credentials: 'include',
        headers: {
          'content-type': 'application/json',
          'x-csrf-token': csrfToken
        },
        body: JSON.stringify({
          name: sellerName,
          email: sellerEmail,
          password: sellerPassword
        })
      });
      const payload = await response.json().catch(() => ({}));
      if (!response.ok) throw new Error(payload.message ?? 'Gagal membuat seller.');
      sellerSuccess = `Seller ${payload.name ?? sellerName} berhasil dibuat.`;
      sellerName = '';
      sellerEmail = '';
      sellerPassword = '';
      await loadBrands();
    } catch (submitError) {
      error = submitError instanceof Error ? submitError.message : 'Gagal membuat seller.';
    } finally {
      creatingSeller = false;
    }
  }

  async function assignSellerToBrand() {
    creatingMember = true;
    error = '';
    memberSuccess = '';
    try {
      const csrfToken = await fetchCsrfToken();
      const response = await fetch(apiUrl('/api/admin/brand-members'), {
        method: 'POST',
        credentials: 'include',
        headers: {
          'content-type': 'application/json',
          'x-csrf-token': csrfToken
        },
        body: JSON.stringify({
          brand_id: assignBrandId,
          user_id: assignUserId,
          member_role: assignMemberRole
        })
      });
      const payload = await response.json().catch(() => ({}));
      if (!response.ok) throw new Error(payload.message ?? 'Gagal membuat assignment brand member.');
      memberSuccess = 'Assignment seller ke brand berhasil disimpan.';
    } catch (submitError) {
      error = submitError instanceof Error ? submitError.message : 'Gagal assign seller ke brand.';
    } finally {
      creatingMember = false;
    }
  }

  $: brandSlug = brandName
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9\s-]/g, '')
    .replace(/\s+/g, '-')
    .replace(/-+/g, '-');

  onMount(loadBrands);
</script>

<svelte:head><title>Brands — Admin</title></svelte:head>

<AdminShell active="brands" title="Brands" description="Super Admin dapat membuat brand marketplace dan menetapkan seller assignment lewat endpoint admin yang tersedia.">
  <a slot="header-action" class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-4 py-2.5 text-sm font-semibold" href="/admin/login">Admin login</a>

  {#if loading}
    <StateNotice tone="neutral" title="Memuat brand" message="Mengambil daftar brand dari endpoint admin terproteksi." />
  {:else if permission}
    <StateNotice tone="warning" title="Butuh sesi Super Admin" message="Brand admin hanya boleh diakses oleh Super Admin. Login dulu untuk membuat brand atau assignment." actionHref="/admin/login" actionLabel="Buka admin login" />
  {:else}
    {#if error}
      <StateNotice tone="destructive" title="Operasi brand gagal" message={error} />
    {/if}
    {#if brandSuccess}
      <StateNotice tone="success" title="Brand tersimpan" message={brandSuccess} />
    {/if}
    {#if sellerSuccess}
      <StateNotice tone="success" title="Seller tersimpan" message={sellerSuccess} />
    {/if}
    {#if memberSuccess}
      <StateNotice tone="success" title="Assignment tersimpan" message={memberSuccess} />
    {/if}

    <section class="grid gap-4 xl:grid-cols-3" aria-label="Brand operations">
      <form class="panel space-y-4 p-5" on:submit|preventDefault={createBrand}>
        <div class="flex items-center justify-between gap-3">
          <h2 class="text-lg font-semibold">Buat brand baru</h2>
          <StatusBadge tone="primary" label="POST /api/admin/brands" />
        </div>

        <label class="block text-sm font-medium">
          Nama brand
          <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={brandName} placeholder="Brand Nusantara" required />
        </label>

        <label class="block text-sm font-medium">
          Slug brand
          <input class="mt-2 w-full rounded-xl border border-border bg-muted px-3 py-3 text-muted-foreground" bind:value={brandSlug} readonly />
        </label>

        <label class="block text-sm font-medium">
          Deskripsi
          <textarea class="mt-2 min-h-24 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={brandDescription} placeholder="Brand untuk produk kopi dan hampers lokal."></textarea>
        </label>

        <label class="block text-sm font-medium">
          Status
          <select class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={brandStatus}>
            <option value="active">active</option>
            <option value="draft">draft</option>
          </select>
        </label>

        <button class="min-h-11 rounded-xl bg-primary px-4 py-3 text-sm font-semibold text-primary-foreground disabled:cursor-not-allowed disabled:opacity-50" disabled={creatingBrand || brandName.trim().length === 0 || brandSlug.trim().length === 0}>
          {creatingBrand ? 'Menyimpan brand…' : 'Simpan brand'}
        </button>
      </form>

      <form class="panel space-y-4 p-5" on:submit|preventDefault={createSeller}>
        <div class="flex items-center justify-between gap-3">
          <h2 class="text-lg font-semibold">Buat seller</h2>
          <StatusBadge tone="primary" label="POST /api/admin/users" />
        </div>

        <label class="block text-sm font-medium">
          Nama seller
          <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={sellerName} placeholder="Seller Brand A" required />
        </label>

        <label class="block text-sm font-medium">
          Email seller
          <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={sellerEmail} type="email" placeholder="seller@example.com" required />
        </label>

        <label class="block text-sm font-medium">
          Password sementara
          <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={sellerPassword} type="password" minlength="12" required />
          <span class="mt-1 block text-xs text-muted-foreground">Minimal 12 karakter untuk mengikuti policy auth backend.</span>
        </label>

        <button class="min-h-11 rounded-xl bg-primary px-4 py-3 text-sm font-semibold text-primary-foreground disabled:cursor-not-allowed disabled:opacity-50" disabled={creatingSeller || sellerName.trim().length === 0 || sellerEmail.trim().length === 0 || sellerPassword.length < 12}>
          {creatingSeller ? 'Membuat seller…' : 'Simpan seller'}
        </button>
      </form>

      <form class="panel space-y-4 p-5" on:submit|preventDefault={assignSellerToBrand}>
        <div class="flex items-center justify-between gap-3">
          <h2 class="text-lg font-semibold">Assign seller ke brand</h2>
          <StatusBadge tone="secondary" label="POST /api/admin/brand-members" />
        </div>

        <label class="block text-sm font-medium">
          Brand
          <select class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={assignBrandId} required>
            {#each brands as brand}
              <option value={brand.id}>{brand.name} ({brand.slug})</option>
            {/each}
          </select>
        </label>

        <label class="block text-sm font-medium">
          Seller/User
          <select class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={assignUserId} required>
            {#each users as user}
              <option value={user.id}>{user.name} — {user.email} ({user.role_code ?? 'unknown'})</option>
            {/each}
          </select>
          <span class="mt-1 block text-xs text-muted-foreground">Daftar user diambil dari backend dan seller baru akan muncul setelah form create seller berhasil.</span>
        </label>

        <label class="block text-sm font-medium">
          Role assignment
          <select class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={assignMemberRole}>
            <option value="seller">seller</option>
            <option value="manager">manager</option>
          </select>
        </label>

        <button class="min-h-11 rounded-xl bg-primary px-4 py-3 text-sm font-semibold text-primary-foreground disabled:cursor-not-allowed disabled:opacity-50" disabled={creatingMember || assignBrandId.length === 0 || assignUserId.length === 0}>
          {creatingMember ? 'Menyimpan assignment…' : 'Simpan assignment'}
        </button>
      </form>
    </section>

    {#if brands.length === 0}
      <StateNotice tone="primary" title="Belum ada brand" message="Buat brand pertama menggunakan form di atas agar produk bisa dibuat terhadap brand nyata." />
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
  {/if}
</AdminShell>
