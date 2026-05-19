use sqlx::{PgPool, Row};

#[derive(Debug, thiserror::Error)]
pub enum DemoSeedError {
    #[error("database error during demo seed: {0}")]
    Database(#[from] sqlx::Error),
}

/// Seed idempotent clothing demo data.
/// Safe to call multiple times — uses ON CONFLICT DO NOTHING / DO UPDATE.
/// All rows are flagged with `is_demo = true` for clean removal.
pub async fn seed_demo_clothing(pool: &PgPool) -> Result<(), DemoSeedError> {
    let mut tx = pool.begin().await?;

    // ── 1. Brands ────────────────────────────────────────────────────────────
    sqlx::query(
        r#"
        INSERT INTO brands (name, slug, description, status, is_demo)
        VALUES
          ('Batik Nusantara',  'batik-nusantara',  'Koleksi batik premium dari seluruh nusantara.',  'active', true),
          ('Urban Threads',    'urban-threads',    'Streetwear modern untuk generasi muda.',          'active', true),
          ('Modest Wear ID',   'modest-wear-id',   'Busana muslim kontemporer berkualitas tinggi.',   'active', true)
        ON CONFLICT (slug) DO UPDATE
          SET name = EXCLUDED.name,
              description = EXCLUDED.description,
              status = EXCLUDED.status,
              is_demo = true
        "#,
    )
    .execute(&mut *tx)
    .await?;

    // ── 2. Demo seller users (one per brand) ─────────────────────────────────
    // Password hash is a fixed argon2id placeholder — demo accounts only.
    // Real deployments should rotate these via admin panel.
    let demo_hash = "$argon2id$v=19$m=19456,t=2,p=1$ZGVtb3NhbHQxMjM0NTY$demohashdemohashdemohashdemohashdemohashdemo";

    for (name, email) in [
        ("Demo Seller Batik",  "demo-seller-batik@demo.local"),
        ("Demo Seller Urban",  "demo-seller-urban@demo.local"),
        ("Demo Seller Modest", "demo-seller-modest@demo.local"),
    ] {
        sqlx::query(
            r#"
            INSERT INTO users (role_id, name, email, password_hash, status)
            SELECT id, $1, $2, $3, 'active'
            FROM roles WHERE code = 'seller'
            ON CONFLICT (email) DO NOTHING
            "#,
        )
        .bind(name)
        .bind(email)
        .bind(demo_hash)
        .execute(&mut *tx)
        .await?;
    }

    // ── 3. Brand members (seller → brand) ────────────────────────────────────
    for (email, slug) in [
        ("demo-seller-batik@demo.local",  "batik-nusantara"),
        ("demo-seller-urban@demo.local",  "urban-threads"),
        ("demo-seller-modest@demo.local", "modest-wear-id"),
    ] {
        sqlx::query(
            r#"
            INSERT INTO brand_members (brand_id, user_id, member_role)
            SELECT b.id, u.id, 'seller'
            FROM brands b, users u
            WHERE b.slug = $1 AND u.email = $2
            ON CONFLICT (user_id, brand_id) DO NOTHING
            "#,
        )
        .bind(slug)
        .bind(email)
        .execute(&mut *tx)
        .await?;
    }

    // ── 4. Products + variants ────────────────────────────────────────────────
    // Each brand gets 4 products with 1 default variant each.
    // Slugs are brand-scoped so they won't conflict across brands.
    let products: &[(&str, &str, &str, &str, &str, &str, i32)] = &[
        // (brand_slug, product_slug, product_name, description, variant_name, price, stock)
        ("batik-nusantara", "batik-parang-klasik",   "Batik Parang Klasik",   "Motif parang klasik dari Solo.",          "M / Coklat",  "285000.00", 50),
        ("batik-nusantara", "batik-mega-mendung",    "Batik Mega Mendung",    "Motif mega mendung khas Cirebon.",        "L / Biru",    "310000.00", 40),
        ("batik-nusantara", "batik-kawung-premium",  "Batik Kawung Premium",  "Batik kawung dengan bahan premium.",      "XL / Hitam",  "350000.00", 30),
        ("batik-nusantara", "batik-truntum-modern",  "Batik Truntum Modern",  "Reinterpretasi modern motif truntum.",    "M / Merah",   "295000.00", 45),
        ("urban-threads",   "kaos-oversized-basic",  "Kaos Oversized Basic",  "Kaos oversized 100% cotton combed 30s.", "M / Putih",   "185000.00", 100),
        ("urban-threads",   "hoodie-fleece-urban",   "Hoodie Fleece Urban",   "Hoodie fleece tebal anti dingin.",        "L / Abu",     "420000.00", 60),
        ("urban-threads",   "celana-cargo-street",   "Celana Cargo Street",   "Celana cargo 6 kantong streetwear.",      "32 / Hitam",  "375000.00", 55),
        ("urban-threads",   "jaket-bomber-urban",    "Jaket Bomber Urban",    "Jaket bomber dengan detail bordir.",      "L / Olive",   "495000.00", 35),
        ("modest-wear-id",  "gamis-daily-modest",    "Gamis Daily Modest",    "Gamis harian bahan rayon premium.",       "M / Navy",    "320000.00", 70),
        ("modest-wear-id",  "tunik-batik-modest",    "Tunik Batik Modest",    "Tunik batik kombinasi polos.",            "L / Krem",    "265000.00", 65),
        ("modest-wear-id",  "hijab-segi-empat",      "Hijab Segi Empat",      "Hijab segi empat bahan voal premium.",   "All Size",    "95000.00",  150),
        ("modest-wear-id",  "rok-plisket-modest",    "Rok Plisket Modest",    "Rok plisket panjang anti kusut.",         "M / Hitam",   "215000.00", 80),
    ];

    for (brand_slug, prod_slug, prod_name, description, variant_name, price, stock) in products {
        // Insert product (idempotent by brand_id + slug unique constraint).
        sqlx::query(
            r#"
            INSERT INTO products (brand_id, name, slug, description, status, is_demo)
            SELECT b.id, $2, $3, $4, 'published', true
            FROM brands b WHERE b.slug = $1
            ON CONFLICT (brand_id, slug) DO UPDATE
              SET name = EXCLUDED.name,
                  description = EXCLUDED.description,
                  status = EXCLUDED.status,
                  is_demo = true
            "#,
        )
        .bind(brand_slug)
        .bind(prod_name)
        .bind(prod_slug)
        .bind(description)
        .execute(&mut *tx)
        .await?;

        // Insert default variant (idempotent by product_id + sku unique constraint).
        // SKU is derived from slug so it's stable across re-seeds.
        let sku = format!("DEMO-{}", prod_slug.to_ascii_uppercase().replace('-', "_"));
        sqlx::query(
            r#"
            INSERT INTO product_variants (product_id, sku, name, attributes, price, stock, status)
            SELECT p.id, $2, $3, $4::jsonb, $5::numeric, $6, 'active'
            FROM products p
            JOIN brands b ON b.id = p.brand_id
            WHERE b.slug = $7 AND p.slug = $1
            ON CONFLICT (product_id, sku) DO UPDATE
              SET name   = EXCLUDED.name,
                  price  = EXCLUDED.price,
                  stock  = EXCLUDED.stock,
                  status = EXCLUDED.status
            "#,
        )
        .bind(prod_slug)
        .bind(&sku)
        .bind(variant_name)
        .bind(serde_json::json!({"size": variant_name.split(" / ").next().unwrap_or("OS"), "color": variant_name.split(" / ").nth(1).unwrap_or("")}).to_string())
        .bind(price)
        .bind(stock)
        .bind(brand_slug)
        .execute(&mut *tx)
        .await?;
    }

    // ── 5. Demo orders (2 sample orders) ─────────────────────────────────────
    // Order 1: single item from batik-nusantara
    seed_demo_order(
        &mut tx,
        "Demo Customer Satu",
        "08111000001",
        "Jl. Merdeka No. 1, Jakarta Pusat",
        "batik-nusantara",
        "batik-parang-klasik",
        2,
    )
    .await?;

    // Order 2: single item from urban-threads
    seed_demo_order(
        &mut tx,
        "Demo Customer Dua",
        "08111000002",
        "Jl. Sudirman No. 99, Bandung",
        "urban-threads",
        "kaos-oversized-basic",
        1,
    )
    .await?;

    tx.commit().await?;
    Ok(())
}

/// Remove all demo data (rows with is_demo = true) in dependency order.
pub async fn clear_demo_clothing(pool: &PgPool) -> Result<(), DemoSeedError> {
    let mut tx = pool.begin().await?;

    // Remove demo orders and their children first.
    sqlx::query(
        r#"
        DELETE FROM payment_proofs
        WHERE order_id IN (SELECT id FROM orders WHERE is_demo = true)
        "#,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        DELETE FROM order_status_history
        WHERE order_id IN (SELECT id FROM orders WHERE is_demo = true)
        "#,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        DELETE FROM order_items
        WHERE order_id IN (SELECT id FROM orders WHERE is_demo = true)
        "#,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        DELETE FROM order_brand_groups
        WHERE order_id IN (SELECT id FROM orders WHERE is_demo = true)
        "#,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query("DELETE FROM orders WHERE is_demo = true")
        .execute(&mut *tx)
        .await?;

    // Remove demo products and variants.
    sqlx::query(
        r#"
        DELETE FROM product_variants
        WHERE product_id IN (SELECT id FROM products WHERE is_demo = true)
        "#,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query("DELETE FROM products WHERE is_demo = true")
        .execute(&mut *tx)
        .await?;

    // Remove demo brand members and their permissions.
    sqlx::query(
        r#"
        DELETE FROM brand_member_permissions
        WHERE brand_member_id IN (
          SELECT bm.id FROM brand_members bm
          JOIN brands b ON b.id = bm.brand_id
          WHERE b.is_demo = true
        )
        "#,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        DELETE FROM brand_members
        WHERE brand_id IN (SELECT id FROM brands WHERE is_demo = true)
        "#,
    )
    .execute(&mut *tx)
    .await?;

    // Remove demo brands.
    sqlx::query("DELETE FROM brands WHERE is_demo = true")
        .execute(&mut *tx)
        .await?;

    // Remove demo seller users — only those with seller role and demo email domain.
    // Super Admin accounts (even with @demo.local email) are NOT removed here.
    sqlx::query(
        r#"
        DELETE FROM sessions
        WHERE user_id IN (
          SELECT u.id FROM users u
          JOIN roles r ON r.id = u.role_id
          WHERE u.email LIKE '%@demo.local'
            AND r.code = 'seller'
        )
        "#,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        DELETE FROM users
        WHERE email LIKE '%@demo.local'
          AND role_id = (SELECT id FROM roles WHERE code = 'seller')
        "#,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

/// Insert a single demo order with one item, idempotent by order_number derived from inputs.
async fn seed_demo_order(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    customer_name: &str,
    customer_contact: &str,
    shipping_address: &str,
    brand_slug: &str,
    product_slug: &str,
    quantity: i32,
) -> Result<(), sqlx::Error> {
    // Derive a stable order number from the customer name so re-seeding is idempotent.
    let order_number = format!(
        "DEMO-{}",
        customer_name
            .chars()
            .filter(|c| c.is_alphanumeric())
            .take(8)
            .collect::<String>()
            .to_ascii_uppercase()
    );

    // Skip if this demo order already exists.
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM orders WHERE order_number = $1)",
    )
    .bind(&order_number)
    .fetch_one(&mut **tx)
    .await?;

    if exists {
        return Ok(());
    }

    // Fetch variant price and id.
    let row = sqlx::query(
        r#"
        SELECT v.id AS variant_id, v.price::text AS price, b.id AS brand_id,
               p.name AS product_name, v.name AS variant_name, b.name AS brand_name,
               v.sku
        FROM product_variants v
        JOIN products p ON p.id = v.product_id
        JOIN brands b ON b.id = p.brand_id
        WHERE b.slug = $1 AND p.slug = $2
        LIMIT 1
        "#,
    )
    .bind(brand_slug)
    .bind(product_slug)
    .fetch_optional(&mut **tx)
    .await?;

    let Some(row) = row else {
        // Product not seeded yet — skip order silently.
        return Ok(());
    };

    let variant_id: uuid::Uuid = row.get("variant_id");
    let price: String = row.get("price");
    let brand_id: uuid::Uuid = row.get("brand_id");
    let product_name: String = row.get("product_name");
    let variant_name: String = row.get("variant_name");
    let brand_name: String = row.get("brand_name");
    let sku: Option<String> = row.get("sku");

    // Compute line total.
    let unit_cents = price
        .split('.')
        .next()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0)
        * 100;
    let line_cents = unit_cents * quantity as i64;
    let line_total = format!("{}.{:02}", line_cents / 100, line_cents % 100);
    let total = line_total.clone();

    let tracking_token = uuid::Uuid::new_v4().to_string();

    let order_id: uuid::Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO orders (
          order_number, public_tracking_token, customer_name, customer_contact,
          shipping_address, payment_status, global_status,
          subtotal_snapshot, total_snapshot, is_demo
        )
        VALUES ($1, $2, $3, $4, $5, 'pending', 'open', $6::numeric, $6::numeric, true)
        RETURNING id
        "#,
    )
    .bind(&order_number)
    .bind(&tracking_token)
    .bind(customer_name)
    .bind(customer_contact)
    .bind(shipping_address)
    .bind(&total)
    .fetch_one(&mut **tx)
    .await?;

    let group_id: uuid::Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO order_brand_groups (order_id, brand_id, fulfillment_status, subtotal_snapshot)
        VALUES ($1, $2, 'not_ready', $3::numeric)
        RETURNING id
        "#,
    )
    .bind(order_id)
    .bind(brand_id)
    .bind(&total)
    .fetch_one(&mut **tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO order_items (
          order_id, order_brand_group_id, brand_id, product_id, product_variant_id,
          product_name_snapshot, variant_name_snapshot, brand_name_snapshot, sku_snapshot,
          unit_price_snapshot, quantity, line_total_snapshot
        )
        SELECT $1, $2, b.id, p.id, v.id,
               $3, $4, $5, $6,
               $7::numeric, $8, $9::numeric
        FROM product_variants v
        JOIN products p ON p.id = v.product_id
        JOIN brands b ON b.id = p.brand_id
        WHERE v.id = $10
        "#,
    )
    .bind(order_id)
    .bind(group_id)
    .bind(&product_name)
    .bind(&variant_name)
    .bind(&brand_name)
    .bind(&sku)
    .bind(&price)
    .bind(quantity)
    .bind(&line_total)
    .bind(variant_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_seed_error_displays_database_source() {
        // Verify the error type wraps sqlx::Error correctly.
        let err = DemoSeedError::Database(sqlx::Error::RowNotFound);
        assert!(err.to_string().contains("database error during demo seed"));
    }
}
