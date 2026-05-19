-- Migration 0007: Add admin and karyawan roles; add is_demo marker columns.
-- Additive only. Safe to run multiple times (idempotent via ON CONFLICT / IF NOT EXISTS).

-- 1. Insert new roles (idempotent).
INSERT INTO roles (code, name)
VALUES
  ('admin',    'Admin'),
  ('karyawan', 'Karyawan')
ON CONFLICT (code) DO NOTHING;

-- 2. Add is_demo flag to brands (additive, nullable with default false).
ALTER TABLE brands
  ADD COLUMN IF NOT EXISTS is_demo boolean NOT NULL DEFAULT false;

-- 3. Add is_demo flag to products (additive, nullable with default false).
ALTER TABLE products
  ADD COLUMN IF NOT EXISTS is_demo boolean NOT NULL DEFAULT false;

-- 4. Add is_demo flag to orders (additive, nullable with default false).
ALTER TABLE orders
  ADD COLUMN IF NOT EXISTS is_demo boolean NOT NULL DEFAULT false;

-- 5. Indexes for efficient demo data cleanup.
CREATE INDEX IF NOT EXISTS idx_brands_is_demo   ON brands(is_demo)   WHERE is_demo = true;
CREATE INDEX IF NOT EXISTS idx_products_is_demo ON products(is_demo) WHERE is_demo = true;
CREATE INDEX IF NOT EXISTS idx_orders_is_demo   ON orders(is_demo)   WHERE is_demo = true;
