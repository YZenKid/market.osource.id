<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl } from '$lib/api/base';
  import AdminShell from '$lib/ui/AdminShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  type DashboardState = 'loading' | 'ready' | 'unauthenticated' | 'error';

  let state: DashboardState = 'loading';
  let error = '';

  async function loadDashboard() {
    state = 'loading';
    error = '';
    try {
      const response = await fetch(apiUrl('/api/admin/brands'), { credentials: 'include' });
      if (response.status === 401 || response.status === 403) {
        state = 'unauthenticated';
        return;
      }
      if (!response.ok) throw new Error('Admin API belum siap atau database belum tersedia.');
      state = 'ready';
    } catch (loadError) {
      error = loadError instanceof Error ? loadError.message : 'Gagal memuat admin dashboard.';
      state = 'error';
    }
  }

  onMount(loadDashboard);
</script>

<svelte:head>
  <title>Admin Dashboard — market.osource.id</title>
</svelte:head>

<AdminShell
  active="dashboard"
  title="Status marketplace"
  description="Dashboard awal untuk operator: cek kesiapan brand, katalog, package gate, install lock, storage, dan tunnel tanpa mengekspos data sensitif."
>
  <a slot="header-action" class="inline-flex min-h-11 items-center rounded-xl bg-primary px-4 py-2.5 text-sm font-semibold text-primary-foreground" href="/admin/system">
    Cek system
  </a>

  {#if state === 'loading'}
    <StateNotice tone="neutral" title="Memuat status admin" message="Menghubungi endpoint admin terproteksi untuk memastikan sesi dan database tersedia." />
  {:else if state === 'unauthenticated'}
    <StateNotice tone="warning" title="Admin membutuhkan sesi Super Admin" message="Endpoint admin dijaga backend. Selesaikan install atau login sebagai Super Admin sebelum mengelola brand, produk, package, dan system." actionHref="/install" actionLabel="Buka install panel" />
  {:else if state === 'error'}
    <StateNotice tone="destructive" title="Status admin belum tersedia" message={error} />
  {/if}

  <section class="grid gap-4 md:grid-cols-2 xl:grid-cols-4" aria-label="Status cards">
    <article class="panel p-5">
      <p class="text-sm text-muted-foreground">Brands</p>
      <p class="mt-2 text-2xl font-bold tabular-nums">—</p>
      <StatusBadge tone="primary" label="Super Admin scoped" />
      <p class="mt-4 text-sm leading-6 text-muted-foreground">Seller dibuat oleh Super Admin dan akses brand harus brand-scoped.</p>
    </article>
    <article class="panel p-5">
      <p class="text-sm text-muted-foreground">Products</p>
      <p class="mt-2 text-2xl font-bold tabular-nums">—</p>
      <StatusBadge tone="success" label="Variant-priced" />
      <p class="mt-4 text-sm leading-6 text-muted-foreground">Harga MVP bersumber dari product variants, termasuk default/internal variant.</p>
    </article>
    <article class="panel p-5">
      <p class="text-sm text-muted-foreground">Packages</p>
      <p class="mt-2 text-2xl font-bold">Base</p>
      <StatusBadge tone="secondary" label="Capability registry" />
      <p class="mt-4 text-sm leading-6 text-muted-foreground">UI menampilkan status package, bukan dynamic plugin runtime penuh.</p>
    </article>
    <article class="panel p-5">
      <p class="text-sm text-muted-foreground">Payment proof</p>
      <p class="mt-2 text-2xl font-bold tabular-nums">Protected</p>
      <StatusBadge tone="warning" label="Private media only" />
      <p class="mt-4 text-sm leading-6 text-muted-foreground">Tidak ada preview bukti transfer sebagai public static media.</p>
    </article>
  </section>

  <section class="grid gap-4 lg:grid-cols-2" aria-label="Admin next sections">
    {#each [
      { href: '/admin/brands', title: 'Brand workspace', copy: 'Kelola brand marketplace dan siapkan assignment seller tanpa self-service registration.', badge: 'Brand members' },
      { href: '/admin/products', title: 'Catalog workspace', copy: 'Pantau produk, status publikasi, stock, dan kesiapan default variant.', badge: 'Products' },
      { href: '/admin/packages', title: 'Package status', copy: 'Lihat package open-core, capability, dan status enabled/unlicensed secara eksplisit.', badge: 'Open-core' },
      { href: '/admin/system', title: 'Runtime & tunnel', copy: 'Periksa storage local, install lock, database, dan tunnel opt-in dengan pesan risiko.', badge: 'System' }
    ] as item}
      <a class="panel block p-5 transition-colors hover:border-primary/40 hover:bg-muted/35" href={item.href}>
        <div class="flex items-start justify-between gap-4">
          <div>
            <h2 class="text-lg font-semibold">{item.title}</h2>
            <p class="mt-2 text-sm leading-6 text-muted-foreground">{item.copy}</p>
          </div>
          <StatusBadge tone="neutral" label={item.badge} />
        </div>
      </a>
    {/each}
  </section>
</AdminShell>
