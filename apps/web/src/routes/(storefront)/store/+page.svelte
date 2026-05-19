<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/state';
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
    variants?: Array<{ id?: string; price?: string; stock?: number; name?: string }>;
  };

  const demoImages = ['/assets/demo/product-1.png', '/assets/demo/product-2.png', '/assets/demo/product-3.png'];

  let products: Product[] = [];
  let loading = true;
  let error = '';
  let addSuccess = '';
  let selectedBrand = '';
  let sortMode: 'featured' | 'price-asc' | 'price-desc' = 'featured';

  function imageFor(index: number) {
    return demoImages[index % demoImages.length];
  }

  function priceValue(product: Product) {
    return Number(product.variants?.[0]?.price ?? 0);
  }

  function priceLabel(product: Product) {
    return `Rp${priceValue(product).toLocaleString('id-ID')}`;
  }

  function stockValue(product: Product) {
    return Number(product.variants?.[0]?.stock ?? 0);
  }

  function inferCategory(product: Product) {
    const text = `${product.name} ${product.description ?? ''} ${product.brand_name ?? ''}`.toLowerCase();
    if (text.includes('batik')) return 'batik';
    if (text.includes('modest') || text.includes('hijab') || text.includes('tunik') || text.includes('abaya')) return 'modest';
    if (text.includes('outer') || text.includes('jacket') || text.includes('jaket')) return 'outerwear';
    if (text.includes('aksesori') || text.includes('tas') || text.includes('scarf')) return 'aksesori';
    return 'casual';
  }

  async function addToCart(product: Product) {
    const variant = product.variants?.[0];
    if (!variant?.id) {
      error = 'Variant aktif untuk produk ini belum tersedia.';
      return;
    }
    if (stockValue(product) <= 0) {
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
      stock: stockValue(product),
      quantity: 1
    });

    addSuccess = `${product.name} ditambahkan ke cart.`;
    setTimeout(() => {
      addSuccess = '';
    }, 4000);
  }

  async function loadProducts() {
    loading = true;
    error = '';
    try {
      const response = await fetch(apiUrl('/api/storefront/products'), { credentials: 'include' });
      if (!response.ok) throw new Error('Storefront API belum tersedia.');
      const payload = await response.json().catch(() => ({}));
      products = Array.isArray(payload.products) ? payload.products : [];
    } catch (loadError) {
      error = loadError instanceof Error ? loadError.message : 'Gagal memuat produk.';
      products = [];
    } finally {
      loading = false;
    }
  }

  $: categoryParam = page.url.searchParams.get('category') ?? '';
  $: brands = [...new Set(products.map((product) => product.brand_name).filter(Boolean))] as string[];
  $: visibleProducts = products
    .filter((product) => (selectedBrand ? product.brand_name === selectedBrand : true))
    .filter((product) => (categoryParam ? inferCategory(product) === categoryParam : true))
    .slice()
    .sort((a, b) => {
      if (sortMode === 'price-asc') return priceValue(a) - priceValue(b);
      if (sortMode === 'price-desc') return priceValue(b) - priceValue(a);
      return 0;
    });
  $: brandCards = brands.length > 0 ? brands.slice(0, 3) : ['Batik Nusantara', 'Urban Threads', 'Modest Wear ID'];

  onMount(loadProducts);
</script>

<svelte:head>
  <title>Storefront — market.osource.id</title>
</svelte:head>

