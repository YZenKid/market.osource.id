import { browser } from '$app/environment';
import { writable } from 'svelte/store';
import { apiUrl } from '$lib/api/base';

export type CartItem = {
  productId: string;
  variantId: string;
  name: string;
  slug: string;
  description?: string | null;
  brandName?: string | null;
  price: string;
  stock: number;
  quantity: number;
};

// Shape returned by the backend cart endpoints.
type ApiCartItem = {
  product_id: string;
  variant_id: string;
  name: string;
  slug: string;
  description?: string | null;
  brand_name?: string | null;
  price: string;
  stock: number;
  quantity: number;
};

function toCartItem(item: ApiCartItem): CartItem {
  return {
    productId: item.product_id,
    variantId: item.variant_id,
    name: item.name,
    slug: item.slug,
    description: item.description,
    brandName: item.brand_name,
    price: item.price,
    stock: item.stock,
    quantity: item.quantity
  };
}

async function fetchCart(): Promise<CartItem[]> {
  try {
    const res = await fetch(apiUrl('/api/storefront/cart'), { credentials: 'include' });
    if (!res.ok) return [];
    const payload = await res.json().catch(() => ({ items: [] }));
    const items: ApiCartItem[] = Array.isArray(payload.items) ? payload.items : [];
    return items.map(toCartItem);
  } catch {
    return [];
  }
}

function createCartStore() {
  const { subscribe, set } = writable<CartItem[]>([]);

  return {
    subscribe,

    /** Load cart from backend. Safe to call on mount; no-ops outside browser. */
    async hydrate() {
      if (!browser) return;
      const items = await fetchCart();
      set(items);
    },

    /** Add or increment a variant in the backend cart, then sync state. */
    async add(item: CartItem) {
      if (!browser) return;
      try {
        let nextQuantity = item.quantity;
        const unsubscribe = subscribe((items) => {
          const existing = items.find((entry) => entry.variantId === item.variantId);
          nextQuantity = Math.min(
            (existing?.quantity ?? 0) + item.quantity,
            Math.max(item.stock, item.quantity, 1)
          );
        });
        unsubscribe();

        const res = await fetch(apiUrl('/api/storefront/cart'), {
          method: 'POST',
          headers: { 'content-type': 'application/json' },
          credentials: 'include',
          body: JSON.stringify({
            product_id: item.productId,
            variant_id: item.variantId,
            quantity: nextQuantity
          })
        });
        if (!res.ok) return;
        const payload = await res.json().catch(() => ({ items: [] }));
        const items: ApiCartItem[] = Array.isArray(payload.items) ? payload.items : [];
        set(items.map(toCartItem));
      } catch {
        // silently ignore network errors; UI can show stale state
      }
    },

    /** Set absolute quantity for a variant. quantity <= 0 removes the item. */
    async setQuantity(variantId: string, quantity: number) {
      if (!browser) return;
      if (quantity <= 0) {
        await this.remove(variantId);
        return;
      }
      try {
        // Re-use the upsert endpoint: POST with the new absolute quantity.
        // We need product_id too — read it from current store state.
        let productId = '';
        const unsub = subscribe((items) => {
          productId = items.find((i) => i.variantId === variantId)?.productId ?? '';
        });
        unsub();

        if (!productId) return;

        const res = await fetch(apiUrl('/api/storefront/cart'), {
          method: 'POST',
          headers: { 'content-type': 'application/json' },
          credentials: 'include',
          body: JSON.stringify({
            product_id: productId,
            variant_id: variantId,
            quantity
          })
        });
        if (!res.ok) return;
        const payload = await res.json().catch(() => ({ items: [] }));
        const items: ApiCartItem[] = Array.isArray(payload.items) ? payload.items : [];
        set(items.map(toCartItem));
      } catch {
        // silently ignore
      }
    },

    /** Remove a single variant from the backend cart. */
    async remove(variantId: string) {
      if (!browser) return;
      try {
        const res = await fetch(apiUrl(`/api/storefront/cart/items/${variantId}`), {
          method: 'DELETE',
          credentials: 'include'
        });
        if (!res.ok) return;
        const payload = await res.json().catch(() => ({ items: [] }));
        const items: ApiCartItem[] = Array.isArray(payload.items) ? payload.items : [];
        set(items.map(toCartItem));
      } catch {
        // silently ignore
      }
    },

    /**
     * Clear local store state immediately (called after successful checkout).
     * Does NOT call the backend — checkout already consumed the items.
     * If you need to also wipe the backend cart (e.g. abandoned cart), call
     * remove() for each item or add a dedicated clear endpoint later.
     */
    clear() {
      set([]);
    }
  };
}

export const cart = createCartStore();
