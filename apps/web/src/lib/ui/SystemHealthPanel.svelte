<script lang="ts">
  import StatusBadge from '$lib/ui/StatusBadge.svelte';
  import type { HealthItem } from '$lib/ui/types';

  export let items: HealthItem[] = [];
</script>

<section class="panel p-5">
  <div class="flex items-center justify-between gap-3">
    <div>
      <h2 class="text-lg font-semibold">System health</h2>
      <p class="text-sm text-muted-foreground">Read-only status install, storage, tunnel, database, demo data.</p>
    </div>
    <a class="inline-flex min-h-11 items-center rounded-xl border border-border px-3 py-2 text-sm font-semibold" href="/admin/settings">Settings</a>
  </div>

  <div class="mt-4 space-y-3">
    {#each items as item}
      <div class="rounded-2xl border border-border bg-background p-4">
        <div class="flex items-start justify-between gap-3">
          <div>
            <p class="font-semibold">{item.label}</p>
            <p class="mt-1 text-sm leading-6 text-muted-foreground">{item.helper}</p>
          </div>
          <StatusBadge tone={item.tone} label={item.value} />
        </div>
      </div>
    {/each}
  </div>
</section>
