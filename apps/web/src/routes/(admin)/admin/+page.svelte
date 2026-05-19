<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl } from '$lib/api/base';
  import AdminShell from '$lib/ui/AdminShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import KPICard from '$lib/ui/KPICard.svelte';
  import RecentOrdersTable from '$lib/ui/RecentOrdersTable.svelte';
  import TaskQueue from '$lib/ui/TaskQueue.svelte';
  import SystemHealthPanel from '$lib/ui/SystemHealthPanel.svelte';
  import type { RecentOrder, QueueItem, HealthItem } from '$lib/ui/types';

  type DashState = 'loading' | 'unauthenticated' | 'error' | 'ready';

  let state: DashState = 'loading';
  let error = '';

  // KPI
  let brandCount = 0;
  let productCount = 0;
  let orderCount = 0;
  let pendingPaymentCount = 0;

  // User
  let userName = '';
  let userEmail = '';
  let roleCode = '';
  let roleLabel = '';
  let scopeLabel = '';

  // Panels
  let recentOrders: RecentOrder[] = [];
  let taskItems: QueueItem[] = [];
  let healthItems: HealthItem[] = [];

  // Runtime
  let installLocked = false;
  let dbConnected = false;

  async function loadDashboard() {
    state = 'loading';
    error = '';
    try {
      const [meRes, brandsRes, productsRes, ordersRes] = await Promise.all([
        fetch(apiUrl('/api/auth/me'), { credentials: 'include' }),
        fetch(apiUrl('/api/admin/brands'), { credentials: 'include' }),
        fetch(apiUrl('/api/admin/products'), { credentials: 'include' }),
        fetch(apiUrl('/api/admin/orders'), { credentials: 'include' })
      ]);

      if (meRes.status === 401 || meRes.status === 403) {
        state = 'unauthenticated';
        return;
      }
      if (!meRes.ok) throw new Error('Sesi admin tidak dapat diverifikasi.');

      const me = await meRes.json().catch(() => ({}));
      const user = me?.user;
      userName = user?.name ?? '';
      userEmail = user?.email ?? '';
      roleCode = user?.role?.code ?? '';
      roleLabel = user?.role?.name ?? roleCode;

      const isSuperOrAdmin = roleCode === 'super_admin' || roleCode === 'admin';
      scopeLabel = isSuperOrAdmin ? 'Semua brand' : 'Brand assigned';

      if (brandsRes.ok) {
        const b = await brandsRes.json().catch(() => ({}));
        brandCount = Array.isArray(b.brands) ? b.brands.length : 0;
      }
      if (productsRes.ok) {
        const p = await productsRes.json().catch(() => ({}));
        productCount = Array.isArray(p.products) ? p.products.length : 0;
      }

      let orders: any[] = [];
      if (ordersRes.ok) {
        const o = await ordersRes.json().catch(() => ({}));
        orders = Array.isArray(o.orders) ? o.orders : [];
        orderCount = orders.length;
        pendingPaymentCount = orders.filter(
          (ord: any) => ord.payment_status === 'pending' || ord.payment_status === 'waiting_payment_verification'
        ).length;
      }

      // Build recent orders (last 10)
      recentOrders = orders.slice(0, 10).map((ord: any) => ({
        orderNumber: ord.order_number ?? '—',
        customerName: ord.customer_name ?? '—',
        brandLabel: ord.groups?.[0] ? `${ord.groups.length} brand` : '—',
        total: `Rp${Number(ord.total_snapshot ?? 0).toLocaleString('id-ID')}`,
        paymentStatus: ord.payment_status ?? 'unknown',
        fulfillmentStatus: ord.groups?.[0]?.fulfillment_status ?? 'unknown',
        timeLabel: ord.created_at ? new Date(ord.created_at).toLocaleDateString('id-ID') : '—',
        href: `/store/orders/${ord.public_tracking_token}`
      }));

      // Task queue
      const unprocessedOrders = orders.filter((o: any) => o.global_status === 'pending').length;
      taskItems = [
        {
          label: 'Payment menunggu verifikasi',
          count: pendingPaymentCount,
          helper: 'Order dengan bukti transfer yang belum diverifikasi.',
          href: '/admin/orders'
        },
        {
          label: 'Order belum diproses',
          count: unprocessedOrders,
          helper: 'Order masuk yang belum ada tindakan fulfillment.',
          href: '/admin/orders'
        },
        {
          label: 'Produk stok tipis',
          count: 0,
          helper: 'Produk dengan stok ≤ 5 unit perlu perhatian.',
          href: '/admin/products'
        }
      ];

      // Fetch install state for health panel
      const installRes = await fetch(apiUrl('/api/install/state'), { credentials: 'include' }).catch(() => null);
      if (installRes?.ok) {
        const inst = await installRes.json().catch(() => ({}));
        installLocked = Boolean(inst?.locked);
        dbConnected = Boolean(inst?.database_connected);
      }

      healthItems = [
        {
          label: 'Install lock',
          value: installLocked ? 'Locked' : 'Open',
          helper: installLocked ? 'Setup sudah dikunci server-side.' : 'Setup belum dikunci.',
          tone: installLocked ? 'success' : 'warning'
        },
        {
          label: 'Database',
          value: dbConnected ? 'Connected' : 'Unavailable',
          helper: 'Koneksi PostgreSQL backend.',
          tone: dbConnected ? 'success' : 'destructive'
        },
        {
          label: 'Storage',
          value: 'Local',
          helper: 'Storage lokal aktif. Konfigurasi di Settings.',
          tone: 'primary'
        },
        {
          label: 'Demo data',
          value: 'Lihat Settings',
          helper: 'Seed atau hapus data demo dari Settings → Demo Data.',
          tone: 'neutral'
        }
      ];

      state = 'ready';
    } catch (err) {
      error = err instanceof Error ? err.message : 'Gagal memuat dashboard.';
      state = 'error';
    }
  }

  onMount(loadDashboard);
