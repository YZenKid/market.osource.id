use sqlx::{PgPool, Row};

pub async fn run_core_migrations(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(7_310_202_605_i64)
        .execute(pool)
        .await?;
    let result = sqlx::migrate!("../../migrations/core").run(pool).await;
    sqlx::query("SELECT pg_advisory_unlock($1)")
        .bind(7_310_202_605_i64)
        .execute(pool)
        .await?;
    result?;
    Ok(())
}

pub async fn run_package_migrations(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(7_310_202_606_i64)
        .execute(pool)
        .await?;
    let result = sqlx::migrate!("../../migrations/packages").run(pool).await;
    sqlx::query("SELECT pg_advisory_unlock($1)")
        .bind(7_310_202_606_i64)
        .execute(pool)
        .await?;
    result?;
    Ok(())
}

pub async fn database_ready(pool: &PgPool) -> bool {
    sqlx::query("SELECT 1")
        .map(|row: sqlx::postgres::PgRow| row.get::<i32, _>(0))
        .fetch_one(pool)
        .await
        .is_ok()
}
