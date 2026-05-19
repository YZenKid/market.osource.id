<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl } from '$lib/api/base';
  import AdminShell from '$lib/ui/AdminShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  type DashboardState = 'loading' | 'ready' | 'unauthenticated' | 'error';

  let state: DashboardState = 'loading';
  let error = '';
  let brandCount = 0;
  let productCount = 0;

  type MePayload = {
    user?: {
      role?: {
        code?: string;
      };
    };
  };

  async function loadDashboard() {
    state = 'loading';
    error = '';
    try {
      const [meResponse, brandsResponse, productsResponse] = await Promise.all([
        fetch(apiUrl('/api/auth/me'), { credentials: 'include' }),
        fetch(apiUrl('/api/admin/brands'), { credentials: 'include' }),
        fetch(apiUrl('/api/admin/products'), { credentials: 'include' })
      ]);

      if (meResponse.status === 401 || meResponse.status === 403) {
        state = 'unauthenticated';
        return;
      }

      if (!meResponse.ok) throw new Error('Sesi admin tidak dapat diverifikasi dari /api/auth/me.');
      if (!brandsResponse.ok || !productsResponse.ok) {
        throw new Error('Admin API belum siap atau database belum tersedia.');
      }

      const mePayload: MePayload = await meResponse.json();
      const brandsPayload = await brandsResponse.json();
      const productsPayload = await productsResponse.json();

      if (mePayload.user?.role?.code !== 'super_admin') {
        state = 'unauthenticated';
        error = 'Route ini diprioritaskan untuk Super Admin pada milestone 1.';
        return;
      }

      brandCount = Array.isArray(brandsPayload.brands) ? brandsPayload.brands.length : 0;
      productCount = Array.isArray(productsPayload.products) ? productsPayload.products.length : 0;
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
  <div slot="header-action" class="flex flex-wrap gap-2">
    <a class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-4 py-2.5 text-sm font-semibold" href="/admin/login">Admin login</a>
    <a class="inline-flex min-h-11 items-center rounded-xl bg-primary px-4 py-2.5 text-sm font-semibold text-primary-foreground" href="/admin/system">Cek system</a>
  </div>

  {#if state === 'loading'}
    <StateNotice tone="neutral" title="Memuat status admin" message="Menghubungi endpoint admin terproteksi untuk memastikan sesi dan database tersedia." />
  {:else if state === 'unauthenticated'}
    <StateNotice tone="warning" title="Admin membutuhkan sesi Super Admin" message={error || 'Endpoint admin dijaga backend. Selesaikan install atau login sebagai Super Admin sebelum mengelola brand, produk, package, dan system.'} actionHref="/admin/login" actionLabel="Buka admin login" />
  {:else if state === 'error'}
    <StateNotice tone="destructive" title="Status admin belum tersedia" message={error} />
  {/if}

  <section class="grid gap-4 md:grid-cols-2 xl:grid-cols-4" aria-label="Status cards">
    <article class="panel p-5">
      <p class="text-sm text-muted-foreground">Brands</p>
      <p class="mt-2 text-2xl font-bold tabular-nums">{state === 'ready' ? brandCount : '—'}</p>
      <StatusBadge tone="primary" label="Super Admin scoped" />
      <p class="mt-4 text-sm leading-6 text-muted-foreground">Seller dibuat oleh Super Admin dan akses brand harus brand-scoped.</p>
    </article>
    <article class="panel p-5">
      <p class="text-sm text-muted-foreground">Products</p>
      <p class="mt-2 text-2xl font-bold tabular-nums">{state === 'ready' ? productCount : '—'}</p>
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
      { href: '/admin/orders', title: 'Order workspace', copy: 'Pantau order lintas brand, status pembayaran, dan bukti transfer private secara terproteksi.', badge: 'Orders' },
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
