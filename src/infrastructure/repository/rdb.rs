pub mod schema;
pub mod user;

use std::sync::Arc;

use crate::config::CONFIG;
use crate::domain::repository::user_repository::UserRepository;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

type ConnectionManagerPg = ConnectionManager<PgConnection>;
type DbPool = r2d2::Pool<ConnectionManagerPg>;
type DbCon = r2d2::PooledConnection<ConnectionManagerPg>;

pub static DB_POOL: Lazy<DbPool> = Lazy::new(|| create_db_pool());

pub struct RdbRepository {}

pub fn create_db_pool() -> DbPool {
    r2d2::Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(
            CONFIG.database_url(),
        ))
        .expect("failed to create db connection pool")
}
