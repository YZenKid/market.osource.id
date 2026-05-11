use serde_json::{Map, Value};
use sqlx::{PgPool, Row};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEventRecord {
    pub id: uuid::Uuid,
    pub actor_user_id: Option<uuid::Uuid>,
    pub action: String,
    pub target_type: String,
    pub target_id: Option<uuid::Uuid>,
    pub result: String,
    pub metadata: Value,
}

#[derive(Debug, Clone)]
pub struct AuditEventInput {
    pub actor_user_id: Option<uuid::Uuid>,
    pub action: String,
    pub target_type: String,
    pub target_id: Option<uuid::Uuid>,
    pub result: String,
    pub metadata: Value,
}

#[derive(Debug, thiserror::Error)]
pub enum AuditRepositoryError {
    #[error("audit metadata must be a JSON object")]
    MetadataMustBeObject,
    #[error("audit metadata contains blocked sensitive key `{0}`")]
    SensitiveMetadataKey(String),
    #[error("database error")]
    Database(#[from] sqlx::Error),
}

pub async fn write_audit_event(
    pool: &PgPool,
    input: AuditEventInput,
) -> Result<AuditEventRecord, AuditRepositoryError> {
    validate_safe_metadata(&input.metadata)?;

    let row = sqlx::query(
        r#"
        INSERT INTO audit_events (actor_user_id, action, target_type, target_id, result, metadata)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, actor_user_id, action, target_type, target_id, result, metadata
        "#,
    )
    .bind(input.actor_user_id)
    .bind(&input.action)
    .bind(&input.target_type)
    .bind(input.target_id)
    .bind(&input.result)
    .bind(input.metadata)
    .fetch_one(pool)
    .await?;

    Ok(AuditEventRecord {
        id: row.get("id"),
        actor_user_id: row.get("actor_user_id"),
        action: row.get("action"),
        target_type: row.get("target_type"),
        target_id: row.get("target_id"),
        result: row.get("result"),
        metadata: row.get("metadata"),
    })
}

pub fn safe_audit_metadata(entries: &[(&str, Value)]) -> Result<Value, AuditRepositoryError> {
    let mut object = Map::new();
    for (key, value) in entries {
        if is_sensitive_key(key) {
            return Err(AuditRepositoryError::SensitiveMetadataKey(
                (*key).to_string(),
            ));
        }
        object.insert((*key).to_string(), value.clone());
    }
    Ok(Value::Object(object))
}

fn validate_safe_metadata(metadata: &Value) -> Result<(), AuditRepositoryError> {
    let Some(object) = metadata.as_object() else {
        return Err(AuditRepositoryError::MetadataMustBeObject);
    };

    for key in object.keys() {
        if is_sensitive_key(key) {
            return Err(AuditRepositoryError::SensitiveMetadataKey(key.clone()));
        }
    }
    Ok(())
}

fn is_sensitive_key(key: &str) -> bool {
    let normalized = key.to_ascii_lowercase();
    normalized.contains("password")
        || normalized.contains("secret")
        || normalized.contains("token")
        || normalized.contains("email")
        || normalized.contains("phone")
        || normalized.contains("address")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_metadata_accepts_minimal_non_pii_shape() {
        let metadata = safe_audit_metadata(&[
            ("reason", Value::String("invalid_credentials".to_string())),
            ("rate_limited", Value::Bool(false)),
        ])
        .expect("metadata should be accepted");

        assert_eq!(metadata["reason"], "invalid_credentials");
        assert_eq!(metadata["rate_limited"], false);
    }

    #[test]
    fn safe_metadata_rejects_sensitive_keys() {
        let error =
            safe_audit_metadata(&[("email", Value::String("user@example.com".to_string()))])
                .expect_err("email metadata should be rejected");

        assert!(matches!(
            error,
            AuditRepositoryError::SensitiveMetadataKey(_)
        ));
    }
}
