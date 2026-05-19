-- Demo seed: Clothing company data for market.osource.id
-- Idempotent: safe to run multiple times.
-- All rows flagged with is_demo = true for clean removal.
-- Run AFTER installation is locked and migrations applied.
--
-- Usage:
--   psql "$DATABASE_URL" -f scripts/demo/seed_clothing.sql
--
-- Or via admin endpoint (super_admin only):
--   POST /api/admin/demo/seed  (with CSRF + session cookie)

BEGIN;

-- ── Roles guard ──────────────────────────────────────────────────────────────
-- Ensure admin and karyawan roles exist (migration 0007 should have done this).
INSERT INTO roles (code, name)
VALUES ('admin', 'Admin'), ('karyawan', 'Karyawan')
ON CONFLICT (code) DO NOTHING;

-- ── Brands ───────────────────────────────────────────────────────────────────
INSERT INTO brands (name, slug, description, status, is_demo)
VALUES
  ('Batik Nusantara', 'batik-nusantara', 'Koleksi batik premium dari seluruh nusantara.', 'active', true),
  ('Urban Threads',   'urban-threads',   'Streetwear modern untuk generasi muda.',         'active', true),
  ('Modest Wear ID',  'modest-wear-id',  'Busana muslim kontemporer berkualitas tinggi.',  'active', true)
ON CONFLICT (slug) DO UPDATE
  SET name = EXCLUDED.name,
      description = EXCLUDED.description,
      status = EXCLUDED.status,
      is_demo = true;

-- ── Demo seller users ─────────────────────────────────────────────────────────
-- Password hash is a fixed argon2id placeholder for demo only.
-- Rotate via admin panel before any real use.
DO $$
DECLARE
  seller_role_id uuid;
  demo_hash text := '$argon2id$v=19$m=19456,t=2,p=1$ZGVtb3NhbHQxMjM0NTY$demohashdemohashdemohashdemohashdemohashdemo';
BEGIN
  SELECT id INTO seller_role_id FROM roles WHERE code = 'seller';

  INSERT INTO users (role_id, name, email, password_hash, status)
  VALUES
    (seller_role_id, 'Demo Seller Batik',  'demo-seller-batik@demo.local',  demo_hash, 'active'),
    (seller_role_id, 'Demo Seller Urban',  'demo-seller-urban@demo.local',  demo_hash, 'active'),
    (seller_role_id, 'Demo Seller Modest', 'demo-seller-modest@demo.local', demo_hash, 'active')
  ON CONFLICT (email) DO NOTHING;
END $$;

-- ── Brand members ─────────────────────────────────────────────────────────────
INSERT INTO brand_members (brand_id, user_id, member_role)
SELECT b.id, u.id, 'seller'
FROM brands b, users u
WHERE (b.slug = 'batik-nusantara' AND u.email = 'demo-seller-batik@demo.local')
   OR (b.slug = 'urban-threads'   AND u.email = 'demo-seller-urban@demo.local')
   OR (b.slug = 'modest-wear-id'  AND u.email = 'demo-seller-modest@demo.local')
ON CONFLICT (user_id, brand_id) DO NOTHING;

-- ── Products ──────────────────────────────────────────────────────────────────
INSERT INTO products (brand_id, name, slug, description, status, is_demo)
SELECT b.id, p.name, p.slug, p.description, 'published', true
FROM brands b
JOIN (VALUES
  ('batik-nusantara', 'Batik Parang Klasik',  'batik-parang-klasik',  'Motif parang klasik dari Solo.'),
  ('batik-nusantara', 'Batik Mega Mendung',   'batik-mega-mendung',   'Motif mega mendung khas Cirebon.'),
  ('batik-nusantara', 'Batik Kawung Premium', 'batik-kawung-premium', 'Batik kawung dengan bahan premium.'),
  ('batik-nusantara', 'Batik Truntum Modern', 'batik-truntum-modern', 'Reinterpretasi modern motif truntum.'),
  ('urban-threads',   'Kaos Oversized Basic', 'kaos-oversized-basic', 'Kaos oversized 100% cotton combed 30s.'),
  ('urban-threads',   'Hoodie Fleece Urban',  'hoodie-fleece-urban',  'Hoodie fleece tebal anti dingin.'),
  ('urban-threads',   'Celana Cargo Street',  'celana-cargo-street',  'Celana cargo 6 kantong streetwear.'),
  ('urban-threads',   'Jaket Bomber Urban',   'jaket-bomber-urban',   'Jaket bomber dengan detail bordir.'),
  ('modest-wear-id',  'Gamis Daily Modest',   'gamis-daily-modest',   'Gamis harian bahan rayon premium.'),
  ('modest-wear-id',  'Tunik Batik Modest',   'tunik-batik-modest',   'Tunik batik kombinasi polos.'),
  ('modest-wear-id',  'Hijab Segi Empat',     'hijab-segi-empat',     'Hijab segi empat bahan voal premium.'),
  ('modest-wear-id',  'Rok Plisket Modest',   'rok-plisket-modest',   'Rok plisket panjang anti kusut.')
) AS p(brand_slug, name, slug, description) ON b.slug = p.brand_slug
ON CONFLICT (brand_id, slug) DO UPDATE
  SET name = EXCLUDED.name,
      description = EXCLUDED.description,
      status = EXCLUDED.status,
      is_demo = true;

