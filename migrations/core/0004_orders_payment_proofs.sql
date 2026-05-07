CREATE TABLE IF NOT EXISTS carts (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  anonymous_token text UNIQUE,
  user_id uuid REFERENCES users(id),
  expires_at timestamptz,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS cart_items (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  cart_id uuid NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
  product_id uuid NOT NULL REFERENCES products(id),
  product_variant_id uuid NOT NULL REFERENCES product_variants(id),
  quantity integer NOT NULL CHECK (quantity > 0),
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now(),
  UNIQUE (cart_id, product_variant_id)
);

CREATE TABLE IF NOT EXISTS orders (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  order_number text NOT NULL UNIQUE,
  public_tracking_token text NOT NULL UNIQUE,
  customer_name text NOT NULL,
  customer_contact text NOT NULL,
  shipping_address text NOT NULL,
  payment_status text NOT NULL DEFAULT 'pending',
  global_status text NOT NULL DEFAULT 'open',
  subtotal_snapshot numeric(14,2) NOT NULL,
  total_snapshot numeric(14,2) NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS order_brand_groups (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  order_id uuid NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  brand_id uuid NOT NULL REFERENCES brands(id),
  fulfillment_status text NOT NULL DEFAULT 'not_ready',
  subtotal_snapshot numeric(14,2) NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now(),
  UNIQUE (order_id, brand_id)
);

CREATE TABLE IF NOT EXISTS order_items (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  order_id uuid NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  order_brand_group_id uuid NOT NULL REFERENCES order_brand_groups(id) ON DELETE CASCADE,
  brand_id uuid NOT NULL REFERENCES brands(id),
  product_id uuid REFERENCES products(id),
  product_variant_id uuid REFERENCES product_variants(id),
  product_name_snapshot text NOT NULL,
  variant_name_snapshot text,
  brand_name_snapshot text NOT NULL,
  sku_snapshot text,
  unit_price_snapshot numeric(14,2) NOT NULL,
  quantity integer NOT NULL CHECK (quantity > 0),
  line_total_snapshot numeric(14,2) NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS order_status_history (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  order_id uuid NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  order_brand_group_id uuid REFERENCES order_brand_groups(id),
  changed_by_user_id uuid REFERENCES users(id),
  status_type text NOT NULL,
  from_status text,
  to_status text NOT NULL,
  note text,
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS payment_proofs (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  order_id uuid NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  uploaded_by_user_id uuid REFERENCES users(id),
  file_object_id uuid NOT NULL REFERENCES file_objects(id),
  status text NOT NULL DEFAULT 'uploaded',
  note text,
  verified_by_user_id uuid REFERENCES users(id),
  verified_at timestamptz,
  rejected_by_user_id uuid REFERENCES users(id),
  rejected_at timestamptz,
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_orders_payment_status ON orders(payment_status);
CREATE INDEX IF NOT EXISTS idx_orders_global_status ON orders(global_status);
CREATE INDEX IF NOT EXISTS idx_order_brand_groups_brand_status ON order_brand_groups(brand_id, fulfillment_status);
CREATE INDEX IF NOT EXISTS idx_order_items_group_id ON order_items(order_brand_group_id);
CREATE INDEX IF NOT EXISTS idx_payment_proofs_order_id ON payment_proofs(order_id);
