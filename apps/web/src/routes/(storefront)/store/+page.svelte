<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl } from '$lib/api/base';
  import { cart } from '$lib/stores/cart';
  import StoreShell from '$lib/ui/StoreShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  type Product = {
    id: string;
    name: string;
    slug: string;
    description?: string | null;
    brand_name?: string | null;
    status?: string | null;
    variants?: Array<{ id?: string; price?: string; stock?: number }>;
  };

  let products: Product[] = [];
  let loading = true;
  let usingFallback = false;
  let error = '';
  let addSuccess = '';

  const priceOf = (product: Product) => Number(product.variants?.[0]?.price ?? 0).toLocaleString('id-ID');

  async function addToCart(product: Product) {
    const variant = product.variants?.[0];
    if (!variant?.id) {
      error = 'Variant aktif untuk produk ini belum tersedia.';
      return;
    }
    if (Number(variant.stock ?? 0) <= 0) {
      error = 'Produk ini sedang habis dan belum bisa dimasukkan ke cart.';
      return;
    }
    await cart.add({
      productId: product.id,
      variantId: variant.id,
      name: product.name,
      slug: product.slug,
      description: product.description,
      brandName: product.brand_name,
      price: variant.price ?? '0',
      stock: Number(variant.stock ?? 0),
      quantity: 1
    });
    addSuccess = `${product.name} ditambahkan ke cart.`;
    // Auto-clear banner after 4 seconds so it doesn't persist across navigations.
    setTimeout(() => { addSuccess = ''; }, 4000);
  }

  async function loadProducts() {
    loading = true;
    error = '';
    addSuccess = '';
    try {
      const response = await fetch(apiUrl('/api/storefront/products'), { credentials: 'include' });
      if (!response.ok) throw new Error('Storefront API belum tersedia.');
      const payload = await response.json();
      const apiProducts = Array.isArray(payload.products) ? payload.products : [];
      products = apiProducts;
      usingFallback = false;
    } catch (loadError) {
      error = loadError instanceof Error ? loadError.message : 'Gagal memuat produk.';
      products = [];
      usingFallback = false;
    } finally {
      loading = false;
    }
  }

  onMount(loadProducts);
</script>

<svelte:head>
  <title>Storefront — market.osource.id</title>
</svelte:head>

<StoreShell active="store">
  <div class="mx-auto max-w-6xl px-4 py-8 sm:px-6 lg:px-8">
    <header class="grid gap-6 border-b border-border pb-8 lg:grid-cols-[1.2fr_0.8fr] lg:items-end">
      <div>
        <p class="text-sm font-semibold uppercase tracking-[0.2em] text-primary">Storefront MVP</p>
        <h1 class="mt-2 max-w-3xl text-3xl font-bold tracking-tight sm:text-4xl">Belanja lintas brand dalam satu checkout</h1>
        <p class="mt-3 max-w-2xl text-sm leading-6 text-muted-foreground">
          Product-first foundation untuk browse, cart, dan checkout manual transfer. Halaman ini memprioritaskan data nyata dari API storefront.
        </p>
      </div>
      <div class="panel p-4">
        <p class="text-sm font-semibold">Cart & checkout readiness</p>
        <p class="mt-2 text-sm leading-6 text-muted-foreground">Order lintas brand akan dikelompokkan per brand saat fulfillment. Payment proof tetap private/protected.</p>
        <div class="mt-4 flex flex-wrap gap-2">
          <a class="inline-flex min-h-11 items-center rounded-xl bg-primary px-4 py-2.5 text-sm font-semibold text-primary-foreground" href="/store/checkout">Checkout</a>
          <a class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-4 py-2.5 text-sm font-semibold" href="/admin/products">Kelola produk</a>
        </div>
      </div>
    </header>

    {#if loading}
      <div class="mt-6"><StateNotice tone="neutral" title="Memuat katalog" message="Mencoba mengambil produk dari /api/storefront/products." /></div>
    {:else if error}
      <div class="mt-6"><StateNotice tone="destructive" title="Katalog tidak dapat dimuat" message={error} actionHref="/admin/products" actionLabel="Buka admin produk" /></div>
    {:else if products.length === 0}
      <section class="mt-6 panel p-6">
        <div class="grid gap-5 md:grid-cols-[220px_1fr] md:items-center">
          <img
            src="/assets/empty-catalog.png"
            alt="Ilustrasi katalog kosong yang menandakan belum ada produk published di storefront."
            class="w-full rounded-xl border border-border object-contain aspect-[4/3]"
            loading="lazy"
            decoding="async"
          />
          <StateNotice tone="primary" title="Belum ada produk published" message="Katalog kosong. Admin perlu membuat brand dan produk published sebelum storefront menampilkan item nyata." actionHref="/admin/products" actionLabel="Buat produk di admin" />
        </div>
      </section>
    {/if}

    {#if addSuccess}
      <div class="mt-6"><StateNotice tone="success" title="Cart diperbarui" message={addSuccess} actionHref="/store/cart" actionLabel="Lihat cart" /></div>
    {/if}

    <section class="mt-8 grid gap-4 sm:grid-cols-2 lg:grid-cols-3" aria-label="Product listing">
      {#each products as product}
        <article class="panel overflow-hidden transition-colors hover:border-primary/40">
          <div class="flex aspect-[4/3] items-center justify-center bg-muted px-6 text-center text-sm text-muted-foreground" aria-hidden="true">
            <span class="rounded-xl border border-border bg-surface px-3 py-2">Gambar produk belum tersedia</span>
          </div>
          <div class="p-5">
            <div class="flex items-start justify-between gap-3">
              <p class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">{product.brand_name ?? 'Brand'}</p>
              <StatusBadge tone={Number(product.variants?.[0]?.stock ?? 0) > 0 ? 'success' : 'warning'} label={Number(product.variants?.[0]?.stock ?? 0) > 0 ? 'Ready' : 'Stock check'} />
            </div>
            <h2 class="mt-2 text-lg font-semibold">{product.name}</h2>
            <p class="mt-2 min-h-12 text-sm leading-6 text-muted-foreground">{product.description ?? 'Deskripsi produk belum tersedia.'}</p>
            <div class="mt-4 flex items-center justify-between gap-3">
              <span class="font-semibold tabular-nums">Rp{priceOf(product)}</span>
              <button class="rounded-xl border border-border bg-surface px-3 py-2 text-sm font-semibold text-primary disabled:cursor-not-allowed disabled:opacity-50" disabled={Number(product.variants?.[0]?.stock ?? 0) <= 0} on:click={() => addToCart(product)}>
                {Number(product.variants?.[0]?.stock ?? 0) > 0 ? 'Tambah' : 'Habis'}
              </button>
            </div>
          </div>
        </article>
      {/each}
    </section>
  </div>
</StoreShell>
