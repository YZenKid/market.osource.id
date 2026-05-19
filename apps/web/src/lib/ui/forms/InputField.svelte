<script lang="ts">
  export let id: string | undefined = undefined;
  export let label: string;
  export let type = 'text';
  export let value = '';
  export let placeholder = '';
  export let helper: string | undefined = undefined;
  export let error: string | undefined = undefined;
  export let required = false;
  export let disabled = false;
  export let readonly = false;
  // Use any to avoid TS strict HTMLAttribute union mismatch across Svelte versions
  export let autocomplete: string | null | undefined = undefined;
  export let minlength: number | undefined = undefined;
  export let inputmode: string | null | undefined = undefined;
  export let rows: number | undefined = undefined;
  export let as: 'input' | 'textarea' = 'input';

  $: describedBy = [helper ? `${id}-helper` : null, error ? `${id}-error` : null].filter(Boolean).join(' ') || undefined;
</script>

<label class="block text-sm font-medium text-foreground" for={id}>
  <span>{label}</span>
  {#if as === 'textarea'}
    <textarea
      id={id}
      class={`mt-2 min-h-11 w-full rounded-xl border bg-surface px-4 py-3 text-sm leading-6 text-foreground transition-colors placeholder:text-muted-foreground focus:border-primary ${error ? 'border-destructive/50' : 'border-border'}`}
      bind:value
      {placeholder}
      {required}
      {disabled}
      {readonly}
      autocomplete={autocomplete as any}
      {rows}
      aria-invalid={error ? 'true' : 'false'}
      aria-describedby={describedBy}
    ></textarea>
  {:else}
    <input
      id={id}
      class={`mt-2 min-h-11 w-full rounded-xl border bg-surface px-4 py-3 text-sm text-foreground transition-colors placeholder:text-muted-foreground focus:border-primary ${error ? 'border-destructive/50' : 'border-border'} ${readonly ? 'bg-muted text-muted-foreground' : ''}`}
      bind:value
      {type}
      {placeholder}
      {required}
      {disabled}
      {readonly}
      autocomplete={autocomplete as any}
      {minlength}
      inputmode={inputmode as any}
      aria-invalid={error ? 'true' : 'false'}
      aria-describedby={describedBy}
    />
  {/if}

  {#if helper}
    <span id={id ? `${id}-helper` : undefined} class="mt-1 block text-xs leading-5 text-muted-foreground">{helper}</span>
  {/if}
  {#if error}
    <span id={id ? `${id}-error` : undefined} class="mt-1 block text-xs font-medium leading-5 text-destructive">{error}</span>
  {/if}
</label>
