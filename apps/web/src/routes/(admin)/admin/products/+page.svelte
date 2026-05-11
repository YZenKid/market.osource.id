<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl } from '$lib/api/base';
  import AdminShell from '$lib/ui/AdminShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  type Product = { id: string; name: string; slug: string; status?: string | null; brand_name?: string | null; variants?: Array<{ price?: string; stock?: number }> };
  let products: Product[] = [];
  let loading = true;
  let permission = false;
  let error = '';

  async function loadProducts() {
    loading = true;
    permission = false;
    error = '';
    try {
      const response = await fetch(apiUrl('/api/admin/products'), { credentials: 'include' });
      if (response.status === 401 || response.status === 403) {
        permission = true;
        return;
      }
      if (!response.ok) throw new Error('Product API belum tersedia.');
      const payload = await response.json();
      products = Array.isArray(payload.products) ? payload.products : [];
    } catch (loadError) {
      error = loadError instanceof Error ? loadError.message : 'Gagal memuat produk.';
    } finally {
      loading = false;
    }
  }

  onMount(loadProducts);
</script>

<svelte:head><title>Products — Admin</title></svelte:head>

<AdminShell active="products" title="Products" description="Catalog admin foundation dengan status, brand context, variant price, stock, dan empty/error states.">
  {#if loading}
    <StateNotice tone="neutral" title="Memuat produk" message="Mengambil katalog admin dari endpoint terproteksi." />
  {:else if permission}
    <StateNotice tone="warning" title="Butuh izin admin" message="Daftar produk admin memerlukan sesi Super Admin. Seller-scoped UI belum diklaim pada Gate H ini." actionHref="/install" actionLabel="Cek install/login" />
  {:else if error}
    <StateNotice tone="destructive" title="Produk belum dapat dimuat" message={error} />
  {:else if products.length === 0}
    <StateNotice tone="primary" title="Belum ada produk" message="Tambahkan brand dan produk melalui endpoint admin. Produk sederhana tetap memakai default/internal variant sesuai ERD/TRD." />
  {:else}
    <section class="panel overflow-hidden" aria-label="Product table">
      <div class="grid gap-3 border-b border-border bg-muted/40 p-4 text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground md:grid-cols-[1.4fr_1fr_0.7fr_0.7fr]">
        <span>Produk</span><span>Brand</span><span>Stock</span><span>Status</span>
      </div>
      {#each products as product}
        <article class="grid gap-3 border-b border-border p-4 last:border-b-0 md:grid-cols-[1.4fr_1fr_0.7fr_0.7fr] md:items-center">
          <div>
            <h2 class="font-semibold">{product.name}</h2>
            <p class="mt-1 font-mono text-xs text-muted-foreground">{product.slug}</p>
          </div>
          <p class="text-sm text-muted-foreground">{product.brand_name ?? 'Brand belum dimuat'}</p>
          <p class="text-sm font-semibold tabular-nums">{product.variants?.[0]?.stock ?? '—'}</p>
          <StatusBadge tone={product.status === 'published' ? 'success' : 'neutral'} label={product.status ?? 'draft'} />
        </article>
      {/each}
    </section>
  {/if}
</AdminShell>
