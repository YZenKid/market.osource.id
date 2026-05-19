<script lang="ts">
  import StatusBadge from '$lib/ui/StatusBadge.svelte';
  import type { RecentOrder } from '$lib/ui/types';

  export let orders: RecentOrder[] = [];

  function paymentTone(status: string): 'neutral' | 'warning' | 'success' {
    if (status === 'verified') return 'success';
    if (status === 'waiting_payment_verification') return 'warning';
    return 'neutral';
  }

  function fulfillmentTone(status: string): 'warning' | 'primary' | 'success' {
    if (status === 'completed' || status === 'ready_to_ship') return 'success';
    if (status === 'processing') return 'primary';
    return 'warning';
  }
</script>

<section class="panel overflow-hidden">
  <div class="flex items-center justify-between gap-3 border-b border-border px-5 py-4">
    <div>
      <h2 class="text-lg font-semibold">Recent orders</h2>
      <p class="text-sm text-muted-foreground">Order terbaru dan status pembayaran lintas scope aktif.</p>
    </div>
    <StatusBadge tone="primary" label={`${orders.length} visible`} />
  </div>

  {#if orders.length === 0}
    <div class="p-5 text-sm text-muted-foreground">Belum ada order terbaru pada scope ini.</div>
  {:else}
    <div class="hidden md:block">
      <div class="grid grid-cols-[1fr_1fr_0.9fr_0.8fr_0.9fr_0.9fr_0.8fr] gap-3 border-b border-border bg-muted/40 px-5 py-3 text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">
        <span>Order</span>
        <span>Pelanggan</span>
        <span>Brand</span>
        <span>Total</span>
        <span>Payment</span>
        <span>Fulfillment</span>
        <span>Aksi</span>
      </div>
      {#each orders as order}
        <div class="grid grid-cols-[1fr_1fr_0.9fr_0.8fr_0.9fr_0.9fr_0.8fr] gap-3 border-b border-border px-5 py-4 text-sm last:border-b-0">
          <div>
            <p class="font-semibold">{order.orderNumber}</p>
            <p class="mt-1 text-xs text-muted-foreground">{order.timeLabel}</p>
          </div>
          <p>{order.customerName}</p>
          <p class="text-muted-foreground">{order.brandLabel}</p>
          <p class="font-semibold tabular-nums">{order.total}</p>
          <StatusBadge tone={paymentTone(order.paymentStatus)} label={order.paymentStatus} />
          <StatusBadge tone={fulfillmentTone(order.fulfillmentStatus)} label={order.fulfillmentStatus} />
          <a class="inline-flex min-h-11 items-center rounded-xl border border-border px-3 py-2 font-semibold" href={order.href}>Buka</a>
        </div>
      {/each}
    </div>

    <div class="grid gap-3 p-4 md:hidden">
      {#each orders as order}
        <article class="rounded-2xl border border-border bg-background p-4">
          <div class="flex items-start justify-between gap-3">
            <div>
              <p class="font-semibold">{order.orderNumber}</p>
              <p class="mt-1 text-sm text-muted-foreground">{order.customerName}</p>
            </div>
            <a class="inline-flex min-h-11 items-center rounded-xl border border-border px-3 py-2 text-sm font-semibold" href={order.href}>Buka</a>
          </div>
          <div class="mt-3 flex flex-wrap gap-2">
            <StatusBadge tone={paymentTone(order.paymentStatus)} label={order.paymentStatus} />
            <StatusBadge tone={fulfillmentTone(order.fulfillmentStatus)} label={order.fulfillmentStatus} />
            <StatusBadge tone="secondary" label={order.brandLabel} />
          </div>
          <div class="mt-3 flex items-center justify-between text-sm">
            <span class="text-muted-foreground">{order.timeLabel}</span>
            <span class="font-semibold tabular-nums">{order.total}</span>
          </div>
        </article>
      {/each}
    </div>
  {/if}
</section>
