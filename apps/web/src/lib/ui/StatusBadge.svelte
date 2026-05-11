<script lang="ts">
  type Tone = 'neutral' | 'primary' | 'success' | 'warning' | 'destructive' | 'secondary';

  export let tone: Tone = 'neutral';
  export let label: string;

  const toneClass: Record<Tone, string> = {
    neutral: 'border-border bg-muted text-muted-foreground',
    primary: 'border-primary/30 bg-primary/10 text-primary',
    success: 'border-success/30 bg-success/10 text-success',
    warning: 'border-warning/30 bg-warning/10 text-warning',
    destructive: 'border-destructive/30 bg-destructive/10 text-destructive',
    secondary: 'border-secondary/40 bg-secondary/15 text-secondary-foreground'
  };
</script>

<span class={`status-chip ${toneClass[tone]}`}>
  <slot>{label}</slot>
</span>
