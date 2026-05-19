-- Manual reset fallback for marketplace installation.
-- DANGER: destructive. Use only when app-level reset endpoint is unavailable.
-- Run against isolated intended database only.
--
-- Usage:
--   psql "$DATABASE_URL" -f scripts/ops/reset_install.sql

BEGIN;

DELETE FROM audit_events;
DELETE FROM payment_proofs;
DELETE FROM order_status_history;
DELETE FROM order_items;
DELETE FROM order_brand_groups;
DELETE FROM orders;
DELETE FROM cart_items;
DELETE FROM carts;
DELETE FROM product_images;
DELETE FROM product_variants;
DELETE FROM products;
DELETE FROM categories;
DELETE FROM brand_member_permissions;
DELETE FROM brand_members;
DELETE FROM brands;
DELETE FROM file_objects;
DELETE FROM sessions;
DELETE FROM users;
DELETE FROM tunnel_settings;
DELETE FROM storage_settings;
DELETE FROM marketplace_settings;
DELETE FROM installation_state;

COMMIT;
