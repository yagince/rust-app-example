use std::net::SocketAddr;

use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

use crate::infrastructure::repository::rdb::create_connection;

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
}

pub mod api;

pub async fn serve() -> anyhow::Result<()> {
    let db_conn = create_connection().await?;

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], 3000)))
        .serve(api::api(AppState { db_conn }).await?.into_make_service())
        .await?;
    Ok(())
}