<StoreShell active="store">
  <div class="mx-auto max-w-6xl px-4 py-8 sm:px-6 lg:px-8">
    <!-- Hero -->
    <section class="grid gap-6 lg:grid-cols-[1.05fr_0.95fr] lg:items-center">
      <div>
        <p class="eyebrow">Clothing storefront</p>
        <h1 class="mt-2 max-w-3xl text-3xl font-bold tracking-tight sm:text-4xl">
          Katalog fashion multi-brand untuk batik, casual, dan modest wear lokal.
        </h1>
        <p class="mt-3 max-w-2xl text-sm leading-6 text-muted-foreground">
          Jelajahi produk lintas brand dalam satu storefront. Cart dan checkout tetap satu alur, sementara fulfillment dipisah per brand di backend.
        </p>
        <div class="mt-6 flex flex-wrap gap-3">
          <a class="inline-flex min-h-11 items-center rounded-xl bg-primary px-5 py-3 text-sm font-semibold text-primary-foreground" href="#catalog">Belanja sekarang</a>
          <a class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-5 py-3 text-sm font-semibold" href="/store/cart">Lihat cart</a>
        </div>
        <div class="mt-6 flex flex-wrap gap-2">
          {#each brandCards as brand}
            <StatusBadge tone="secondary" label={brand} />
          {/each}
        </div>
      </div>

      <div class="grid grid-cols-2 gap-3 sm:grid-cols-3">
        {#each demoImages as image, index}
          <div class={`overflow-hidden rounded-2xl border border-border bg-surface shadow-panel ${index === 0 ? 'sm:col-span-2' : ''}`}>
            <img src={image} alt="Produk demo clothing marketplace" class="aspect-[4/5] h-full w-full object-cover" loading="eager" />
          </div>
        {/each}
      </div>
    </section>

    <!-- Brand strip -->
    <section class="mt-8 grid gap-4 sm:grid-cols-3">
      {#each brandCards as brand, index}
        <article class="panel p-5">
          <div class="flex items-start justify-between gap-3">
            <div>
              <p class="text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">Brand spotlight</p>
              <h2 class="mt-2 text-lg font-semibold">{brand}</h2>
            </div>
            <StatusBadge tone={index === 0 ? 'primary' : index === 1 ? 'secondary' : 'success'} label="Aktif" />
          </div>
          <p class="mt-3 text-sm leading-6 text-muted-foreground">
            {index === 0 ? 'Tekstur kain dan potongan batik premium untuk koleksi lokal.' : index === 1 ? 'Silhouette urban casual untuk kebutuhan daily wear.' : 'Busana modest modern dengan ritme warna yang hangat.'}
          </p>
        </article>
      {/each}
    </section>

    <!-- Feedback -->
    {#if addSuccess}
      <div class="mt-6">
        <StateNotice tone="success" title="Cart diperbarui" message={addSuccess} actionHref="/store/cart" actionLabel="Lihat cart" />
      </div>
    {/if}

    <!-- Filter row -->
    <section id="catalog" class="mt-8 rounded-2xl border border-border bg-surface p-4 sm:p-5">
      <div class="flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between">
        <div>
          <p class="text-sm font-semibold">Filter katalog</p>
          <p class="mt-1 text-sm text-muted-foreground">Filter kategori dari URL dan pilih brand/sort lokal di browser.</p>
        </div>
        <div class="grid gap-3 sm:grid-cols-2 lg:min-w-[420px]">
          <label class="block text-sm font-medium">
            Brand
            <select class="mt-2 min-h-11 w-full rounded-xl border border-border bg-background px-4 py-3 text-sm" bind:value={selectedBrand}>
              <option value="">Semua brand</option>
              {#each brands as brand}
                <option value={brand}>{brand}</option>
              {/each}
            </select>
          </label>
          <label class="block text-sm font-medium">
            Urutkan
            <select class="mt-2 min-h-11 w-full rounded-xl border border-border bg-background px-4 py-3 text-sm" bind:value={sortMode}>
              <option value="featured">Unggulan</option>
              <option value="price-asc">Harga terendah</option>
              <option value="price-desc">Harga tertinggi</option>
            </select>
          </label>
        </div>
      </div>
    </section>

    <!-- Product grid -->
    <section class="mt-8 grid grid-cols-2 gap-4 lg:grid-cols-4" aria-label="Product listing">
      {#if loading}
        <div class="col-span-full"><StateNotice tone="neutral" title="Memuat katalog" message="Mengambil produk published dari /api/storefront/products." /></div>
      {:else if error}
        <div class="col-span-full"><StateNotice tone="destructive" title="Katalog tidak dapat dimuat" message={error} actionHref="/admin/products" actionLabel="Buka admin produk" /></div>
      {:else if visibleProducts.length === 0}
        <div class="col-span-full panel p-6">
          <div class="grid gap-5 md:grid-cols-[240px_1fr] md:items-center">
            <img
              src="/assets/empty-catalog.png"
              alt="Ilustrasi katalog kosong yang menandakan belum ada produk published di storefront."
              class="aspect-[4/3] w-full rounded-xl border border-border object-contain"
              loading="lazy"
              decoding="async"
            />
            <StateNotice
              tone="primary"
              title="Belum ada produk untuk filter ini"
              message="Ubah filter atau seed demo data dari Settings → Demo Data. Jika katalog benar-benar kosong, admin perlu membuat produk published terlebih dahulu."
              actionHref="/admin/settings/demo"
              actionLabel="Buka demo data"
            />
          </div>
        </div>
      {:else}
        {#each visibleProducts as product, index}
          <article class="panel overflow-hidden transition-colors hover:border-primary/40">
            <div class="overflow-hidden bg-muted">
              <img
                src={imageFor(index)}
                alt={product.name}
                class="aspect-[4/5] h-full w-full object-cover transition-transform duration-150 hover:scale-[1.02]"
                loading="lazy"
                decoding="async"
              />
            </div>
            <div class="p-4 sm:p-5">
              <div class="flex items-start justify-between gap-3">
                <p class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">{product.brand_name ?? 'Brand'}</p>
                <StatusBadge tone={stockValue(product) > 0 ? 'success' : 'warning'} label={stockValue(product) > 0 ? `Stock ${stockValue(product)}` : 'Habis'} />
              </div>
              <h2 class="mt-2 text-base font-semibold sm:text-lg">{product.name}</h2>
              <p class="mt-2 min-h-[3rem] text-sm leading-6 text-muted-foreground">{product.description ?? 'Deskripsi produk belum tersedia.'}</p>
              <div class="mt-3 flex items-center gap-2">
                <StatusBadge tone="secondary" label={inferCategory(product)} />
                {#if product.variants?.[0]?.name}
                  <StatusBadge tone="neutral" label={product.variants[0].name ?? 'Default'} />
                {/if}
              </div>
              <div class="mt-4 flex items-center justify-between gap-3">
                <span class="font-semibold tabular-nums">{priceLabel(product)}</span>
                <button
                  class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-4 py-2 text-sm font-semibold text-primary disabled:cursor-not-allowed disabled:opacity-50"
                  disabled={stockValue(product) <= 0}
                  on:click={() => addToCart(product)}
                >
                  {stockValue(product) > 0 ? 'Tambah' : 'Habis'}
                </button>
              </div>
            </div>
          </article>
        {/each}
      {/if}
    </section>
  </div>
</StoreShell>
