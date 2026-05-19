<script lang="ts">
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  export let label: string;
  export let value: string | number;
  export let helper: string;
  export let badge: string;
  export let badgeTone: 'neutral' | 'primary' | 'success' | 'warning' | 'destructive' | 'secondary' = 'primary';
  export let delta: string | undefined = undefined;
  export let href: string | undefined = undefined;
</script>

<svelte:element this={href ? 'a' : 'article'} href={href} class="metric-card block transition-colors hover:border-primary/30">
  <div class="flex items-start justify-between gap-3">
    <p class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">{label}</p>
    <StatusBadge tone={badgeTone} label={badge} />
  </div>
  <p class="mt-3 text-2xl font-bold tracking-tight tabular-nums lg:text-3xl">{value}</p>
  <div class="mt-3 flex items-center justify-between gap-3 text-sm">
    <p class="leading-6 text-muted-foreground">{helper}</p>
    {#if delta}
      <span class="status-chip border-border bg-muted text-muted-foreground">{delta}</span>
    {/if}
  </div>
</svelte:element>
