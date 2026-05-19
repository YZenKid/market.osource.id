<script lang="ts">
  export let title: string;
  export let description: string | undefined = undefined;
  export let eyebrow: string | undefined = undefined;
</script>

<section class="panel bg-background p-5">
  <div class="section-heading">
    <div>
      {#if eyebrow}
        <p class="eyebrow">{eyebrow}</p>
      {/if}
      <h2 class="text-lg font-semibold text-foreground">{title}</h2>
      {#if description}
        <p class="mt-1 text-sm leading-6 text-muted-foreground">{description}</p>
      {/if}
    </div>
    <slot name="meta" />
  </div>
  <div class="mt-4">
    <slot />
  </div>
</section>
