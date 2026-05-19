<script lang="ts">
  export let type: 'button' | 'submit' | 'reset' = 'button';
  export let href: string | undefined = undefined;
  export let tone: 'primary' | 'secondary' | 'destructive' = 'primary';
  export let loading = false;
  export let disabled = false;
  export let fullWidth = false;

  const toneClass = {
    primary: 'bg-primary text-primary-foreground border-primary hover:bg-primary/90',
    secondary: 'bg-surface text-foreground border-border hover:bg-muted',
    destructive: 'bg-destructive text-white border-destructive hover:bg-destructive/90'
  };
</script>

{#if href}
  <a
    class={`inline-flex min-h-11 items-center justify-center rounded-xl border px-4 py-2.5 text-sm font-semibold transition-colors ${fullWidth ? 'w-full' : ''} ${toneClass[tone]}`}
    {href}
  >
    <slot />
  </a>
{:else}
  <button
    {type}
    class={`inline-flex min-h-11 items-center justify-center rounded-xl border px-4 py-2.5 text-sm font-semibold transition-colors disabled:cursor-not-allowed disabled:opacity-50 ${fullWidth ? 'w-full' : ''} ${toneClass[tone]}`}
    {disabled}
    aria-busy={loading ? 'true' : 'false'}
  >
    <slot>{loading ? 'Memproses…' : 'Lanjut'}</slot>
  </button>
{/if}
