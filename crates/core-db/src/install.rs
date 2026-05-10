use sqlx::{PgPool, Postgres, Row, Transaction};

pub const INSTALL_BOOTSTRAP_LOCK_ID: i64 = 7_310_202_607;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct InstallationStateRecord {
    pub state: String,
    pub runtime_mode: String,
    pub core_version: String,
    pub installed: bool,
    pub locked: bool,
}

#[derive(Debug, Clone)]
pub struct BootstrapInstallInput {
    pub marketplace_name: String,
    pub base_url: String,
    pub admin_name: String,
    pub admin_email: String,
    pub admin_password_hash: String,
    pub runtime_mode: String,
    pub core_version: String,
    pub storage_path: String,
}

#[derive(Debug, thiserror::Error)]
pub enum InstallRepositoryError {
    #[error("database error")]
    Database(#[from] sqlx::Error),
    #[error("installation is already locked")]
    AlreadyLocked,
}

pub async fn get_installation_state(
    pool: &PgPool,
) -> Result<Option<InstallationStateRecord>, InstallRepositoryError> {
    let row = sqlx::query(
        r#"
        SELECT state, runtime_mode, core_version, installed_at, locked_at
        FROM installation_state
        ORDER BY updated_at DESC
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| InstallationStateRecord {
        state: row.get("state"),
        runtime_mode: row.get("runtime_mode"),
        core_version: row.get("core_version"),
        installed: row
            .try_get::<Option<chrono::DateTime<chrono::Utc>>, _>("installed_at")
            .ok()
            .flatten()
            .is_some(),
        locked: row
            .try_get::<Option<chrono::DateTime<chrono::Utc>>, _>("locked_at")
            .ok()
            .flatten()
            .is_some(),
    }))
}

