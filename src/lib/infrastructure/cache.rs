use redis::aio::ConnectionManager;

pub async fn init_redis() -> anyhow::Result<ConnectionManager> {
    Ok(ConnectionManager::new(redis::Client::open("")?).await?)
}