-- ── Product variants ──────────────────────────────────────────────────────────
INSERT INTO product_variants (product_id, sku, name, attributes, price, stock, status)
SELECT pr.id, v.sku, v.variant_name, v.attributes::jsonb, v.price::numeric, v.stock, 'active'
FROM products pr
JOIN brands b ON b.id = pr.brand_id
JOIN (VALUES
  ('batik-nusantara', 'batik-parang-klasik',  'DEMO-BATIK_PARANG_KLASIK',  'M / Coklat',  '{"size":"M","color":"Coklat"}',  285000, 50),
  ('batik-nusantara', 'batik-mega-mendung',   'DEMO-BATIK_MEGA_MENDUNG',   'L / Biru',    '{"size":"L","color":"Biru"}',    310000, 40),
  ('batik-nusantara', 'batik-kawung-premium', 'DEMO-BATIK_KAWUNG_PREMIUM', 'XL / Hitam',  '{"size":"XL","color":"Hitam"}',  350000, 30),
  ('batik-nusantara', 'batik-truntum-modern', 'DEMO-BATIK_TRUNTUM_MODERN', 'M / Merah',   '{"size":"M","color":"Merah"}',   295000, 45),
  ('urban-threads',   'kaos-oversized-basic', 'DEMO-KAOS_OVERSIZED_BASIC', 'M / Putih',   '{"size":"M","color":"Putih"}',   185000, 100),
  ('urban-threads',   'hoodie-fleece-urban',  'DEMO-HOODIE_FLEECE_URBAN',  'L / Abu',     '{"size":"L","color":"Abu"}',     420000, 60),
  ('urban-threads',   'celana-cargo-street',  'DEMO-CELANA_CARGO_STREET',  '32 / Hitam',  '{"size":"32","color":"Hitam"}',  375000, 55),
  ('urban-threads',   'jaket-bomber-urban',   'DEMO-JAKET_BOMBER_URBAN',   'L / Olive',   '{"size":"L","color":"Olive"}',   495000, 35),
  ('modest-wear-id',  'gamis-daily-modest',   'DEMO-GAMIS_DAILY_MODEST',   'M / Navy',    '{"size":"M","color":"Navy"}',    320000, 70),
  ('modest-wear-id',  'tunik-batik-modest',   'DEMO-TUNIK_BATIK_MODEST',   'L / Krem',    '{"size":"L","color":"Krem"}',    265000, 65),
  ('modest-wear-id',  'hijab-segi-empat',     'DEMO-HIJAB_SEGI_EMPAT',     'All Size',    '{"size":"All Size","color":""}', 95000,  150),
  ('modest-wear-id',  'rok-plisket-modest',   'DEMO-ROK_PLISKET_MODEST',   'M / Hitam',   '{"size":"M","color":"Hitam"}',   215000, 80)
) AS v(brand_slug, prod_slug, sku, variant_name, attributes, price, stock)
  ON b.slug = v.brand_slug AND pr.slug = v.prod_slug
ON CONFLICT (product_id, sku) DO UPDATE
  SET name       = EXCLUDED.name,
      attributes = EXCLUDED.attributes,
      price      = EXCLUDED.price,
      stock      = EXCLUDED.stock,
      status     = EXCLUDED.status;

