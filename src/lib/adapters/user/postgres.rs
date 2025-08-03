use crate::domain::user::ports::UserRepositoryPort;
use sqlx::PgPool;

pub struct SQLxUserRepository;
impl UserRepositoryPort for SQLxUserRepository {}
