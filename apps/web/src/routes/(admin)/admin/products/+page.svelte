<script lang="ts">
  import { onMount } from 'svelte';
  import { apiUrl } from '$lib/api/base';
  import AdminShell from '$lib/ui/AdminShell.svelte';
  import StateNotice from '$lib/ui/StateNotice.svelte';
  import StatusBadge from '$lib/ui/StatusBadge.svelte';

  type Product = {
    id: string;
    name: string;
    slug: string;
    status?: string | null;
    brand_name?: string | null;
    variants?: Array<{ price?: string; stock?: number }>;
  };

  type Brand = { id: string; name: string; slug: string; status?: string | null };

  let products: Product[] = [];
  let brands: Brand[] = [];

  let loading = true;
  let permission = false;
  let error = '';
  let success = '';
  let creating = false;

  let brandId = '';
  let name = '';
  let slug = '';
  let description = '';
  let status = 'published';
  let variantName = 'Default';
  let variantPrice = '0';
  let variantStock = 0;

  async function loadProducts() {
    loading = true;
    permission = false;
    error = '';
    try {
      const [productsResponse, brandsResponse] = await Promise.all([
        fetch(apiUrl('/api/admin/products'), { credentials: 'include' }),
        fetch(apiUrl('/api/admin/brands'), { credentials: 'include' })
      ]);
      if (productsResponse.status === 401 || productsResponse.status === 403) {
        permission = true;
        return;
      }
      if (!productsResponse.ok) throw new Error('Product API belum tersedia.');

      const productsPayload = await productsResponse.json();
      products = Array.isArray(productsPayload.products) ? productsPayload.products : [];

      if (brandsResponse.ok) {
        const brandsPayload = await brandsResponse.json();
        brands = Array.isArray(brandsPayload.brands) ? brandsPayload.brands : [];
        if (!brandId && brands.length > 0) brandId = brands[0].id;
      }
    } catch (loadError) {
      error = loadError instanceof Error ? loadError.message : 'Gagal memuat produk.';
    } finally {
      loading = false;
    }
  }

  async function fetchCsrfToken() {
    const response = await fetch(apiUrl('/api/auth/csrf'), { credentials: 'include' });
    const payload = await response.json().catch(() => ({}));
    if (!response.ok || typeof payload.token !== 'string') {
      throw new Error(payload.message ?? 'Gagal menyiapkan token keamanan.');
    }
    return payload.token as string;
  }

  async function createProduct() {
    creating = true;
    error = '';
    success = '';
    try {
      const csrfToken = await fetchCsrfToken();
      const response = await fetch(apiUrl('/api/admin/products'), {
        method: 'POST',
        credentials: 'include',
        headers: {
          'content-type': 'application/json',
          'x-csrf-token': csrfToken
        },
        body: JSON.stringify({
          brand_id: brandId,
          category_id: null,
          name,
          slug,
          description: description || null,
          status,
          default_variant: {
            sku: null,
            name: variantName || 'Default',
            attributes: { internal: true },
            price: variantPrice,
            stock: Number(variantStock),
            status: 'active'
          }
        })
      });
      const payload = await response.json().catch(() => ({}));
      if (!response.ok) throw new Error(payload.message ?? 'Gagal membuat produk.');
      success = `Produk ${payload.name ?? name} berhasil dibuat.`;
      name = '';
      slug = '';
      description = '';
      variantPrice = '0';
      variantStock = 0;
      await loadProducts();
    } catch (submitError) {
      error = submitError instanceof Error ? submitError.message : 'Gagal membuat produk.';
    } finally {
      creating = false;
    }
  }

  const priceOf = (product: Product) => Number(product.variants?.[0]?.price ?? 0).toLocaleString('id-ID');

  $: slug = name
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9\s-]/g, '')
    .replace(/\s+/g, '-')
    .replace(/-+/g, '-');

  onMount(loadProducts);
</script>

<svelte:head><title>Products — Admin</title></svelte:head>

