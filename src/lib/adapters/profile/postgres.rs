use crate::domain::profile::ports::ProfileRepositoryPort;
use sqlx::PgPool;

pub struct SQLxProfileRepository;
impl ProfileRepositoryPort for SQLxProfileRepository {}
