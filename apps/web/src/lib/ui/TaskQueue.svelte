<script lang="ts">
  import StatusBadge from '$lib/ui/StatusBadge.svelte';
  import type { QueueItem } from '$lib/ui/types';

  export let items: QueueItem[] = [];
</script>

<section class="panel p-5">
  <div class="flex items-center justify-between gap-3">
    <div>
      <h2 class="text-lg font-semibold">Task queue</h2>
      <p class="text-sm text-muted-foreground">Antrian kerja operasional yang perlu ditindak.</p>
    </div>
    <StatusBadge tone="warning" label={`${items.reduce((sum, item) => sum + item.count, 0)} total`} />
  </div>

  <div class="mt-4 space-y-3">
    {#each items as item}
      <a class="block rounded-2xl border border-border bg-background p-4 transition-colors hover:border-primary/30" href={item.href}>
        <div class="flex items-start justify-between gap-3">
          <div>
            <p class="font-semibold">{item.label}</p>
            <p class="mt-1 text-sm leading-6 text-muted-foreground">{item.helper}</p>
          </div>
          <StatusBadge tone={item.count > 0 ? 'warning' : 'success'} label={String(item.count)} />
        </div>
      </a>
    {/each}
  </div>
</section>
