use crate::domain::user::ports::AuthRepositoryPort;
use redis::aio::ConnectionManager;

pub struct RedisAuthRepository;
impl AuthRepositoryPort for RedisAuthRepository {}