<AdminShell active="products" title="Products" description="Super Admin dapat membuat produk terhadap brand nyata beserta default variant price/stock dari backend.">
  <a slot="header-action" class="inline-flex min-h-11 items-center rounded-xl border border-border bg-surface px-4 py-2.5 text-sm font-semibold" href="/admin/brands">Kelola brands</a>

  {#if loading}
    <StateNotice tone="neutral" title="Memuat produk" message="Mengambil katalog admin dari endpoint terproteksi." />
  {:else if permission}
    <StateNotice tone="warning" title="Butuh izin admin" message="Daftar produk admin memerlukan sesi seller/super admin. Untuk create produk milestone 1 gunakan sesi Super Admin." actionHref="/admin/login" actionLabel="Buka admin login" />
  {:else}
    {#if error}
      <StateNotice tone="destructive" title="Produk belum dapat diproses" message={error} />
    {/if}
    {#if success}
      <StateNotice tone="success" title="Produk tersimpan" message={success} />
    {/if}

    <section class="panel p-5" aria-label="Create product form">
      <div class="mb-4 flex items-center justify-between gap-3">
        <h2 class="text-lg font-semibold">Buat produk</h2>
        <StatusBadge tone="primary" label="POST /api/admin/products" />
      </div>

      {#if brands.length === 0}
        <StateNotice tone="warning" title="Belum ada brand" message="Produk hanya bisa dibuat terhadap brand nyata. Buat brand dulu dari halaman Brands." actionHref="/admin/brands" actionLabel="Buka brands" />
      {:else}
        <form class="grid gap-4 md:grid-cols-2" on:submit|preventDefault={createProduct}>
          <label class="block text-sm font-medium md:col-span-2">
            Brand
            <select class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={brandId} required>
              {#each brands as brand}
                <option value={brand.id}>{brand.name} ({brand.slug})</option>
              {/each}
            </select>
          </label>

          <label class="block text-sm font-medium">
            Nama produk
            <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={name} required />
          </label>

          <label class="block text-sm font-medium">
            Slug produk
            <input class="mt-2 w-full rounded-xl border border-border bg-muted px-3 py-3 text-muted-foreground" bind:value={slug} readonly />
          </label>

          <label class="block text-sm font-medium md:col-span-2">
            Deskripsi
            <textarea class="mt-2 min-h-24 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={description}></textarea>
          </label>

          <label class="block text-sm font-medium">
            Status
            <select class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={status}>
              <option value="published">published</option>
              <option value="draft">draft</option>
            </select>
          </label>

          <label class="block text-sm font-medium">
            Nama default variant
            <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={variantName} />
          </label>

          <label class="block text-sm font-medium">
            Harga variant
            <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={variantPrice} inputmode="numeric" required />
          </label>

          <label class="block text-sm font-medium">
            Stock variant
            <input class="mt-2 w-full rounded-xl border border-border bg-surface px-3 py-3" bind:value={variantStock} type="number" min="0" required />
          </label>

          <div class="md:col-span-2">
            <button class="min-h-11 rounded-xl bg-primary px-4 py-3 text-sm font-semibold text-primary-foreground disabled:cursor-not-allowed disabled:opacity-50" disabled={creating || brandId.length === 0 || name.trim().length === 0 || slug.trim().length === 0 || variantPrice.trim().length === 0}>
              {creating ? 'Menyimpan produk…' : 'Simpan produk'}
            </button>
          </div>
        </form>
      {/if}
    </section>

    {#if products.length === 0}
      <StateNotice tone="primary" title="Belum ada produk" message="Setelah brand tersedia, buat produk agar storefront menampilkan data nyata dari API." />
    {:else}
      <section class="panel overflow-hidden" aria-label="Product table">
        <div class="grid gap-3 border-b border-border bg-muted/40 p-4 text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground md:grid-cols-[1.4fr_1fr_0.7fr_0.7fr_0.8fr]">
          <span>Produk</span><span>Brand</span><span>Harga</span><span>Stock</span><span>Status</span>
        </div>
        {#each products as product}
          <article class="grid gap-3 border-b border-border p-4 last:border-b-0 md:grid-cols-[1.4fr_1fr_0.7fr_0.7fr_0.8fr] md:items-center">
            <div>
              <h2 class="font-semibold">{product.name}</h2>
              <p class="mt-1 font-mono text-xs text-muted-foreground">{product.slug}</p>
            </div>
            <p class="text-sm text-muted-foreground">{product.brand_name ?? 'Brand belum dimuat'}</p>
            <p class="text-sm font-semibold tabular-nums">Rp{priceOf(product)}</p>
            <p class="text-sm font-semibold tabular-nums">{product.variants?.[0]?.stock ?? '—'}</p>
            <StatusBadge tone={product.status === 'published' ? 'success' : 'neutral'} label={product.status ?? 'draft'} />
          </article>
        {/each}
      </section>
    {/if}
  {/if}
</AdminShell>
