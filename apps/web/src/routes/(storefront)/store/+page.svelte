<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl } from '$lib/api/base';
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

  const fallbackProducts: Product[] = [
    { id: 'fallback-1', name: 'Kopi Arabika Lokal', slug: 'kopi-arabika-lokal', brand_name: 'Brand Utama', description: 'Static fallback ketika katalog API belum tersedia.', status: 'published', variants: [{ price: '75000', stock: 24 }] },
    { id: 'fallback-2', name: 'Paket Hampers Komunitas', slug: 'hampers-komunitas', brand_name: 'Brand Partner', description: 'Contoh kartu produk dengan brand, harga variant, dan stock.', status: 'published', variants: [{ price: '125000', stock: 8 }] },
    { id: 'fallback-3', name: 'Kerajinan Rotan Mini', slug: 'kerajinan-rotan-mini', brand_name: 'Brand Kreatif', description: 'Fallback aman untuk validasi layout tanpa mengklaim data produksi.', status: 'published', variants: [{ price: '99000', stock: 3 }] }
  ];

  let products: Product[] = fallbackProducts;
  let loading = true;
  let usingFallback = true;
  let error = '';

  const priceOf = (product: Product) => Number(product.variants?.[0]?.price ?? 0).toLocaleString('id-ID');

  async function loadProducts() {
    loading = true;
    error = '';
    try {
      const response = await fetch(apiUrl('/api/storefront/products'), { credentials: 'include' });
      if (!response.ok) throw new Error('Storefront API belum tersedia.');
      const payload = await response.json();
      const apiProducts = Array.isArray(payload.products) ? payload.products : [];
      products = apiProducts.length > 0 ? apiProducts : [];
      usingFallback = false;
    } catch (loadError) {
      error = loadError instanceof Error ? loadError.message : 'Gagal memuat produk.';
      products = fallbackProducts;
      usingFallback = true;
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
          Product-first foundation untuk browse, cart, dan checkout manual transfer. Data memakai API bila tersedia dan fallback statis yang ditandai jelas bila backend belum siap.
        </p>
      </div>
      <div class="panel p-4">
        <p class="text-sm font-semibold">Cart & checkout readiness</p>
        <p class="mt-2 text-sm leading-6 text-muted-foreground">Order lintas brand akan dikelompokkan per brand saat fulfillment. Payment proof tetap private/protected.</p>
        <a class="mt-4 inline-flex min-h-11 items-center rounded-xl bg-primary px-4 py-2.5 text-sm font-semibold text-primary-foreground" href="/store/cart">Lihat cart</a>
      </div>
    </header>

    {#if loading}
      <div class="mt-6"><StateNotice tone="neutral" title="Memuat katalog" message="Mencoba mengambil produk dari /api/storefront/products." /></div>
    {:else if usingFallback}
      <div class="mt-6"><StateNotice tone="warning" title="Menggunakan fallback statis" message={`${error} Layout tetap bisa divalidasi, tetapi produk di bawah bukan data produksi.`} /></div>
    {:else if products.length === 0}
      <div class="mt-6"><StateNotice tone="primary" title="Belum ada produk published" message="Katalog kosong. Admin perlu membuat brand dan produk published sebelum storefront menampilkan item nyata." /></div>
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
              <a class="rounded-xl border border-border bg-surface px-3 py-2 text-sm font-semibold text-primary" href="/store/cart">Tambah</a>
            </div>
          </div>
        </article>
      {/each}
    </section>
  </div>
</StoreShell>
