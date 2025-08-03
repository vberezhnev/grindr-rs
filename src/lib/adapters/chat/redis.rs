use crate::domain::chat::ports::ChatRepositoryPort;
use redis::aio::ConnectionManager;

pub struct RedisChatRepository;
impl ChatRepositoryPort for RedisChatRepository {}