-- ── Demo orders (2 sample orders) ────────────────────────────────────────────
DO $$
DECLARE
  v_order_id   uuid;
  v_group_id   uuid;
  v_variant_id uuid;
  v_brand_id   uuid;
  v_product_id uuid;
  v_price      numeric;
BEGIN
  -- Order 1: Batik Parang Klasik x2
  IF NOT EXISTS (SELECT 1 FROM orders WHERE order_number = 'DEMO-CUSTSAT1') THEN
    SELECT v.id, v.price, b.id, p.id
      INTO v_variant_id, v_price, v_brand_id, v_product_id
    FROM product_variants v
    JOIN products p ON p.id = v.product_id
    JOIN brands b ON b.id = p.brand_id
    WHERE b.slug = 'batik-nusantara' AND p.slug = 'batik-parang-klasik'
    LIMIT 1;

    IF v_variant_id IS NOT NULL THEN
      INSERT INTO orders (order_number, public_tracking_token, customer_name, customer_contact,
                          shipping_address, payment_status, global_status,
                          subtotal_snapshot, total_snapshot, is_demo)
      VALUES ('DEMO-CUSTSAT1', gen_random_uuid()::text, 'Demo Customer Satu', '08111000001',
              'Jl. Merdeka No. 1, Jakarta Pusat', 'pending', 'open',
              (v_price * 2)::numeric, (v_price * 2)::numeric, true)
      RETURNING id INTO v_order_id;

      INSERT INTO order_brand_groups (order_id, brand_id, fulfillment_status, subtotal_snapshot)
      VALUES (v_order_id, v_brand_id, 'not_ready', (v_price * 2)::numeric)
      RETURNING id INTO v_group_id;

      INSERT INTO order_items (order_id, order_brand_group_id, brand_id, product_id, product_variant_id,
                               product_name_snapshot, variant_name_snapshot, brand_name_snapshot, sku_snapshot,
                               unit_price_snapshot, quantity, line_total_snapshot)
      SELECT v_order_id, v_group_id, v_brand_id, v_product_id, v_variant_id,
             p.name, pv.name, b.name, pv.sku,
             v_price, 2, (v_price * 2)
      FROM product_variants pv
      JOIN products p ON p.id = pv.product_id
      JOIN brands b ON b.id = p.brand_id
      WHERE pv.id = v_variant_id;
    END IF;
  END IF;

  -- Order 2: Kaos Oversized Basic x1
  IF NOT EXISTS (SELECT 1 FROM orders WHERE order_number = 'DEMO-CUSTDUA2') THEN
    SELECT v.id, v.price, b.id, p.id
      INTO v_variant_id, v_price, v_brand_id, v_product_id
    FROM product_variants v
    JOIN products p ON p.id = v.product_id
    JOIN brands b ON b.id = p.brand_id
    WHERE b.slug = 'urban-threads' AND p.slug = 'kaos-oversized-basic'
    LIMIT 1;

    IF v_variant_id IS NOT NULL THEN
      INSERT INTO orders (order_number, public_tracking_token, customer_name, customer_contact,
                          shipping_address, payment_status, global_status,
                          subtotal_snapshot, total_snapshot, is_demo)
      VALUES ('DEMO-CUSTDUA2', gen_random_uuid()::text, 'Demo Customer Dua', '08111000002',
              'Jl. Sudirman No. 99, Bandung', 'pending', 'open',
              v_price, v_price, true)
      RETURNING id INTO v_order_id;

      INSERT INTO order_brand_groups (order_id, brand_id, fulfillment_status, subtotal_snapshot)
      VALUES (v_order_id, v_brand_id, 'not_ready', v_price)
      RETURNING id INTO v_group_id;

      INSERT INTO order_items (order_id, order_brand_group_id, brand_id, product_id, product_variant_id,
                               product_name_snapshot, variant_name_snapshot, brand_name_snapshot, sku_snapshot,
                               unit_price_snapshot, quantity, line_total_snapshot)
      SELECT v_order_id, v_group_id, v_brand_id, v_product_id, v_variant_id,
             p.name, pv.name, b.name, pv.sku,
             v_price, 1, v_price
      FROM product_variants pv
      JOIN products p ON p.id = pv.product_id
      JOIN brands b ON b.id = p.brand_id
      WHERE pv.id = v_variant_id;
    END IF;
  END IF;
END $$;

COMMIT;
