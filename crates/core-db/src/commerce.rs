use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Row, Transaction};
use std::collections::BTreeMap;

pub const ORDER_PAYMENT_STATUS_PENDING: &str = "pending";
pub const ORDER_GLOBAL_STATUS_OPEN: &str = "open";
pub const ORDER_GROUP_STATUS_NOT_READY: &str = "not_ready";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BrandRecord {
    pub id: uuid::Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateBrandInput {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CategoryRecord {
    pub id: uuid::Uuid,
    pub brand_id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub name: String,
    pub slug: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct CreateCategoryInput {
    pub brand_id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub name: String,
    pub slug: String,
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProductRecord {
    pub id: uuid::Uuid,
    pub brand_id: uuid::Uuid,
    pub brand_name: Option<String>,
    pub category_id: Option<uuid::Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProductVariantRecord {
    pub id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub sku: Option<String>,
    pub name: String,
    pub attributes: serde_json::Value,
    /// Decimal text from PostgreSQL numeric. `product_variants.price` is canonical.
    pub price: String,
    pub stock: i32,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProductWithVariantsRecord {
    pub product: ProductRecord,
    pub variants: Vec<ProductVariantRecord>,
}

#[derive(Debug, Clone)]
pub struct CreateProductInput {
    pub brand_id: uuid::Uuid,
    pub category_id: Option<uuid::Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub default_variant: CreateVariantInput,
}

#[derive(Debug, Clone)]
pub struct CreateVariantInput {
    pub sku: Option<String>,
    pub name: String,
    pub attributes: serde_json::Value,
    /// Decimal string accepted by PostgreSQL as numeric.
    pub price: String,
    pub stock: i32,
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrandMemberRecord {
    pub id: uuid::Uuid,
    pub brand_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub member_role: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CartRecord {
    pub id: uuid::Uuid,
    pub anonymous_token: Option<String>,
    pub user_id: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CartItemRecord {
    pub id: uuid::Uuid,
    pub cart_id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub product_variant_id: uuid::Uuid,
    pub quantity: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CheckoutItemInput {
    pub variant_id: uuid::Uuid,
    pub quantity: i32,
}

#[derive(Debug, Clone)]
pub struct CheckoutInput {
    pub customer_name: String,
    pub customer_contact: String,
    pub shipping_address: String,
    pub items: Vec<CheckoutItemInput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckoutVariantSnapshot {
    pub product_id: uuid::Uuid,
    pub variant_id: uuid::Uuid,
    pub brand_id: uuid::Uuid,
    pub product_name: String,
    pub variant_name: Option<String>,
    pub brand_name: String,
    pub sku: Option<String>,
    pub unit_price: String,
    pub stock: i32,
    pub quantity: i32,
    pub line_total: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OrderRecord {
    pub id: uuid::Uuid,
    pub order_number: String,
    pub public_tracking_token: String,
    pub customer_name: String,
    pub customer_contact: String,
    pub shipping_address: String,
    pub payment_status: String,
    pub global_status: String,
    pub subtotal_snapshot: String,
    pub total_snapshot: String,
    pub groups: Vec<OrderBrandGroupRecord>,
    pub items: Vec<OrderItemRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StorefrontOrderSummaryRecord {
    pub order_number: String,
    pub payment_status: String,
    pub global_status: String,
    pub total_snapshot: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OrderBrandGroupRecord {
    pub id: uuid::Uuid,
    pub order_id: uuid::Uuid,
    pub brand_id: uuid::Uuid,
    pub fulfillment_status: String,
    pub subtotal_snapshot: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OrderItemRecord {
    pub id: uuid::Uuid,
    pub order_id: uuid::Uuid,
    pub order_brand_group_id: uuid::Uuid,
    pub brand_id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub product_variant_id: uuid::Uuid,
    pub product_name_snapshot: String,
    pub variant_name_snapshot: Option<String>,
    pub brand_name_snapshot: String,
    pub sku_snapshot: Option<String>,
    pub unit_price_snapshot: String,
    pub quantity: i32,
    pub line_total_snapshot: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AdminOrderRecord {
    pub id: uuid::Uuid,
    pub order_number: String,
    pub public_tracking_token: String,
    pub customer_name: String,
    pub payment_status: String,
    pub global_status: String,
    pub total_snapshot: String,
    pub created_at: DateTime<Utc>,
    pub groups: Vec<OrderBrandGroupRecord>,
    pub item_count: i64,
    pub payment_proof_statuses: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum CommerceRepositoryError {
    #[error("checkout requires at least one item")]
    EmptyCheckout,
    #[error("checkout item quantity must be greater than zero")]
    InvalidQuantity,
    #[error("product variant not found or inactive")]
    VariantNotFound,
    #[error("product variant is out of stock")]
    OutOfStock,
    #[error("requested quantity exceeds available stock")]
    Overstock,
    #[error("database error")]
    Database(#[from] sqlx::Error),
}

pub async fn update_order_brand_group_fulfillment(
    pool: &PgPool,
    group_id: uuid::Uuid,
    to_status: &str,
    changed_by_user_id: uuid::Uuid,
    note: Option<&str>,
) -> Result<OrderBrandGroupRecord, CommerceRepositoryError> {
    let mut tx = pool.begin().await?;
    let existing = sqlx::query(
        r#"
        SELECT id, order_id, brand_id, fulfillment_status, subtotal_snapshot::text AS subtotal_snapshot
        FROM order_brand_groups
        WHERE id = $1
        FOR UPDATE
        "#,
    )
    .bind(group_id)
    .fetch_one(&mut *tx)
    .await?;

    let from_status: String = existing.get("fulfillment_status");
    let order_id: uuid::Uuid = existing.get("order_id");

    let updated = sqlx::query(
        r#"
        UPDATE order_brand_groups
        SET fulfillment_status = $2, updated_at = now()
        WHERE id = $1
        RETURNING id, order_id, brand_id, fulfillment_status, subtotal_snapshot::text AS subtotal_snapshot
        "#,
    )
    .bind(group_id)
    .bind(to_status)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO order_status_history (order_id, order_brand_group_id, changed_by_user_id, status_type, from_status, to_status, note)
        VALUES ($1, $2, $3, 'fulfillment', $4, $5, $6)
        "#,
    )
    .bind(order_id)
    .bind(group_id)
    .bind(changed_by_user_id)
    .bind(from_status)
    .bind(to_status)
    .bind(note)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(order_group_from_row(&updated))
}

pub async fn seller_can_access_order_brand_group(
    pool: &PgPool,
    group_id: uuid::Uuid,
    user_id: uuid::Uuid,
) -> Result<bool, CommerceRepositoryError> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
          SELECT 1
          FROM order_brand_groups obg
          JOIN brand_members bm ON bm.brand_id = obg.brand_id
          WHERE obg.id = $1 AND bm.user_id = $2
        )
        "#,
    )
    .bind(group_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn create_brand(
    pool: &PgPool,
    input: CreateBrandInput,
) -> Result<BrandRecord, CommerceRepositoryError> {
    let row = sqlx::query(
        r#"
        INSERT INTO brands (name, slug, description, status)
        VALUES ($1, $2, $3, COALESCE($4, 'active'))
        RETURNING id, name, slug, description, status, created_at, updated_at
        "#,
    )
    .bind(input.name)
    .bind(input.slug)
    .bind(input.description)
    .bind(input.status)
    .fetch_one(pool)
    .await?;

    Ok(brand_from_row(&row))
}

pub async fn list_brands(pool: &PgPool) -> Result<Vec<BrandRecord>, CommerceRepositoryError> {
    let rows = sqlx::query(
        r#"
        SELECT id, name, slug, description, status, created_at, updated_at
        FROM brands
        ORDER BY created_at DESC, name ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.iter().map(brand_from_row).collect())
}

pub async fn list_brands_for_user(
    pool: &PgPool,
    user_id: uuid::Uuid,
) -> Result<Vec<BrandRecord>, CommerceRepositoryError> {
    let rows = sqlx::query(
        r#"
        SELECT b.id, b.name, b.slug, b.description, b.status, b.created_at, b.updated_at
        FROM brands b
        JOIN brand_members bm ON bm.brand_id = b.id
        WHERE bm.user_id = $1
        ORDER BY b.created_at DESC, b.name ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.iter().map(brand_from_row).collect())
}

pub async fn create_category(
    pool: &PgPool,
    input: CreateCategoryInput,
) -> Result<CategoryRecord, CommerceRepositoryError> {
    let row = sqlx::query(
        r#"
        INSERT INTO categories (brand_id, parent_id, name, slug, status)
        VALUES ($1, $2, $3, $4, COALESCE($5, 'active'))
        RETURNING id, brand_id, parent_id, name, slug, status
        "#,
    )
    .bind(input.brand_id)
    .bind(input.parent_id)
    .bind(input.name)
    .bind(input.slug)
    .bind(input.status)
    .fetch_one(pool)
    .await?;

    Ok(category_from_row(&row))
}

pub async fn list_categories(
    pool: &PgPool,
    brand_id: Option<uuid::Uuid>,
) -> Result<Vec<CategoryRecord>, CommerceRepositoryError> {
    let rows = sqlx::query(
        r#"
        SELECT id, brand_id, parent_id, name, slug, status
        FROM categories
        WHERE ($1::uuid IS NULL OR brand_id = $1)
        ORDER BY name ASC
        "#,
    )
    .bind(brand_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.iter().map(category_from_row).collect())
}

pub async fn create_product_with_default_variant(
    pool: &PgPool,
    input: CreateProductInput,
) -> Result<ProductWithVariantsRecord, CommerceRepositoryError> {
    let mut tx = pool.begin().await?;
    let product = insert_product_in_tx(&mut tx, &input).await?;
    let variant = insert_variant_in_tx(&mut tx, product.id, &input.default_variant).await?;
    tx.commit().await?;
    Ok(ProductWithVariantsRecord {
        product,
        variants: vec![variant],
    })
}

pub async fn list_admin_products(
    pool: &PgPool,
) -> Result<Vec<ProductWithVariantsRecord>, CommerceRepositoryError> {
    products_from_rows(fetch_product_rows(pool, false, None).await?)
}

pub async fn list_admin_products_for_user(
    pool: &PgPool,
    user_id: uuid::Uuid,
) -> Result<Vec<ProductWithVariantsRecord>, CommerceRepositoryError> {
    products_from_rows(fetch_product_rows_for_user(pool, false, None, user_id).await?)
}

pub async fn list_storefront_products(
    pool: &PgPool,
) -> Result<Vec<ProductWithVariantsRecord>, CommerceRepositoryError> {
    products_from_rows(fetch_product_rows(pool, true, None).await?)
}

pub async fn storefront_product_detail(
    pool: &PgPool,
    product_id: uuid::Uuid,
) -> Result<Option<ProductWithVariantsRecord>, CommerceRepositoryError> {
    Ok(
        products_from_rows(fetch_product_rows(pool, true, Some(product_id)).await?)?
            .into_iter()
            .next(),
    )
}

pub async fn create_brand_member(
    pool: &PgPool,
    brand_id: uuid::Uuid,
    user_id: uuid::Uuid,
    member_role: Option<String>,
) -> Result<BrandMemberRecord, CommerceRepositoryError> {
    let row = sqlx::query(
        r#"
        INSERT INTO brand_members (brand_id, user_id, member_role)
        VALUES ($1, $2, COALESCE($3, 'seller'))
        ON CONFLICT (user_id, brand_id) DO UPDATE SET member_role = EXCLUDED.member_role
        RETURNING id, brand_id, user_id, member_role
        "#,
    )
    .bind(brand_id)
    .bind(user_id)
    .bind(member_role)
    .fetch_one(pool)
    .await?;

    Ok(BrandMemberRecord {
        id: row.get("id"),
        brand_id: row.get("brand_id"),
        user_id: row.get("user_id"),
        member_role: row.get("member_role"),
    })
}

pub async fn create_guest_cart(
    pool: &PgPool,
    anonymous_token: String,
) -> Result<CartRecord, CommerceRepositoryError> {
    let row = sqlx::query(
        r#"
        INSERT INTO carts (anonymous_token)
        VALUES ($1)
        ON CONFLICT (anonymous_token) DO UPDATE SET updated_at = now()
        RETURNING id, anonymous_token, user_id
        "#,
    )
    .bind(anonymous_token)
    .fetch_one(pool)
    .await?;

    Ok(CartRecord {
        id: row.get("id"),
        anonymous_token: row.get("anonymous_token"),
        user_id: row.get("user_id"),
    })
}

pub async fn upsert_guest_cart_item(
    pool: &PgPool,
    cart_id: uuid::Uuid,
    product_id: uuid::Uuid,
    product_variant_id: uuid::Uuid,
    quantity: i32,
) -> Result<CartItemRecord, CommerceRepositoryError> {
    if quantity <= 0 {
        return Err(CommerceRepositoryError::InvalidQuantity);
    }

    let row = sqlx::query(
        r#"
        INSERT INTO cart_items (cart_id, product_id, product_variant_id, quantity)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (cart_id, product_variant_id)
        DO UPDATE SET quantity = EXCLUDED.quantity, updated_at = now()
        RETURNING id, cart_id, product_id, product_variant_id, quantity
        "#,
    )
    .bind(cart_id)
    .bind(product_id)
    .bind(product_variant_id)
    .bind(quantity)
    .fetch_one(pool)
    .await?;

    Ok(CartItemRecord {
        id: row.get("id"),
        cart_id: row.get("cart_id"),
        product_id: row.get("product_id"),
        product_variant_id: row.get("product_variant_id"),
        quantity: row.get("quantity"),
    })
}

/// A cart item enriched with product/variant/brand data for the storefront UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StorefrontCartItem {
    pub product_id: uuid::Uuid,
    pub variant_id: uuid::Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub brand_name: Option<String>,
    pub price: String,
    pub stock: i32,
    pub quantity: i32,
}

/// Return all enriched cart items for a guest cart identified by `anonymous_token`.
/// Returns an empty vec if the cart does not exist yet.
pub async fn list_guest_cart_items(
    pool: &PgPool,
    anonymous_token: &str,
) -> Result<Vec<StorefrontCartItem>, CommerceRepositoryError> {
    let rows = sqlx::query(
        r#"
        SELECT
          p.id          AS product_id,
          v.id          AS variant_id,
          p.name        AS product_name,
          p.slug        AS slug,
          p.description AS description,
          b.name        AS brand_name,
          v.price::text AS price,
          v.stock       AS stock,
          ci.quantity   AS quantity
        FROM carts c
        JOIN cart_items ci ON ci.cart_id = c.id
        JOIN product_variants v ON v.id = ci.product_variant_id
        JOIN products p ON p.id = v.product_id
        JOIN brands b ON b.id = p.brand_id
        WHERE c.anonymous_token = $1
          AND v.status = 'active'
          AND p.status = 'published'
          AND b.status = 'active'
        ORDER BY ci.created_at ASC
        "#,
    )
    .bind(anonymous_token)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .iter()
        .map(|row| StorefrontCartItem {
            product_id: row.get("product_id"),
            variant_id: row.get("variant_id"),
            name: row.get("product_name"),
            slug: row.get("slug"),
            description: row.get("description"),
            brand_name: row.get("brand_name"),
            price: row.get("price"),
            stock: row.get("stock"),
            quantity: row.get("quantity"),
        })
        .collect())
}

/// Remove a single cart item by variant id for a guest cart.
/// No-ops silently if the cart or item does not exist.
pub async fn delete_guest_cart_item(
    pool: &PgPool,
    anonymous_token: &str,
    variant_id: uuid::Uuid,
) -> Result<(), CommerceRepositoryError> {
    sqlx::query(
        r#"
        DELETE FROM cart_items
        WHERE product_variant_id = $2
          AND cart_id = (
            SELECT id FROM carts WHERE anonymous_token = $1 LIMIT 1
          )
        "#,
    )
    .bind(anonymous_token)
    .bind(variant_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Remove all items from a guest cart. No-ops if the cart does not exist.
pub async fn clear_guest_cart(
    pool: &PgPool,
    anonymous_token: &str,
) -> Result<(), CommerceRepositoryError> {
    sqlx::query(
        r#"
        DELETE FROM cart_items
        WHERE cart_id = (
          SELECT id FROM carts WHERE anonymous_token = $1 LIMIT 1
        )
        "#,
    )
    .bind(anonymous_token)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn checkout_submitted_items(
    pool: &PgPool,
    input: CheckoutInput,
) -> Result<OrderRecord, CommerceRepositoryError> {
    validate_checkout_items(&input.items)?;

    let mut tx = pool.begin().await?;
    let snapshots = reserve_checkout_snapshots(&mut tx, &input.items).await?;
    let grouped = group_checkout_snapshots_by_brand(&snapshots);
    let subtotal = checkout_subtotal_sql(&snapshots);
    let order_number = generate_order_number();
    let public_tracking_token = uuid::Uuid::new_v4().to_string();

    let order_row = sqlx::query(
        r#"
        INSERT INTO orders (
          order_number, public_tracking_token, customer_name, customer_contact,
          shipping_address, payment_status, global_status, subtotal_snapshot, total_snapshot
        )
        VALUES ($1, $2, $3, $4, $5, 'pending', 'open', $6::numeric, $6::numeric)
        RETURNING id, order_number, public_tracking_token, customer_name, customer_contact,
                  shipping_address, payment_status, global_status,
                  subtotal_snapshot::text AS subtotal_snapshot, total_snapshot::text AS total_snapshot
        "#,
    )
    .bind(order_number)
    .bind(public_tracking_token)
    .bind(input.customer_name)
    .bind(input.customer_contact)
    .bind(input.shipping_address)
    .bind(subtotal)
    .fetch_one(&mut *tx)
    .await?;

    let order_id: uuid::Uuid = order_row.get("id");
    let mut groups = Vec::new();
    let mut group_ids = BTreeMap::new();
    for (brand_id, brand_subtotal) in grouped {
        let group_row = sqlx::query(
            r#"
            INSERT INTO order_brand_groups (order_id, brand_id, fulfillment_status, subtotal_snapshot)
            VALUES ($1, $2, 'not_ready', $3::numeric)
            RETURNING id, order_id, brand_id, fulfillment_status, subtotal_snapshot::text AS subtotal_snapshot
            "#,
        )
        .bind(order_id)
        .bind(brand_id)
        .bind(brand_subtotal)
        .fetch_one(&mut *tx)
        .await?;
        let group = order_group_from_row(&group_row);
        group_ids.insert(brand_id, group.id);
        groups.push(group);
    }

    let mut items = Vec::new();
    for snapshot in snapshots {
        let group_id = group_ids[&snapshot.brand_id];
        let item_row = sqlx::query(
            r#"
            INSERT INTO order_items (
              order_id, order_brand_group_id, brand_id, product_id, product_variant_id,
              product_name_snapshot, variant_name_snapshot, brand_name_snapshot, sku_snapshot,
              unit_price_snapshot, quantity, line_total_snapshot
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::numeric, $11, $12::numeric)
            RETURNING id, order_id, order_brand_group_id, brand_id, product_id, product_variant_id,
                      product_name_snapshot, variant_name_snapshot, brand_name_snapshot, sku_snapshot,
                      unit_price_snapshot::text AS unit_price_snapshot, quantity,
                      line_total_snapshot::text AS line_total_snapshot
            "#,
        )
        .bind(order_id)
        .bind(group_id)
        .bind(snapshot.brand_id)
        .bind(snapshot.product_id)
        .bind(snapshot.variant_id)
        .bind(snapshot.product_name)
        .bind(snapshot.variant_name)
        .bind(snapshot.brand_name)
        .bind(snapshot.sku)
        .bind(snapshot.unit_price)
        .bind(snapshot.quantity)
        .bind(snapshot.line_total)
        .fetch_one(&mut *tx)
        .await?;
        items.push(order_item_from_row(&item_row));
    }

    tx.commit().await?;

    Ok(OrderRecord {
        id: order_id,
        order_number: order_row.get("order_number"),
        public_tracking_token: order_row.get("public_tracking_token"),
        customer_name: order_row.get("customer_name"),
        customer_contact: order_row.get("customer_contact"),
        shipping_address: order_row.get("shipping_address"),
        payment_status: order_row.get("payment_status"),
        global_status: order_row.get("global_status"),
        subtotal_snapshot: order_row.get("subtotal_snapshot"),
        total_snapshot: order_row.get("total_snapshot"),
        groups,
        items,
    })
}

pub async fn find_storefront_order_summary_by_tracking_token(
    pool: &PgPool,
    tracking_token: &str,
) -> Result<Option<StorefrontOrderSummaryRecord>, CommerceRepositoryError> {
    let row = sqlx::query(
        r#"
        SELECT order_number, payment_status, global_status, total_snapshot::text AS total_snapshot
        FROM orders
        WHERE public_tracking_token = $1
        LIMIT 1
        "#,
    )
    .bind(tracking_token)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| StorefrontOrderSummaryRecord {
        order_number: row.get("order_number"),
        payment_status: row.get("payment_status"),
        global_status: row.get("global_status"),
        total_snapshot: row.get("total_snapshot"),
    }))
}

pub async fn list_admin_orders(
    pool: &PgPool,
) -> Result<Vec<AdminOrderRecord>, CommerceRepositoryError> {
    admin_orders_from_rows(fetch_admin_order_rows(pool, None).await?)
}

pub async fn list_admin_orders_for_user(
    pool: &PgPool,
    user_id: uuid::Uuid,
) -> Result<Vec<AdminOrderRecord>, CommerceRepositoryError> {
    admin_orders_from_rows(fetch_admin_order_rows(pool, Some(user_id)).await?)
}

pub fn group_checkout_snapshots_by_brand(
    snapshots: &[CheckoutVariantSnapshot],
) -> BTreeMap<uuid::Uuid, String> {
    let mut cents_by_brand = BTreeMap::<uuid::Uuid, i128>::new();
    for snapshot in snapshots {
        let cents = decimal_text_to_cents(&snapshot.line_total).unwrap_or(0);
        *cents_by_brand.entry(snapshot.brand_id).or_default() += cents;
    }
    cents_by_brand
        .into_iter()
        .map(|(brand_id, cents)| (brand_id, cents_to_decimal_text(cents)))
        .collect()
}

pub fn checkout_status_defaults() -> (&'static str, &'static str, &'static str) {
    (
        ORDER_PAYMENT_STATUS_PENDING,
        ORDER_GLOBAL_STATUS_OPEN,
        ORDER_GROUP_STATUS_NOT_READY,
    )
}

async fn insert_product_in_tx(
    tx: &mut Transaction<'_, Postgres>,
    input: &CreateProductInput,
) -> Result<ProductRecord, CommerceRepositoryError> {
    let row = sqlx::query(
        r#"
        INSERT INTO products (brand_id, category_id, name, slug, description, status)
        VALUES ($1, $2, $3, $4, $5, COALESCE($6, 'draft'))
        RETURNING id, brand_id, NULL::text AS brand_name, category_id, name, slug, description,
                  status, created_at, updated_at
        "#,
    )
    .bind(input.brand_id)
    .bind(input.category_id)
    .bind(&input.name)
    .bind(&input.slug)
    .bind(&input.description)
    .bind(&input.status)
    .fetch_one(&mut **tx)
    .await?;
    Ok(product_from_row(&row))
}

async fn insert_variant_in_tx(
    tx: &mut Transaction<'_, Postgres>,
    product_id: uuid::Uuid,
    input: &CreateVariantInput,
) -> Result<ProductVariantRecord, CommerceRepositoryError> {
    let row = sqlx::query(
        r#"
        INSERT INTO product_variants (product_id, sku, name, attributes, price, stock, status)
        VALUES ($1, $2, $3, $4, $5::numeric, $6, COALESCE($7, 'active'))
        RETURNING id, product_id, sku, name, attributes, price::text AS price, stock, status
        "#,
    )
    .bind(product_id)
    .bind(&input.sku)
    .bind(&input.name)
    .bind(&input.attributes)
    .bind(&input.price)
    .bind(input.stock)
    .bind(&input.status)
    .fetch_one(&mut **tx)
    .await?;
    Ok(variant_from_row(&row))
}

async fn fetch_product_rows(
    pool: &PgPool,
    storefront_only: bool,
    product_id: Option<uuid::Uuid>,
) -> Result<Vec<sqlx::postgres::PgRow>, CommerceRepositoryError> {
    Ok(sqlx::query(
        r#"
        SELECT
          p.id AS product_id, p.brand_id, b.name AS brand_name, p.category_id,
          p.name AS product_name, p.slug, p.description, p.status AS product_status,
          p.created_at, p.updated_at,
          v.id AS variant_id, v.product_id AS variant_product_id, v.sku, v.name AS variant_name,
          v.attributes, v.price::text AS price, v.stock, v.status AS variant_status
        FROM products p
        JOIN brands b ON b.id = p.brand_id
        JOIN product_variants v ON v.product_id = p.id
        WHERE ($1::boolean = false OR (p.status = 'published' AND v.status = 'active' AND b.status = 'active'))
          AND ($2::uuid IS NULL OR p.id = $2)
        ORDER BY p.created_at DESC, v.created_at ASC
        "#,
    )
    .bind(storefront_only)
    .bind(product_id)
    .fetch_all(pool)
    .await?)
}

async fn fetch_product_rows_for_user(
    pool: &PgPool,
    storefront_only: bool,
    product_id: Option<uuid::Uuid>,
    user_id: uuid::Uuid,
) -> Result<Vec<sqlx::postgres::PgRow>, CommerceRepositoryError> {
    Ok(sqlx::query(
        r#"
        SELECT
          p.id AS product_id, p.brand_id, b.name AS brand_name, p.category_id,
          p.name AS product_name, p.slug, p.description, p.status AS product_status,
          p.created_at, p.updated_at,
          v.id AS variant_id, v.product_id AS variant_product_id, v.sku, v.name AS variant_name,
          v.attributes, v.price::text AS price, v.stock, v.status AS variant_status
        FROM products p
        JOIN brands b ON b.id = p.brand_id
        JOIN product_variants v ON v.product_id = p.id
        JOIN brand_members bm ON bm.brand_id = p.brand_id
        WHERE bm.user_id = $3
          AND ($1::boolean = false OR (p.status = 'published' AND v.status = 'active' AND b.status = 'active'))
          AND ($2::uuid IS NULL OR p.id = $2)
        ORDER BY p.created_at DESC, v.created_at ASC
        "#,
    )
    .bind(storefront_only)
    .bind(product_id)
    .bind(user_id)
    .fetch_all(pool)
    .await?)
}

pub async fn user_is_brand_member(
    pool: &PgPool,
    user_id: uuid::Uuid,
    brand_id: uuid::Uuid,
) -> Result<bool, CommerceRepositoryError> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
          SELECT 1
          FROM brand_members
          WHERE user_id = $1
            AND brand_id = $2
        )
        "#,
    )
    .bind(user_id)
    .bind(brand_id)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

fn products_from_rows(
    rows: Vec<sqlx::postgres::PgRow>,
) -> Result<Vec<ProductWithVariantsRecord>, CommerceRepositoryError> {
    let mut products = Vec::<ProductWithVariantsRecord>::new();
    for row in rows {
        let product_id: uuid::Uuid = row.get("product_id");
        if products
            .last()
            .map(|entry| entry.product.id != product_id)
            .unwrap_or(true)
        {
            products.push(ProductWithVariantsRecord {
                product: product_from_product_join_row(&row),
                variants: Vec::new(),
            });
        }
        if let Some(product) = products.last_mut() {
            product.variants.push(variant_from_product_join_row(&row));
        }
    }
    Ok(products)
}

fn admin_orders_from_rows(
    rows: Vec<sqlx::postgres::PgRow>,
) -> Result<Vec<AdminOrderRecord>, CommerceRepositoryError> {
    let mut orders = Vec::<AdminOrderRecord>::new();

    for row in rows {
        let order_id: uuid::Uuid = row.get("order_id");
        let group = OrderBrandGroupRecord {
            id: row.get("group_id"),
            order_id,
            brand_id: row.get("brand_id"),
            fulfillment_status: row.get("fulfillment_status"),
            subtotal_snapshot: row.get("subtotal_snapshot"),
        };
        let proof_status: Option<String> = row.get("payment_proof_status");

        match orders.last_mut() {
            Some(existing) if existing.id == order_id => {
                if !existing.groups.iter().any(|entry| entry.id == group.id) {
                    existing.groups.push(group);
                }
                if let Some(status) = proof_status {
                    if !existing.payment_proof_statuses.contains(&status) {
                        existing.payment_proof_statuses.push(status);
                    }
                }
            }
            _ => {
                let mut payment_proof_statuses = Vec::new();
                if let Some(status) = proof_status {
                    payment_proof_statuses.push(status);
                }
                orders.push(AdminOrderRecord {
                    id: order_id,
                    order_number: row.get("order_number"),
                    public_tracking_token: row.get("public_tracking_token"),
                    customer_name: row.get("customer_name"),
                    payment_status: row.get("payment_status"),
                    global_status: row.get("global_status"),
                    total_snapshot: row.get("total_snapshot"),
                    created_at: row.get("created_at"),
                    groups: vec![group],
                    item_count: row.get("item_count"),
                    payment_proof_statuses,
                });
            }
        }
    }

    Ok(orders)
}

async fn fetch_admin_order_rows(
    pool: &PgPool,
    seller_user_id: Option<uuid::Uuid>,
) -> Result<Vec<sqlx::postgres::PgRow>, CommerceRepositoryError> {
    Ok(sqlx::query(
        r#"
        SELECT
          o.id AS order_id,
          o.order_number,
          o.public_tracking_token,
          o.customer_name,
          o.payment_status,
          o.global_status,
          o.total_snapshot::text AS total_snapshot,
          o.created_at,
          obg.id AS group_id,
          obg.brand_id,
          obg.fulfillment_status,
          obg.subtotal_snapshot::text AS subtotal_snapshot,
          (
            SELECT COUNT(*)::bigint FROM order_items oi WHERE oi.order_id = o.id
          ) AS item_count,
          pp.status AS payment_proof_status
        FROM orders o
        JOIN order_brand_groups obg ON obg.order_id = o.id
        LEFT JOIN payment_proofs pp ON pp.order_id = o.id
        LEFT JOIN brand_members bm ON bm.brand_id = obg.brand_id
        WHERE ($1::uuid IS NULL OR bm.user_id = $1)
        ORDER BY o.created_at DESC, obg.created_at ASC
        "#,
    )
    .bind(seller_user_id)
    .fetch_all(pool)
    .await?)
}

fn validate_checkout_items(items: &[CheckoutItemInput]) -> Result<(), CommerceRepositoryError> {
    if items.is_empty() {
        return Err(CommerceRepositoryError::EmptyCheckout);
    }
    if items.iter().any(|item| item.quantity <= 0) {
        return Err(CommerceRepositoryError::InvalidQuantity);
    }
    Ok(())
}

async fn reserve_checkout_snapshots(
    tx: &mut Transaction<'_, Postgres>,
    items: &[CheckoutItemInput],
) -> Result<Vec<CheckoutVariantSnapshot>, CommerceRepositoryError> {
    let mut snapshots = Vec::new();
    for item in items {
        let updated = sqlx::query(
            r#"
            WITH candidate AS (
              SELECT
                p.id AS product_id,
                v.id AS variant_id,
                b.id AS brand_id,
                p.name AS product_name,
                v.name AS variant_name,
                b.name AS brand_name,
                v.sku,
                v.price::text AS unit_price,
                v.stock AS current_stock,
                (v.price * $2::integer)::text AS line_total
              FROM product_variants v
              JOIN products p ON p.id = v.product_id
              JOIN brands b ON b.id = p.brand_id
              WHERE v.id = $1
                AND v.status = 'active'
                AND p.status = 'published'
                AND b.status = 'active'
              FOR UPDATE OF v
            ),
            decremented AS (
              UPDATE product_variants v
              SET
                stock = v.stock - $2,
                updated_at = now()
              FROM candidate c
              WHERE v.id = c.variant_id
                AND v.stock >= $2
              RETURNING
                c.product_id,
                c.variant_id,
                c.brand_id,
                c.product_name,
                c.variant_name,
                c.brand_name,
                c.sku,
                c.unit_price,
                c.current_stock,
                c.line_total
            )
            SELECT
              product_id,
              variant_id,
              brand_id,
              product_name,
              variant_name,
              brand_name,
              sku,
              unit_price,
              current_stock,
              line_total
            FROM decremented
            "#,
        )
        .bind(item.variant_id)
        .bind(item.quantity)
        .fetch_optional(&mut **tx)
        .await?;

        let row = if let Some(row) = updated {
            row
        } else {
            let current = sqlx::query(
                r#"
                SELECT v.stock
                FROM product_variants v
                JOIN products p ON p.id = v.product_id
                JOIN brands b ON b.id = p.brand_id
                WHERE v.id = $1
                  AND v.status = 'active'
                  AND p.status = 'published'
                  AND b.status = 'active'
                FOR UPDATE OF v
                "#,
            )
            .bind(item.variant_id)
            .fetch_optional(&mut **tx)
            .await?;

            match current {
                None => return Err(CommerceRepositoryError::VariantNotFound),
                Some(current_row) => {
                    let stock: i32 = current_row.get("stock");
                    if stock <= 0 {
                        return Err(CommerceRepositoryError::OutOfStock);
                    }
                    return Err(CommerceRepositoryError::Overstock);
                }
            }
        };

        let stock: i32 = row.get("current_stock");
        if stock <= 0 {
            return Err(CommerceRepositoryError::OutOfStock);
        }
        if item.quantity > stock {
            return Err(CommerceRepositoryError::Overstock);
        }

        snapshots.push(CheckoutVariantSnapshot {
            product_id: row.get("product_id"),
            variant_id: row.get("variant_id"),
            brand_id: row.get("brand_id"),
            product_name: row.get("product_name"),
            variant_name: row.get("variant_name"),
            brand_name: row.get("brand_name"),
            sku: row.get("sku"),
            unit_price: row.get("unit_price"),
            stock,
            quantity: item.quantity,
            line_total: row.get("line_total"),
        });
    }
    Ok(snapshots)
}

fn checkout_subtotal_sql(snapshots: &[CheckoutVariantSnapshot]) -> String {
    let cents = snapshots
        .iter()
        .map(|snapshot| decimal_text_to_cents(&snapshot.line_total).unwrap_or(0))
        .sum();
    cents_to_decimal_text(cents)
}

fn generate_order_number() -> String {
    let compact = uuid::Uuid::new_v4().simple().to_string();
    format!("ORD-{}", &compact[..12].to_ascii_uppercase())
}

fn decimal_text_to_cents(value: &str) -> Option<i128> {
    let (whole, fractional) = value.split_once('.').unwrap_or((value, "0"));
    let whole: i128 = whole.parse().ok()?;
    let mut frac = fractional.chars().take(2).collect::<String>();
    while frac.len() < 2 {
        frac.push('0');
    }
    let frac: i128 = frac.parse().ok()?;
    Some((whole * 100) + frac)
}

fn cents_to_decimal_text(cents: i128) -> String {
    format!("{}.{:02}", cents / 100, (cents % 100).abs())
}

fn brand_from_row(row: &sqlx::postgres::PgRow) -> BrandRecord {
    BrandRecord {
        id: row.get("id"),
        name: row.get("name"),
        slug: row.get("slug"),
        description: row.get("description"),
        status: row.get("status"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn category_from_row(row: &sqlx::postgres::PgRow) -> CategoryRecord {
    CategoryRecord {
        id: row.get("id"),
        brand_id: row.get("brand_id"),
        parent_id: row.get("parent_id"),
        name: row.get("name"),
        slug: row.get("slug"),
        status: row.get("status"),
    }
}

fn product_from_row(row: &sqlx::postgres::PgRow) -> ProductRecord {
    ProductRecord {
        id: row.get("id"),
        brand_id: row.get("brand_id"),
        brand_name: row.get("brand_name"),
        category_id: row.get("category_id"),
        name: row.get("name"),
        slug: row.get("slug"),
        description: row.get("description"),
        status: row.get("status"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn product_from_product_join_row(row: &sqlx::postgres::PgRow) -> ProductRecord {
    ProductRecord {
        id: row.get("product_id"),
        brand_id: row.get("brand_id"),
        brand_name: row.get("brand_name"),
        category_id: row.get("category_id"),
        name: row.get("product_name"),
        slug: row.get("slug"),
        description: row.get("description"),
        status: row.get("product_status"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn variant_from_row(row: &sqlx::postgres::PgRow) -> ProductVariantRecord {
    ProductVariantRecord {
        id: row.get("id"),
        product_id: row.get("product_id"),
        sku: row.get("sku"),
        name: row.get("name"),
        attributes: row.get("attributes"),
        price: row.get("price"),
        stock: row.get("stock"),
        status: row.get("status"),
    }
}

fn variant_from_product_join_row(row: &sqlx::postgres::PgRow) -> ProductVariantRecord {
    ProductVariantRecord {
        id: row.get("variant_id"),
        product_id: row.get("variant_product_id"),
        sku: row.get("sku"),
        name: row.get("variant_name"),
        attributes: row.get("attributes"),
        price: row.get("price"),
        stock: row.get("stock"),
        status: row.get("variant_status"),
    }
}

fn order_group_from_row(row: &sqlx::postgres::PgRow) -> OrderBrandGroupRecord {
    OrderBrandGroupRecord {
        id: row.get("id"),
        order_id: row.get("order_id"),
        brand_id: row.get("brand_id"),
        fulfillment_status: row.get("fulfillment_status"),
        subtotal_snapshot: row.get("subtotal_snapshot"),
    }
}

fn order_item_from_row(row: &sqlx::postgres::PgRow) -> OrderItemRecord {
    OrderItemRecord {
        id: row.get("id"),
        order_id: row.get("order_id"),
        order_brand_group_id: row.get("order_brand_group_id"),
        brand_id: row.get("brand_id"),
        product_id: row.get("product_id"),
        product_variant_id: row.get("product_variant_id"),
        product_name_snapshot: row.get("product_name_snapshot"),
        variant_name_snapshot: row.get("variant_name_snapshot"),
        brand_name_snapshot: row.get("brand_name_snapshot"),
        sku_snapshot: row.get("sku_snapshot"),
        unit_price_snapshot: row.get("unit_price_snapshot"),
        quantity: row.get("quantity"),
        line_total_snapshot: row.get("line_total_snapshot"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[test]
    fn grouping_aggregates_checkout_snapshots_per_brand() {
        let brand_a = uuid::Uuid::new_v4();
        let brand_b = uuid::Uuid::new_v4();
        let snapshots = vec![
            snapshot_for_brand(brand_a, "12000.00"),
            snapshot_for_brand(brand_b, "5000.50"),
            snapshot_for_brand(brand_a, "8000.25"),
        ];

        let grouped = group_checkout_snapshots_by_brand(&snapshots);

        assert_eq!(grouped.get(&brand_a).unwrap(), "20000.25");
        assert_eq!(grouped.get(&brand_b).unwrap(), "5000.50");
    }

    #[test]
    fn checkout_status_defaults_match_erd_canonical_values() {
        assert_eq!(checkout_status_defaults(), ("pending", "open", "not_ready"));
    }

    fn snapshot_for_brand(brand_id: uuid::Uuid, line_total: &str) -> CheckoutVariantSnapshot {
        CheckoutVariantSnapshot {
            product_id: uuid::Uuid::new_v4(),
            variant_id: uuid::Uuid::new_v4(),
            brand_id,
            product_name: "Product".to_string(),
            variant_name: Some("Default".to_string()),
            brand_name: "Brand".to_string(),
            sku: None,
            unit_price: line_total.to_string(),
            stock: 10,
            quantity: 1,
            line_total: line_total.to_string(),
        }
    }

    #[tokio::test]
    #[ignore = "requires TEST_DATABASE_URL pointing at an isolated disposable PostgreSQL database"]
    async fn checkout_decrements_stock_and_persists_order() {
        let database_url =
            std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url)
            .await
            .expect("test database must be reachable");

        crate::run_core_migrations(&pool)
            .await
            .expect("core migrations must apply");
        reset_commerce_test_data(&pool).await;

        let brand = create_brand(
            &pool,
            CreateBrandInput {
                name: "Brand A".to_string(),
                slug: format!("brand-a-{}", uuid::Uuid::new_v4()),
                description: None,
                status: Some("active".to_string()),
            },
        )
        .await
        .expect("brand must be created");

        let product = create_product_with_default_variant(
            &pool,
            CreateProductInput {
                brand_id: brand.id,
                category_id: None,
                name: "Test Product".to_string(),
                slug: format!("product-{}", uuid::Uuid::new_v4()),
                description: None,
                status: Some("published".to_string()),
                default_variant: CreateVariantInput {
                    sku: Some(format!("SKU-{}", uuid::Uuid::new_v4().simple())),
                    name: "Default".to_string(),
                    attributes: serde_json::json!({}),
                    price: "25000.00".to_string(),
                    stock: 5,
                    status: Some("active".to_string()),
                },
            },
        )
        .await
        .expect("product with variant must be created");

        let variant_id = product.variants[0].id;
        let order = checkout_submitted_items(
            &pool,
            CheckoutInput {
                customer_name: "Buyer".to_string(),
                customer_contact: "08123456789".to_string(),
                shipping_address: "Jl. Test".to_string(),
                items: vec![CheckoutItemInput {
                    variant_id,
                    quantity: 3,
                }],
            },
        )
        .await
        .expect("checkout must succeed");

        assert_eq!(order.payment_status, ORDER_PAYMENT_STATUS_PENDING);
        assert_eq!(order.global_status, ORDER_GLOBAL_STATUS_OPEN);
        assert_eq!(order.items.len(), 1);
        assert_eq!(order.items[0].quantity, 3);

        let stock_after: i32 =
            sqlx::query_scalar("SELECT stock FROM product_variants WHERE id = $1")
                .bind(variant_id)
                .fetch_one(&pool)
                .await
                .expect("stock query must succeed");
        assert_eq!(stock_after, 2);
    }

    #[tokio::test]
    #[ignore = "requires TEST_DATABASE_URL pointing at an isolated disposable PostgreSQL database"]
    async fn concurrent_checkout_rejects_overstock_and_never_negative() {
        let database_url =
            std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url)
            .await
            .expect("test database must be reachable");

        crate::run_core_migrations(&pool)
            .await
            .expect("core migrations must apply");
        reset_commerce_test_data(&pool).await;

        let brand = create_brand(
            &pool,
            CreateBrandInput {
                name: "Brand B".to_string(),
                slug: format!("brand-b-{}", uuid::Uuid::new_v4()),
                description: None,
                status: Some("active".to_string()),
            },
        )
        .await
        .expect("brand must be created");

        let product = create_product_with_default_variant(
            &pool,
            CreateProductInput {
                brand_id: brand.id,
                category_id: None,
                name: "Race Product".to_string(),
                slug: format!("race-product-{}", uuid::Uuid::new_v4()),
                description: None,
                status: Some("published".to_string()),
                default_variant: CreateVariantInput {
                    sku: Some(format!("RACE-{}", uuid::Uuid::new_v4().simple())),
                    name: "Default".to_string(),
                    attributes: serde_json::json!({}),
                    price: "10000.00".to_string(),
                    stock: 5,
                    status: Some("active".to_string()),
                },
            },
        )
        .await
        .expect("product with variant must be created");

        let variant_id = product.variants[0].id;

        let (left, right) = tokio::join!(
            checkout_submitted_items(
                &pool,
                CheckoutInput {
                    customer_name: "Buyer Left".to_string(),
                    customer_contact: "left".to_string(),
                    shipping_address: "Left".to_string(),
                    items: vec![CheckoutItemInput {
                        variant_id,
                        quantity: 4,
                    }],
                }
            ),
            checkout_submitted_items(
                &pool,
                CheckoutInput {
                    customer_name: "Buyer Right".to_string(),
                    customer_contact: "right".to_string(),
                    shipping_address: "Right".to_string(),
                    items: vec![CheckoutItemInput {
                        variant_id,
                        quantity: 4,
                    }],
                }
            )
        );

        let success_count = usize::from(left.is_ok()) + usize::from(right.is_ok());
        let overstock_count = usize::from(matches!(left, Err(CommerceRepositoryError::Overstock)))
            + usize::from(matches!(right, Err(CommerceRepositoryError::Overstock)));

        assert_eq!(success_count, 1);
        assert_eq!(overstock_count, 1);

        let stock_after: i32 =
            sqlx::query_scalar("SELECT stock FROM product_variants WHERE id = $1")
                .bind(variant_id)
                .fetch_one(&pool)
                .await
                .expect("stock query must succeed");
        assert_eq!(stock_after, 1);
        assert!(stock_after >= 0);
    }

    async fn reset_commerce_test_data(pool: &PgPool) {
        for query in [
            "DELETE FROM payment_proofs",
            "DELETE FROM order_status_history",
            "DELETE FROM order_items",
            "DELETE FROM order_brand_groups",
            "DELETE FROM orders",
            "DELETE FROM cart_items",
            "DELETE FROM carts",
            "DELETE FROM product_images",
            "DELETE FROM product_variants",
            "DELETE FROM products",
            "DELETE FROM categories",
            "DELETE FROM brand_member_permissions",
            "DELETE FROM brand_members",
            "DELETE FROM brands",
            "DELETE FROM file_objects",
        ] {
            sqlx::query(query)
                .execute(pool)
                .await
                .expect("test data reset succeeds");
        }
    }
}