pub async fn bootstrap_installation(
    pool: &PgPool,
    input: BootstrapInstallInput,
) -> Result<InstallationStateRecord, InstallRepositoryError> {
    let mut transaction = pool.begin().await?;

    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(INSTALL_BOOTSTRAP_LOCK_ID)
        .execute(&mut *transaction)
        .await?;

    if installation_locked_for_update(&mut transaction).await? {
        return Err(InstallRepositoryError::AlreadyLocked);
    }

    let role_id: uuid::Uuid = sqlx::query_scalar("SELECT id FROM roles WHERE code = 'super_admin'")
        .fetch_one(&mut *transaction)
        .await?;

    sqlx::query(
        r#"
        INSERT INTO users (role_id, name, email, password_hash, status)
        VALUES ($1, $2, lower($3), $4, 'active')
        "#,
    )
    .bind(role_id)
    .bind(&input.admin_name)
    .bind(&input.admin_email)
    .bind(&input.admin_password_hash)
    .execute(&mut *transaction)
    .await?;

    sqlx::query("DELETE FROM marketplace_settings")
        .execute(&mut *transaction)
        .await?;
    sqlx::query(
        r#"
        INSERT INTO marketplace_settings (marketplace_name, base_url)
        VALUES ($1, $2)
        "#,
    )
    .bind(&input.marketplace_name)
    .bind(&input.base_url)
    .execute(&mut *transaction)
    .await?;

    sqlx::query("DELETE FROM storage_settings")
        .execute(&mut *transaction)
        .await?;
    sqlx::query(
        r#"
        INSERT INTO storage_settings (active_provider, local_storage_path)
        VALUES ('local', $1)
        "#,
    )
    .bind(&input.storage_path)
    .execute(&mut *transaction)
    .await?;

    sqlx::query("DELETE FROM tunnel_settings")
        .execute(&mut *transaction)
        .await?;
    sqlx::query("INSERT INTO tunnel_settings (enabled, provider) VALUES (false, 'cloudflared')")
        .execute(&mut *transaction)
        .await?;

    let row = sqlx::query(
        r#"
        INSERT INTO installation_state (state, runtime_mode, core_version, installed_at, locked_at)
        VALUES ('locked', $1, $2, now(), now())
        RETURNING state, runtime_mode, core_version, installed_at, locked_at
        "#,
    )
    .bind(&input.runtime_mode)
    .bind(&input.core_version)
    .fetch_one(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(InstallationStateRecord {
        state: row.get("state"),
        runtime_mode: row.get("runtime_mode"),
        core_version: row.get("core_version"),
        installed: true,
        locked: true,
    })
}

async fn installation_locked_for_update(
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT locked_at
        FROM installation_state
        ORDER BY updated_at DESC
        LIMIT 1
        FOR UPDATE
        "#,
    )
    .fetch_optional(&mut **transaction)
    .await?;

    Ok(row
        .and_then(|row| {
            row.try_get::<Option<chrono::DateTime<chrono::Utc>>, _>("locked_at")
                .ok()
        })
        .flatten()
        .is_some())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires TEST_DATABASE_URL pointing at an isolated disposable PostgreSQL database"]
    async fn bootstrap_installation_locks_repeat_setup() {
        let database_url =
            std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url)
            .await
            .expect("test database must be reachable");

        crate::run_core_migrations(&pool)
            .await
            .expect("core migrations must apply");
        reset_install_test_data(&pool).await;

        let input = BootstrapInstallInput {
            marketplace_name: "Market Test".to_string(),
            base_url: "http://127.0.0.1:8080".to_string(),
            admin_name: "Admin".to_string(),
            admin_email: format!("admin-{}@example.com", uuid::Uuid::new_v4()),
            admin_password_hash: "$argon2id$v=19$m=19456,t=2,p=1$fake$hash".to_string(),
            runtime_mode: "vps".to_string(),
            core_version: "0.1.0".to_string(),
            storage_path: "/tmp/market-osource-test".to_string(),
        };

        let record = bootstrap_installation(&pool, input.clone())
            .await
            .expect("first setup succeeds");
        assert!(record.locked);

        let repeat = bootstrap_installation(&pool, input).await;
        assert!(matches!(repeat, Err(InstallRepositoryError::AlreadyLocked)));
    }

    #[tokio::test]
    #[ignore = "requires TEST_DATABASE_URL pointing at an isolated disposable PostgreSQL database"]
    async fn concurrent_bootstrap_allows_only_one_first_admin() {
        let database_url =
            std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url)
            .await
            .expect("test database must be reachable");

        crate::run_core_migrations(&pool)
            .await
            .expect("core migrations must apply");
        reset_install_test_data(&pool).await;

        let first_input = BootstrapInstallInput {
            marketplace_name: "Market Test".to_string(),
            base_url: "http://127.0.0.1:8080".to_string(),
            admin_name: "Admin One".to_string(),
            admin_email: format!("admin-one-{}@example.com", uuid::Uuid::new_v4()),
            admin_password_hash: "$argon2id$v=19$m=19456,t=2,p=1$fake$hash".to_string(),
            runtime_mode: "vps".to_string(),
            core_version: "0.1.0".to_string(),
            storage_path: "/tmp/market-osource-test".to_string(),
        };
        let mut second_input = first_input.clone();
        second_input.admin_name = "Admin Two".to_string();
        second_input.admin_email = format!("admin-two-{}@example.com", uuid::Uuid::new_v4());

        let (first, second) = tokio::join!(
            bootstrap_installation(&pool, first_input),
            bootstrap_installation(&pool, second_input)
        );

        let success_count = usize::from(first.is_ok()) + usize::from(second.is_ok());
        let locked_count = usize::from(matches!(first, Err(InstallRepositoryError::AlreadyLocked)))
            + usize::from(matches!(second, Err(InstallRepositoryError::AlreadyLocked)));
        assert_eq!(success_count, 1);
        assert_eq!(locked_count, 1);

        let super_admin_count: i64 = sqlx::query_scalar(
            r#"
            SELECT count(*)
            FROM users
            JOIN roles ON users.role_id = roles.id
            WHERE roles.code = 'super_admin'
            "#,
        )
        .fetch_one(&pool)
        .await
        .expect("super admin count query succeeds");
        assert_eq!(super_admin_count, 1);
    }

    async fn reset_install_test_data(pool: &PgPool) {
        for query in [
            "DELETE FROM sessions",
            "DELETE FROM users",
            "DELETE FROM installation_state",
            "DELETE FROM marketplace_settings",
            "DELETE FROM storage_settings",
            "DELETE FROM tunnel_settings",
        ] {
            sqlx::query(query)
                .execute(pool)
                .await
                .expect("test data reset succeeds");
        }
    }
}