</script>

<svelte:head>
  <title>Dashboard — Admin</title>
</svelte:head>

<AdminShell
  active="dashboard"
  title="Dashboard"
  description="Status operasional marketplace: KPI, order terbaru, task queue, dan system health."
  {scopeLabel}
  {roleLabel}
  {userName}
  {userEmail}
  healthBadge={state === 'ready' ? (healthItems.some(h => h.tone === 'destructive') ? 'error' : healthItems.some(h => h.tone === 'warning') ? 'warn' : 'ok') : undefined}
>
  <div slot="header-action" class="flex flex-wrap gap-2">
    <a class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-4 py-2.5 text-sm font-semibold" href="/store">Lihat storefront</a>
    <a class="inline-flex min-h-11 items-center rounded-xl bg-primary px-4 py-2.5 text-sm font-semibold text-primary-foreground" href="/admin/orders">Kelola order</a>
  </div>

  {#if state === 'loading'}
    <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
      {#each [1,2,3,4] as _}
        <div class="metric-card animate-pulse">
          <div class="h-3 w-24 rounded bg-muted"></div>
          <div class="mt-3 h-8 w-16 rounded bg-muted"></div>
          <div class="mt-3 h-3 w-32 rounded bg-muted"></div>
        </div>
      {/each}
    </div>
  {:else if state === 'unauthenticated'}
    <StateNotice
      tone="warning"
      title="Login diperlukan"
      message="Dashboard admin memerlukan sesi operator aktif. Login sebagai Super Admin, Admin, Seller, atau Karyawan."
      actionHref="/admin/login"
      actionLabel="Login operator"
    />
  {:else if state === 'error'}
    <StateNotice
      tone="destructive"
      title="Dashboard tidak dapat dimuat"
      message={error}
      actionHref="/admin/login"
      actionLabel="Coba login ulang"
    />
  {/if}

  {#if state === 'ready'}
    <!-- KPI grid -->
    <section class="grid gap-4 sm:grid-cols-2 xl:grid-cols-4" aria-label="KPI cards">
      <KPICard
        label="Brands aktif"
        value={brandCount}
        helper="Brand marketplace terdaftar."
        badge="Brands"
        badgeTone="primary"
        href="/admin/brands"
      />
      <KPICard
        label="Produk published"
        value={productCount}
        helper="Produk aktif di katalog storefront."
        badge="Products"
        badgeTone="success"
        href="/admin/products"
      />
      <KPICard
        label="Order open"
        value={orderCount}
        helper="Total order yang masuk ke sistem."
        badge="Orders"
        badgeTone="secondary"
        href="/admin/orders"
      />
      <KPICard
        label="Payment pending"
        value={pendingPaymentCount}
        helper="Menunggu verifikasi bukti transfer."
        badge={pendingPaymentCount > 0 ? 'Perlu aksi' : 'Clear'}
        badgeTone={pendingPaymentCount > 0 ? 'warning' : 'success'}
        href="/admin/orders"
      />
    </section>

    <!-- Recent orders + right rail -->
    <div class="grid gap-6 xl:grid-cols-[1fr_340px]">
      <RecentOrdersTable orders={recentOrders} />

      <div class="space-y-6">
        <TaskQueue items={taskItems} />
        <SystemHealthPanel items={healthItems} />
      </div>
    </div>
  {/if}
</AdminShell>
