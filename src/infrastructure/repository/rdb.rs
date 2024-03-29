use std::time::Duration;

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};

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

pub async fn create_connection() -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(CONFIG.database_url());
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true)
        .connect_timeout(Duration::from_secs(100))
        .idle_timeout(Duration::from_secs(300));

    Ok(Database::connect(opt).await?)
}

#[cfg(test)]
pub(crate) mod fixtures {
    use super::entity::users;
    use sea_orm::ActiveValue;

    #[macro_export]
    macro_rules! fixture {
        ($conn:ident, $model:expr) => {
            $model.save($conn).await
        };
    }

    pub fn user() -> users::ActiveModel {
        users::ActiveModel {
            name: ActiveValue::Set("test name".into()),
            age: ActiveValue::Set(Some(100)),
            ..Default::default()
        }
    }
}
