use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use tokio::sync::OnceCell;

use crate::config::CONFIG;

pub mod entity;
pub mod user;

pub struct RdbRepository<'a, C: ConnectionTrait> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> RdbRepository<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        Self { conn }
    }
}

pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_connection() -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(CONFIG.database_url());
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true);

    Ok(Database::connect(opt).await?)
}
