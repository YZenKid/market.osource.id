<script lang="ts">
  import StateNotice from '$lib/ui/StateNotice.svelte';

  export let state: 'loading' | 'error' | 'empty' | 'success' | 'warning' | 'locked' | 'unauthenticated' | 'permission' | 'ready' = 'ready';
  export let title = '';
  export let message = '';
  export let actionHref: string | undefined = undefined;
  export let actionLabel: string | undefined = undefined;

  const toneMap = {
    loading: 'neutral',
    error: 'destructive',
    empty: 'primary',
    success: 'success',
    warning: 'warning',
    locked: 'success',
    unauthenticated: 'warning',
    permission: 'warning',
    ready: 'neutral'
  } as const;
</script>

{#if state !== 'ready'}
  <StateNotice tone={toneMap[state]} {title} {message} {actionHref} {actionLabel} />
{:else}
  <slot />
{/if}
