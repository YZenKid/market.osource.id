<script lang="ts">
  type Tone = 'neutral' | 'primary' | 'success' | 'warning' | 'destructive';

  export let tone: Tone = 'neutral';
  export let title: string;
  export let message: string;
  export let actionHref: string | undefined = undefined;
  export let actionLabel: string | undefined = undefined;

  const toneClass: Record<Tone, string> = {
    neutral: 'border-border bg-muted/50 text-foreground',
    primary: 'border-primary/30 bg-primary/10 text-primary',
    success: 'border-success/30 bg-success/10 text-success',
    warning: 'border-warning/30 bg-warning/10 text-warning',
    destructive: 'border-destructive/30 bg-destructive/10 text-destructive'
  };
</script>

<div class={`rounded-2xl border p-4 ${toneClass[tone]}`} role={tone === 'destructive' ? 'alert' : 'status'}>
  <p class="font-semibold">{title}</p>
  <p class="mt-1 text-sm leading-6 opacity-90">{message}</p>
  {#if actionHref && actionLabel}
    <a class="mt-3 inline-flex min-h-11 items-center rounded-xl border border-current/30 px-4 py-2 text-sm font-semibold" href={actionHref}>
      {actionLabel}
    </a>
  {/if}
</div>
