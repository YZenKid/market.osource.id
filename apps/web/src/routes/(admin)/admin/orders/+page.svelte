<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl } from '$lib/api/base';
  import AdminShell from '$lib/ui/AdminShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  type OrderGroup = { id: string; brand_id: string; fulfillment_status: string; subtotal_snapshot: string };
  type OrderRecord = {
    id: string;
    order_number: string;
    public_tracking_token: string;
    customer_name: string;
    payment_status: string;
    global_status: string;
    total_snapshot: string;
    item_count: number;
    groups: OrderGroup[];
    payment_proof_statuses: string[];
  };

  type PaymentProofRecord = {
    id: string;
    order_id: string;
    file_object_id: string;
    status: string;
    can_view_file: boolean;
    can_verify: boolean;
    can_reject: boolean;
  };

  let orders: OrderRecord[] = [];
  let paymentProofs: PaymentProofRecord[] = [];
  let loading = true;
  let permission = false;
  let error = '';
  let success = '';
  let roleCode = '';

  const allowedPermissions = [
    'payment_proof.view_assigned',
    'payment_proof.verify',
    'payment_proof.reject'
  ];

  let assignBrandMemberId = '';
  let assignPermissionCode = allowedPermissions[0];

  async function loadOrders() {
    loading = true;
    permission = false;
    error = '';
    success = '';
    try {
      const [ordersResponse, proofsResponse] = await Promise.all([
        fetch(apiUrl('/api/admin/orders'), { credentials: 'include' }),
        fetch(apiUrl('/api/admin/payment-proofs'), { credentials: 'include' })
      ]);
      if (ordersResponse.status === 401 || ordersResponse.status === 403) {
        permission = true;
        return;
      }
      if (!ordersResponse.ok) throw new Error('Order API belum tersedia.');
      const payload = await ordersResponse.json();
      orders = Array.isArray(payload.orders) ? payload.orders : [];

      const meResponse = await fetch(apiUrl('/api/auth/me'), { credentials: 'include' });
      if (meResponse.ok) {
        const mePayload = await meResponse.json().catch(() => ({}));
        roleCode = mePayload?.user?.role?.code ?? '';
      }

      if (proofsResponse.ok) {
        const proofsPayload = await proofsResponse.json();
        paymentProofs = Array.isArray(proofsPayload.payment_proofs) ? proofsPayload.payment_proofs : [];
      }
    } catch (loadError) {
      error = loadError instanceof Error ? loadError.message : 'Gagal memuat order.';
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

  async function decideProof(proofId: string, action: 'verify' | 'reject') {
    error = '';
    success = '';
    try {
      const csrfToken = await fetchCsrfToken();
      const response = await fetch(apiUrl(`/api/admin/payment-proofs/${proofId}/${action}`), {
        method: 'POST',
        credentials: 'include',
        headers: { 'x-csrf-token': csrfToken }
      });
      const payload = await response.json().catch(() => ({}));
      if (!response.ok) throw new Error(payload.message ?? `Gagal ${action} payment proof.`);
      success = `Payment proof berhasil di-${action === 'verify' ? 'verify' : 'reject'}.`;
      await loadOrders();
    } catch (submitError) {
      error = submitError instanceof Error ? submitError.message : 'Operasi payment proof gagal.';
    }
  }

  async function updateFulfillment(groupId: string, nextStatus: string) {
    error = '';
    success = '';
    try {
      const csrfToken = await fetchCsrfToken();
      const response = await fetch(apiUrl(`/api/admin/order-brand-groups/${groupId}/fulfillment`), {
        method: 'POST',
        credentials: 'include',
        headers: {
          'content-type': 'application/json',
          'x-csrf-token': csrfToken
        },
        body: JSON.stringify({
          fulfillment_status: nextStatus,
          note: `Updated from admin orders UI to ${nextStatus}`
        })
      });
      const payload = await response.json().catch(() => ({}));
      if (!response.ok) throw new Error(payload.message ?? 'Gagal update fulfillment status.');
      success = `Fulfillment brand-group diperbarui ke ${payload.fulfillment_status ?? nextStatus}.`;
      await loadOrders();
    } catch (submitError) {
      error = submitError instanceof Error ? submitError.message : 'Update fulfillment gagal.';
    }
  }

  async function grantPermission() {
    error = '';
    success = '';
    try {
      const csrfToken = await fetchCsrfToken();
      const response = await fetch(apiUrl('/api/admin/brand-member-permissions'), {
        method: 'POST',
        credentials: 'include',
        headers: {
          'content-type': 'application/json',
          'x-csrf-token': csrfToken
        },
        body: JSON.stringify({
          brand_member_id: assignBrandMemberId,
          permission_code: assignPermissionCode
        })
      });
      if (!response.ok && response.status !== 204) {
        const payload = await response.json().catch(() => ({}));
        throw new Error(payload.message ?? 'Gagal grant permission brand member.');
      }
      success = `Permission ${assignPermissionCode} berhasil ditambahkan ke brand member.`;
      await loadOrders();
    } catch (submitError) {
      error = submitError instanceof Error ? submitError.message : 'Grant permission gagal.';
    }
  }

  function isSuperAdmin() {
    return roleCode === 'super_admin';
  }

  function paymentProofsForOrder(orderId: string) {
    return paymentProofs.filter((proof) => proof.order_id === orderId);
  }

  onMount(loadOrders);
</script>

<svelte:head><title>Orders — Admin</title></svelte:head>

<AdminShell active="orders" title="Orders" description="Super Admin dan seller-assigned dapat memantau order lintas brand, status pembayaran, dan jalur bukti transfer private tanpa membuka media secara publik.">
  <a slot="header-action" class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-4 py-2.5 text-sm font-semibold" href="/admin/system">Cek runtime</a>

  {#if loading}
    <StateNotice tone="neutral" title="Memuat order" message="Mengambil daftar order admin/seller dari endpoint terproteksi." />
  {:else if permission}
    <StateNotice tone="warning" title="Butuh sesi seller/super admin" message="Order workspace hanya tersedia untuk role yang telah diautentikasi dan brand-scoped di backend." actionHref="/admin/login" actionLabel="Buka admin login" />
  {:else if error}
    <StateNotice tone="destructive" title="Order belum dapat dimuat" message={error} />
  {:else if orders.length === 0}
    <StateNotice tone="primary" title="Belum ada order" message="Selesaikan checkout dari storefront untuk membuat order lintas brand pertama dan menguji upload bukti transfer private." actionHref="/store/checkout" actionLabel="Buka checkout" />
  {:else}
    {#if success}
      <StateNotice tone="success" title="Operasi order berhasil" message={success} />
    {/if}

    {#if isSuperAdmin()}
      <section class="panel p-5">
        <div class="section-heading">
          <div>
            <h2 class="text-lg font-semibold">Grant payment-proof permission</h2>
            <p class="text-sm text-muted-foreground">Super Admin dapat memberikan permission granular ke brand member bila seller perlu lihat/verify/reject bukti transfer pada order brand-nya.</p>
          </div>
          <StatusBadge tone="secondary" label="Granular proof access" />
        </div>

        <form class="field-grid mt-4" on:submit|preventDefault={grantPermission}>
          <label class="block text-sm font-medium">
            Brand member ID
            <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3 font-mono text-sm" bind:value={assignBrandMemberId} placeholder="UUID brand member" required />
          </label>
          <label class="block text-sm font-medium">
            Permission code
            <select class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={assignPermissionCode}>
              {#each allowedPermissions as permissionCode}
                <option value={permissionCode}>{permissionCode}</option>
              {/each}
            </select>
          </label>
          <div class="md:col-span-2">
            <button class="min-h-11 rounded-xl bg-primary px-4 py-3 text-sm font-semibold text-primary-foreground disabled:cursor-not-allowed disabled:opacity-50" disabled={assignBrandMemberId.trim().length === 0}>
              Grant permission
            </button>
          </div>
        </form>
      </section>
    {/if}

    <section class="page-stack" aria-label="Order list">
      {#each orders as order}
        <article class="list-row">
          <div class="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
            <div>
              <p class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">{order.order_number}</p>
              <h2 class="mt-2 text-lg font-semibold">{order.customer_name}</h2>
              <p class="mt-1 text-sm text-muted-foreground">Tracking token: <span class="font-mono text-xs">{order.public_tracking_token}</span></p>
            </div>
            <div class="flex flex-wrap gap-2">
              <StatusBadge tone={order.payment_status === 'verified' ? 'success' : order.payment_status === 'waiting_payment_verification' ? 'warning' : 'neutral'} label={order.payment_status} />
              <StatusBadge tone="primary" label={order.global_status} />
              <StatusBadge tone="secondary" label={`${order.item_count} item`} />
            </div>
          </div>

          <div class="mt-4 grid gap-4 lg:grid-cols-[1fr_auto] lg:items-start">
            <div>
              <p class="text-sm font-medium">Brand groups</p>
              <div class="mt-3 grid gap-2 sm:grid-cols-2">
                {#each order.groups as group}
                  <div class="rounded-xl border border-border bg-muted/35 px-3 py-3 text-sm">
                    <p class="font-semibold">Brand group</p>
                    <p class="mt-1 text-muted-foreground">Fulfillment: {group.fulfillment_status}</p>
                    <p class="mt-1 font-semibold tabular-nums">Rp{Number(group.subtotal_snapshot ?? 0).toLocaleString('id-ID')}</p>
                    <div class="mt-3 flex flex-wrap gap-2">
                      {#each ['not_ready', 'processing', 'ready_to_ship', 'completed'] as nextStatus}
                        <button class="rounded-xl border border-border bg-surface px-3 py-2 text-xs font-semibold disabled:cursor-not-allowed disabled:opacity-50" disabled={group.fulfillment_status === nextStatus} on:click={() => updateFulfillment(group.id, nextStatus)}>
                          {nextStatus}
                        </button>
                      {/each}
                    </div>
                  </div>
                {/each}
              </div>
              <p class="mt-3 text-sm text-muted-foreground">Proof statuses: {order.payment_proof_statuses.length > 0 ? order.payment_proof_statuses.join(', ') : 'belum ada proof upload'}</p>

              {#if paymentProofsForOrder(order.id).length > 0}
                <div class="mt-4 space-y-3">
                  <p class="text-sm font-medium">Payment proofs</p>
                  {#each paymentProofsForOrder(order.id) as proof}
                    <div class="rounded-xl border border-border bg-background p-3 text-sm">
                      <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
                        <div>
                          <p class="font-semibold">Proof {proof.id}</p>
                          <p class="mt-1 font-mono text-xs text-muted-foreground">File {proof.file_object_id}</p>
                        </div>
                        <div class="flex flex-wrap gap-2">
                          <StatusBadge tone={proof.status === 'verified' ? 'success' : proof.status === 'rejected' ? 'destructive' : 'warning'} label={proof.status} />
                          {#if proof.can_view_file}
                            <a class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-3 py-2 text-sm font-semibold" href={apiUrl(`/media/${proof.file_object_id}`)} target="_blank" rel="noreferrer">Buka file</a>
                          {/if}
                          {#if proof.can_verify}
                            <button class="rounded-xl border border-border bg-surface px-3 py-2 text-sm font-semibold" on:click={() => decideProof(proof.id, 'verify')}>Verify</button>
                          {/if}
                          {#if proof.can_reject}
                            <button class="rounded-xl border border-destructive/30 bg-destructive/10 px-3 py-2 text-sm font-semibold text-destructive" on:click={() => decideProof(proof.id, 'reject')}>Reject</button>
                          {/if}
                        </div>
                      </div>
                    </div>
                  {/each}
                </div>
              {/if}
            </div>
            <div class="text-right">
              <p class="text-sm text-muted-foreground">Total</p>
              <p class="mt-2 text-xl font-bold tabular-nums">Rp{Number(order.total_snapshot ?? 0).toLocaleString('id-ID')}</p>
              <a class="mt-3 inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-4 py-2.5 text-sm font-semibold" href={`/store/orders/${order.public_tracking_token}`}>Lihat tracking</a>
            </div>
          </div>
        </article>
      {/each}
    </section>
  {/if}
</AdminShell>
