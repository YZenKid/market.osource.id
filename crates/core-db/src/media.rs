use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, QueryBuilder, Row, Transaction};

pub const PAYMENT_PROOF_VIEW_ASSIGNED: &str = "payment_proof.view_assigned";
pub const PAYMENT_PROOF_VERIFY: &str = "payment_proof.verify";
pub const PAYMENT_PROOF_REJECT: &str = "payment_proof.reject";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileObjectRecord {
    pub id: uuid::Uuid,
    pub storage_provider: String,
    pub bucket: String,
    pub object_key: String,
    pub original_filename: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub checksum: String,
    pub visibility: String,
    pub created_by_user_id: Option<uuid::Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentProofRecord {
    pub id: uuid::Uuid,
    pub order_id: uuid::Uuid,
    pub uploaded_by_user_id: Option<uuid::Uuid>,
    pub file_object_id: uuid::Uuid,
    pub status: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct PaymentProofSummaryRecord {
    pub id: uuid::Uuid,
    pub order_id: uuid::Uuid,
    pub file_object_id: uuid::Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateFileObjectInput {
    pub storage_provider: String,
    pub bucket: String,
    pub object_key: String,
    pub original_filename: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub checksum: String,
    pub visibility: String,
    pub created_by_user_id: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentProofDecision {
    Verified,
    Rejected,
}

#[derive(Debug, thiserror::Error)]
pub enum MediaRepositoryError {
    #[error("order not found")]
    OrderNotFound,
    #[error("payment proof not found")]
    PaymentProofNotFound,
    #[error("database error")]
    Database(#[from] sqlx::Error),
}

pub async fn list_payment_proofs(
    pool: &PgPool,
) -> Result<Vec<PaymentProofSummaryRecord>, MediaRepositoryError> {
    let rows = sqlx::query(
        r#"
        SELECT id, order_id, file_object_id, status, created_at
        FROM payment_proofs
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.iter().map(payment_proof_summary_from_row).collect())
}

pub async fn list_payment_proofs_for_user(
    pool: &PgPool,
    user_id: uuid::Uuid,
) -> Result<Vec<PaymentProofSummaryRecord>, MediaRepositoryError> {
    let rows = sqlx::query(
        r#"
        SELECT DISTINCT pp.id, pp.order_id, pp.file_object_id, pp.status, pp.created_at
        FROM payment_proofs pp
        JOIN order_brand_groups obg ON obg.order_id = pp.order_id
        JOIN brand_members bm ON bm.brand_id = obg.brand_id
        WHERE bm.user_id = $1
        ORDER BY pp.created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.iter().map(payment_proof_summary_from_row).collect())
}

pub async fn grant_brand_member_permission(
    pool: &PgPool,
    brand_member_id: uuid::Uuid,
    permission_code: &str,
    granted_by_user_id: uuid::Uuid,
) -> Result<(), MediaRepositoryError> {
    sqlx::query(
        r#"
        INSERT INTO brand_member_permissions (brand_member_id, permission_code, granted_by_user_id)
        VALUES ($1, $2, $3)
        ON CONFLICT (brand_member_id, permission_code) DO NOTHING
        "#,
    )
    .bind(brand_member_id)
    .bind(permission_code)
    .bind(granted_by_user_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_file_object(
    pool: &PgPool,
    input: CreateFileObjectInput,
) -> Result<FileObjectRecord, MediaRepositoryError> {
    let row = sqlx::query(
        r#"
        INSERT INTO file_objects (
          storage_provider, bucket, object_key, original_filename, mime_type,
          size_bytes, checksum, visibility, created_by_user_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, storage_provider, bucket, object_key, original_filename, mime_type,
                  size_bytes, checksum, visibility, created_by_user_id, created_at
        "#,
    )
    .bind(&input.storage_provider)
    .bind(&input.bucket)
    .bind(&input.object_key)
    .bind(&input.original_filename)
    .bind(&input.mime_type)
    .bind(input.size_bytes)
    .bind(&input.checksum)
    .bind(&input.visibility)
    .bind(input.created_by_user_id)
    .fetch_one(pool)
    .await?;

    Ok(file_object_from_row(&row))
}

pub async fn create_payment_proof_for_tracking_token(
    pool: &PgPool,
    tracking_token: &str,
    file_object_id: uuid::Uuid,
    uploaded_by_user_id: Option<uuid::Uuid>,
) -> Result<PaymentProofRecord, MediaRepositoryError> {
    let mut tx = pool.begin().await?;
    let order_id = sqlx::query_scalar::<_, uuid::Uuid>(
        r#"
        UPDATE orders
        SET payment_status = 'waiting_payment_verification', updated_at = now()
        WHERE public_tracking_token = $1
        RETURNING id
        "#,
    )
    .bind(tracking_token)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(MediaRepositoryError::OrderNotFound)?;

    let proof =
        insert_payment_proof_in_tx(&mut tx, order_id, file_object_id, uploaded_by_user_id).await?;
    tx.commit().await?;
    Ok(proof)
}

pub async fn find_order_id_by_tracking_token(
    pool: &PgPool,
    tracking_token: &str,
) -> Result<Option<uuid::Uuid>, MediaRepositoryError> {
    let order_id = sqlx::query_scalar::<_, uuid::Uuid>(
        r#"
        SELECT id
        FROM orders
        WHERE public_tracking_token = $1
        LIMIT 1
        "#,
    )
    .bind(tracking_token)
    .fetch_optional(pool)
    .await?;

    Ok(order_id)
}

pub async fn find_payment_proof_file(
    pool: &PgPool,
    file_id: uuid::Uuid,
) -> Result<Option<(FileObjectRecord, PaymentProofRecord)>, MediaRepositoryError> {
    let row = sqlx::query(
        r#"
        SELECT
          file_objects.id AS file_id,
          file_objects.storage_provider,
          file_objects.bucket,
          file_objects.object_key,
          file_objects.original_filename,
          file_objects.mime_type,
          file_objects.size_bytes,
          file_objects.checksum,
          file_objects.visibility,
          file_objects.created_by_user_id,
          file_objects.created_at AS file_created_at,
          payment_proofs.id AS proof_id,
          payment_proofs.order_id,
          payment_proofs.uploaded_by_user_id,
          payment_proofs.file_object_id,
          payment_proofs.status AS proof_status,
          payment_proofs.note,
          payment_proofs.created_at AS proof_created_at
        FROM file_objects
        JOIN payment_proofs ON payment_proofs.file_object_id = file_objects.id
        WHERE file_objects.id = $1
        LIMIT 1
        "#,
    )
    .bind(file_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| {
        (
            file_object_from_joined_row(&row),
            payment_proof_from_joined_row(&row),
        )
    }))
}

pub async fn find_payment_proof_by_id(
    pool: &PgPool,
    proof_id: uuid::Uuid,
) -> Result<Option<PaymentProofRecord>, MediaRepositoryError> {
    let row = sqlx::query(
        r#"
        SELECT id, order_id, uploaded_by_user_id, file_object_id, status, note, created_at
        FROM payment_proofs
        WHERE id = $1
        LIMIT 1
        "#,
    )
    .bind(proof_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| payment_proof_from_row(&row)))
}

pub async fn order_contains_assigned_seller_brand(
    pool: &PgPool,
    order_id: uuid::Uuid,
    user_id: uuid::Uuid,
) -> Result<bool, MediaRepositoryError> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
          SELECT 1
          FROM order_brand_groups obg
          JOIN brand_members bm ON bm.brand_id = obg.brand_id
          WHERE obg.order_id = $1
            AND bm.user_id = $2
        )
        "#,
    )
    .bind(order_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn user_has_brand_scoped_permission_for_order(
    pool: &PgPool,
    user_id: uuid::Uuid,
    order_id: uuid::Uuid,
    permission_code: &str,
) -> Result<bool, MediaRepositoryError> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
          SELECT 1
          FROM order_brand_groups obg
          JOIN brand_members bm ON bm.brand_id = obg.brand_id
          JOIN brand_member_permissions bmp ON bmp.brand_member_id = bm.id
          WHERE obg.order_id = $1
            AND bm.user_id = $2
            AND bmp.permission_code = $3
        )
        "#,
    )
    .bind(order_id)
    .bind(user_id)
    .bind(permission_code)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn decide_payment_proof(
    pool: &PgPool,
    proof_id: uuid::Uuid,
    actor_user_id: uuid::Uuid,
    decision: PaymentProofDecision,
    note: Option<&str>,
) -> Result<PaymentProofRecord, MediaRepositoryError> {
    let (status, verified_by, rejected_by) = match decision {
        PaymentProofDecision::Verified => ("verified", Some(actor_user_id), None),
        PaymentProofDecision::Rejected => ("rejected", None, Some(actor_user_id)),
    };
    let row = sqlx::query(
        r#"
        UPDATE payment_proofs
        SET status = $2,
            note = $3,
            verified_by_user_id = COALESCE($4, verified_by_user_id),
            verified_at = CASE WHEN $4 IS NULL THEN verified_at ELSE now() END,
            rejected_by_user_id = COALESCE($5, rejected_by_user_id),
            rejected_at = CASE WHEN $5 IS NULL THEN rejected_at ELSE now() END
        WHERE id = $1
        RETURNING id, order_id, uploaded_by_user_id, file_object_id, status, note, created_at
        "#,
    )
    .bind(proof_id)
    .bind(status)
    .bind(note)
    .bind(verified_by)
    .bind(rejected_by)
    .fetch_optional(pool)
    .await?
    .ok_or(MediaRepositoryError::PaymentProofNotFound)?;

    Ok(payment_proof_from_row(&row))
}

pub fn build_user_has_brand_scoped_permission_query() -> String {
    let mut builder = QueryBuilder::<Postgres>::new("SELECT EXISTS (");
    builder.push(" SELECT 1 FROM order_brand_groups obg");
    builder.push(" JOIN brand_members bm ON bm.brand_id = obg.brand_id");
    builder.push(" JOIN brand_member_permissions bmp ON bmp.brand_member_id = bm.id");
    builder.push(" WHERE obg.order_id = ");
    builder.push_bind(uuid::Uuid::nil());
    builder.push(" AND bm.user_id = ");
    builder.push_bind(uuid::Uuid::nil());
    builder.push(" AND bmp.permission_code = ");
    builder.push_bind(PAYMENT_PROOF_VIEW_ASSIGNED);
    builder.push(" )");
    builder.sql().to_string()
}

async fn insert_payment_proof_in_tx(
    tx: &mut Transaction<'_, Postgres>,
    order_id: uuid::Uuid,
    file_object_id: uuid::Uuid,
    uploaded_by_user_id: Option<uuid::Uuid>,
) -> Result<PaymentProofRecord, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO payment_proofs (order_id, uploaded_by_user_id, file_object_id, status)
        VALUES ($1, $2, $3, 'uploaded')
        RETURNING id, order_id, uploaded_by_user_id, file_object_id, status, note, created_at
        "#,
    )
    .bind(order_id)
    .bind(uploaded_by_user_id)
    .bind(file_object_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(payment_proof_from_row(&row))
}

fn file_object_from_row(row: &sqlx::postgres::PgRow) -> FileObjectRecord {
    FileObjectRecord {
        id: row.get("id"),
        storage_provider: row.get("storage_provider"),
        bucket: row.get("bucket"),
        object_key: row.get("object_key"),
        original_filename: row.get("original_filename"),
        mime_type: row.get("mime_type"),
        size_bytes: row.get("size_bytes"),
        checksum: row.get("checksum"),
        visibility: row.get("visibility"),
        created_by_user_id: row.get("created_by_user_id"),
        created_at: row.get("created_at"),
    }
}

fn file_object_from_joined_row(row: &sqlx::postgres::PgRow) -> FileObjectRecord {
    FileObjectRecord {
        id: row.get("file_id"),
        storage_provider: row.get("storage_provider"),
        bucket: row.get("bucket"),
        object_key: row.get("object_key"),
        original_filename: row.get("original_filename"),
        mime_type: row.get("mime_type"),
        size_bytes: row.get("size_bytes"),
        checksum: row.get("checksum"),
        visibility: row.get("visibility"),
        created_by_user_id: row.get("created_by_user_id"),
        created_at: row.get("file_created_at"),
    }
}

fn payment_proof_from_row(row: &sqlx::postgres::PgRow) -> PaymentProofRecord {
    PaymentProofRecord {
        id: row.get("id"),
        order_id: row.get("order_id"),
        uploaded_by_user_id: row.get("uploaded_by_user_id"),
        file_object_id: row.get("file_object_id"),
        status: row.get("status"),
        note: row.get("note"),
        created_at: row.get("created_at"),
    }
}

fn payment_proof_from_joined_row(row: &sqlx::postgres::PgRow) -> PaymentProofRecord {
    PaymentProofRecord {
        id: row.get("proof_id"),
        order_id: row.get("order_id"),
        uploaded_by_user_id: row.get("uploaded_by_user_id"),
        file_object_id: row.get("file_object_id"),
        status: row.get("proof_status"),
        note: row.get("note"),
        created_at: row.get("proof_created_at"),
    }
}

fn payment_proof_summary_from_row(row: &sqlx::postgres::PgRow) -> PaymentProofSummaryRecord {
    PaymentProofSummaryRecord {
        id: row.get("id"),
        order_id: row.get("order_id"),
        file_object_id: row.get("file_object_id"),
        status: row.get("status"),
        created_at: row.get("created_at"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_query_is_brand_member_scoped() {
        let sql = build_user_has_brand_scoped_permission_query();

        assert!(sql.contains("order_brand_groups"));
        assert!(sql.contains("brand_members"));
        assert!(sql.contains("brand_member_permissions"));
        assert!(sql.contains("permission_code"));
    }
}
