use std::net::SocketAddr;

use axum::{
    extract::{FromRef, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Json, Router as AxumRouter,
};
use sea_orm::DatabaseConnection;

use crate::{
    domain::repository::user_repository::UserRepository,
    infrastructure::repository::rdb::{create_connection, RdbRepository},
};

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
}

type Router = AxumRouter<AppState>;

pub async fn serve() -> anyhow::Result<()> {
    let db_conn = create_connection().await?;

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], 3000)))
        .serve(api(AppState { db_conn }).await?.into_make_service())
        .await?;
    Ok(())
}

pub async fn api(state: AppState) -> anyhow::Result<AxumRouter> {
    Ok(Router::new().nest("/api", v1()).with_state(state))
}

fn v1() -> Router {
    Router::new().nest("/v1", v1_routes())
}

fn v1_routes() -> Router {
    Router::new().route("/users", routing::get(get_users))
}

async fn get_users(State(conn): State<DatabaseConnection>) -> impl IntoResponse {
    let repo = RdbRepository::new(&conn);
    repo.get_users()
        .await
        .map(|users| (StatusCode::OK, Json(users)))
        .map_err(internal_error)
}

fn internal_error(err: anyhow::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;
    use assert_json_diff::assert_json_include;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use pretty_assertions::assert_eq;
    use sea_orm::ActiveModelTrait;
    use serde_json::json;
    use std::net::{SocketAddr, TcpListener};

    #[tokio::test]
    async fn test_api_v1_get_users() -> anyhow::Result<()> {
        let conn = create_connection().await?;

        let state = AppState {
            db_conn: conn.clone(),
        };

        let x = crate::infrastructure::repository::rdb::entity::users::ActiveModel {
            name: sea_orm::ActiveValue::Set("name".into()),
            age: sea_orm::ActiveValue::Set(Some(100)),
            ..Default::default()
        }
        .save(&conn)
        .await
        .context("insert fixture")?;

        let res: anyhow::Result<_> = async {
            let listner = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>()?)?;
            let addr = listner.local_addr()?;

            tokio::spawn(async move {
                axum::Server::from_tcp(listner)
                    .unwrap()
                    .serve(api(state).await.unwrap().into_make_service())
                    .await
                    .unwrap();
            });
            let client = hyper::Client::new();
            client
                .request(
                    Request::builder()
                        .uri(format!("http://{}/api/v1/users", addr))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .map_err(Into::into)
        }
        .await;

        x.delete(&conn).await?;
        let res = res?;

        assert_eq!(res.status(), StatusCode::OK);

        let body: serde_json::Value =
            serde_json::from_slice(&hyper::body::to_bytes(res.into_body()).await?)?;

        assert_json_include!(
            actual: body,
            expected:
                json!([
                    {
                        "name": "name",
                        "age": 100,
                    },
                ]),
        );
        Ok(())
    }
}
