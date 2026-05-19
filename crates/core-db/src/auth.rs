use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthUserRecord {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role_code: String,
    pub role_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRecord {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub session_hash: String,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentSessionRecord {
    pub session_id: uuid::Uuid,
    pub session_expires_at: DateTime<Utc>,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub role_code: String,
    pub role_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct UserSummaryRecord {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub status: String,
    pub role_code: String,
    pub role_name: String,
}

#[derive(Debug, Clone)]
pub struct CreateSellerInput {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthRepositoryError {
    #[error("database error")]
    Database(#[from] sqlx::Error),
}

pub async fn list_active_users_with_roles(
    pool: &PgPool,
) -> Result<Vec<UserSummaryRecord>, AuthRepositoryError> {
    let rows = sqlx::query(
        r#"
        SELECT users.id, users.name, users.email, users.status,
               roles.code AS role_code, roles.name AS role_name
        FROM users
        JOIN roles ON roles.id = users.role_id
        WHERE users.status = 'active'
        ORDER BY users.created_at DESC, users.name ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.iter().map(user_summary_from_row).collect())
}

pub async fn create_seller_user(
    pool: &PgPool,
    input: CreateSellerInput,
) -> Result<UserSummaryRecord, AuthRepositoryError> {
    let row = sqlx::query(
        r#"
        INSERT INTO users (role_id, name, email, password_hash, status)
        SELECT id, $1, $2, $3, 'active'
        FROM roles WHERE code = 'seller'
        RETURNING users.id, users.name, users.email, users.status,
                  'seller'::text AS role_code, 'Seller'::text AS role_name
        "#,
    )
    .bind(input.name)
    .bind(input.email)
    .bind(input.password_hash)
    .fetch_one(pool)
    .await?;

    Ok(user_summary_from_row(&row))
}

pub async fn find_active_user_by_email_with_role(
    pool: &PgPool,
    email: &str,
) -> Result<Option<AuthUserRecord>, AuthRepositoryError> {
    let row = sqlx::query(
        r#"
        SELECT users.id, users.name, users.email, users.password_hash,
               roles.code AS role_code, roles.name AS role_name
        FROM users
        JOIN roles ON roles.id = users.role_id
        WHERE lower(users.email) = lower($1)
          AND users.status = 'active'
        LIMIT 1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| AuthUserRecord {
        id: row.get("id"),
        name: row.get("name"),
        email: row.get("email"),
        password_hash: row.get("password_hash"),
        role_code: row.get("role_code"),
        role_name: row.get("role_name"),
    }))
}

pub async fn create_session(
    pool: &PgPool,
    user_id: uuid::Uuid,
    session_hash: &str,
    expires_at: DateTime<Utc>,
) -> Result<SessionRecord, AuthRepositoryError> {
    let row = sqlx::query(
        r#"
        INSERT INTO sessions (user_id, session_hash, expires_at)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, session_hash, expires_at, revoked_at
        "#,
    )
    .bind(user_id)
    .bind(session_hash)
    .bind(expires_at)
    .fetch_one(pool)
    .await?;

    Ok(SessionRecord {
        id: row.get("id"),
        user_id: row.get("user_id"),
        session_hash: row.get("session_hash"),
        expires_at: row.get("expires_at"),
        revoked_at: row.get("revoked_at"),
    })
}

pub async fn revoke_session_by_hash(
    pool: &PgPool,
    session_hash: &str,
) -> Result<bool, AuthRepositoryError> {
    let result = sqlx::query(
        r#"
        UPDATE sessions
        SET revoked_at = COALESCE(revoked_at, now())
        WHERE session_hash = $1
          AND revoked_at IS NULL
        "#,
    )
    .bind(session_hash)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_current_session_by_hash(
    pool: &PgPool,
    session_hash: &str,
) -> Result<Option<CurrentSessionRecord>, AuthRepositoryError> {
    let row = sqlx::query(
        r#"
        UPDATE sessions
        SET last_seen_at = now()
        FROM users
        JOIN roles ON roles.id = users.role_id
        WHERE sessions.user_id = users.id
          AND sessions.session_hash = $1
          AND sessions.revoked_at IS NULL
          AND sessions.expires_at > now()
          AND users.status = 'active'
        RETURNING sessions.id AS session_id, sessions.expires_at AS session_expires_at,
                  users.id AS user_id, users.name, users.email,
                  roles.code AS role_code, roles.name AS role_name
        "#,
    )
    .bind(session_hash)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| CurrentSessionRecord {
        session_id: row.get("session_id"),
        session_expires_at: row.get("session_expires_at"),
        user_id: row.get("user_id"),
        name: row.get("name"),
        email: row.get("email"),
        role_code: row.get("role_code"),
        role_name: row.get("role_name"),
    }))
}

fn user_summary_from_row(row: &sqlx::postgres::PgRow) -> UserSummaryRecord {
    UserSummaryRecord {
        id: row.get("id"),
        name: row.get("name"),
        email: row.get("email"),
        status: row.get("status"),
        role_code: row.get("role_code"),
        role_name: row.get("role_name"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires TEST_DATABASE_URL pointing at an isolated disposable PostgreSQL database"]
    async fn session_lookup_ignores_revoked_sessions() {
        let database_url =
            std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url)
            .await
            .expect("test database must be reachable");

        crate::run_core_migrations(&pool)
            .await
            .expect("core migrations must apply");

        let email = format!("session-{}@example.com", uuid::Uuid::new_v4());
        let user_id: uuid::Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO users (role_id, name, email, password_hash, status)
            SELECT id, 'Session User', $1, '$argon2id$v=19$m=19456,t=2,p=1$fake$hash', 'active'
            FROM roles WHERE code = 'seller'
            RETURNING users.id
            "#,
        )
        .bind(&email)
        .fetch_one(&pool)
        .await
        .expect("test user inserted");

        let hash = format!("hash-{}", uuid::Uuid::new_v4());
        create_session(
            &pool,
            user_id,
            &hash,
            Utc::now() + chrono::Duration::days(7),
        )
        .await
        .expect("session inserted");
        assert!(get_current_session_by_hash(&pool, &hash)
            .await
            .expect("lookup succeeds")
            .is_some());

        assert!(revoke_session_by_hash(&pool, &hash)
            .await
            .expect("revoke succeeds"));
        assert!(get_current_session_by_hash(&pool, &hash)
            .await
            .expect("lookup succeeds")
            .is_none());
    }
}
