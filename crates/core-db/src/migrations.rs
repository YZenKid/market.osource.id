use sqlx::{PgPool, Row};

pub const CORE_MIGRATION_LOCK_ID: i64 = 7_310_202_605;
pub const PACKAGE_MIGRATION_LOCK_ID: i64 = 7_310_202_606;

pub fn core_migrations_path() -> &'static str {
    "../../migrations/core"
}

pub fn package_migrations_path() -> &'static str {
    "../../migrations/packages"
}

pub async fn run_core_migrations(pool: &PgPool) -> anyhow::Result<()> {
    let mut connection = pool.acquire().await?;
    run_with_advisory_lock(&mut connection, CORE_MIGRATION_LOCK_ID, || async {
        sqlx::migrate!("../../migrations/core").run(pool).await?;
        Ok(())
    })
    .await?;
    Ok(())
}

pub async fn run_package_migrations(pool: &PgPool) -> anyhow::Result<()> {
    let mut connection = pool.acquire().await?;
    run_with_advisory_lock(&mut connection, PACKAGE_MIGRATION_LOCK_ID, || async {
        sqlx::query(
            r#"
            INSERT INTO package_migrations (package_id, package_version, migration_version, checksum)
            VALUES ('core.package-boundary', '0.1.0', '0001_package_boundary', 'placeholder')
            ON CONFLICT (package_id, migration_version) DO NOTHING
            "#,
        )
        .execute(pool)
        .await?;

        Ok::<(), anyhow::Error>(())
    })
    .await?;
    Ok(())
}

async fn run_with_advisory_lock<F, Fut>(
    connection: &mut sqlx::pool::PoolConnection<sqlx::Postgres>,
    lock_id: i64,
    operation: F,
) -> anyhow::Result<()>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<()>>,
{
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(lock_id)
        .execute(connection.as_mut())
        .await?;

    let operation_result = operation().await;

    let unlock_result: Result<bool, sqlx::Error> =
        sqlx::query_scalar("SELECT pg_advisory_unlock($1)")
            .bind(lock_id)
            .fetch_one(connection.as_mut())
            .await;

    operation_result?;
    match unlock_result {
        Ok(true) => {}
        Ok(false) => anyhow::bail!("migration advisory lock {lock_id} was not held by session"),
        Err(error) => return Err(error.into()),
    }
    Ok(())
}

pub async fn database_ready(pool: &PgPool) -> bool {
    sqlx::query("SELECT 1")
        .map(|row: sqlx::postgres::PgRow| row.get::<i32, _>(0))
        .fetch_one(pool)
        .await
        .is_ok()
}

pub async fn migration_table_ready(pool: &PgPool) -> bool {
    sqlx::query("SELECT version FROM _sqlx_migrations LIMIT 1")
        .execute(pool)
        .await
        .is_ok()
}

pub async fn install_state_ready(pool: &PgPool) -> bool {
    sqlx::query("SELECT 1 FROM installation_state WHERE state IN ('installed', 'locked') LIMIT 1")
        .execute(pool)
        .await
        .is_ok_and(|result| result.rows_affected() > 0)
}

pub async fn package_compatibility_ready(pool: &PgPool) -> bool {
    sqlx::query("SELECT package_id, version, core_version_range, enabled FROM packages LIMIT 1")
        .execute(pool)
        .await
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migration_lock_ids_are_distinct() {
        assert_ne!(CORE_MIGRATION_LOCK_ID, PACKAGE_MIGRATION_LOCK_ID);
    }

    #[test]
    fn migration_paths_are_separate() {
        assert!(core_migrations_path().ends_with("migrations/core"));
        assert!(package_migrations_path().ends_with("migrations/packages"));
        assert_ne!(core_migrations_path(), package_migrations_path());
    }

    fn test_database_url() -> Option<String> {
        std::env::var("TEST_DATABASE_URL").ok()
    }

    fn live_migration_test_lock() -> &'static tokio::sync::Mutex<()> {
        static LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
        LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
    }

    #[tokio::test]
    #[ignore = "requires TEST_DATABASE_URL pointing at an isolated disposable PostgreSQL database"]
    async fn live_migrations_apply_to_fresh_database() {
        let _guard = live_migration_test_lock().lock().await;
        let database_url = test_database_url().expect("TEST_DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url)
            .await
            .expect("test database must be reachable");

        run_core_migrations(&pool)
            .await
            .expect("core migrations must apply");
        run_package_migrations(&pool)
            .await
            .expect("package migrations must apply");

        assert!(database_ready(&pool).await);
        assert!(migration_table_ready(&pool).await);
        assert!(package_compatibility_ready(&pool).await);
        assert!(!install_state_ready(&pool).await);
    }

    #[tokio::test]
    #[ignore = "requires TEST_DATABASE_URL pointing at an isolated disposable PostgreSQL database"]
    async fn advisory_lock_prevents_concurrent_migration_runner() {
        let _guard = live_migration_test_lock().lock().await;
        let database_url = test_database_url().expect("TEST_DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url)
            .await
            .expect("test database must be reachable");

        sqlx::query("SELECT pg_advisory_lock($1)")
            .bind(CORE_MIGRATION_LOCK_ID)
            .execute(&pool)
            .await
            .expect("first session can take migration lock");

        let second_pool = PgPool::connect(&database_url)
            .await
            .expect("second test database session must be reachable");
        let can_take_lock: bool = sqlx::query_scalar("SELECT pg_try_advisory_lock($1)")
            .bind(CORE_MIGRATION_LOCK_ID)
            .fetch_one(&second_pool)
            .await
            .expect("second session can test advisory lock");

        sqlx::query("SELECT pg_advisory_unlock($1)")
            .bind(CORE_MIGRATION_LOCK_ID)
            .execute(&pool)
            .await
            .expect("first session can release migration lock");

        assert!(!can_take_lock);
    }

    #[tokio::test]
    #[ignore = "requires TEST_DATABASE_URL pointing at an isolated disposable PostgreSQL database"]
    async fn core_migration_runner_releases_its_advisory_lock() {
        let _guard = live_migration_test_lock().lock().await;
        let database_url = test_database_url().expect("TEST_DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url)
            .await
            .expect("test database must be reachable");

        run_core_migrations(&pool)
            .await
            .expect("core migrations must apply through advisory-lock runner");

        let second_pool = PgPool::connect(&database_url)
            .await
            .expect("second test database session must be reachable");
        let can_take_lock: bool = sqlx::query_scalar("SELECT pg_try_advisory_lock($1)")
            .bind(CORE_MIGRATION_LOCK_ID)
            .fetch_one(&second_pool)
            .await
            .expect("second session can test released advisory lock");

        if can_take_lock {
            sqlx::query("SELECT pg_advisory_unlock($1)")
                .bind(CORE_MIGRATION_LOCK_ID)
                .execute(&second_pool)
                .await
                .expect("second session can release test lock");
        }

        assert!(can_take_lock);
    }
}
