use sqlx::postgres::PgPool;

pub async fn init_pool() -> anyhow::Result<PgPool> {
    Ok(PgPool::connect("").await?)
}
