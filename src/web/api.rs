use std::net::SocketAddr;

use axum::{
    extract::{FromRef, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Json, Router as AxumRouter,
};
use sea_orm::DatabaseConnection;

use crate::{
    domain::user::{User, UserId},
    infrastructure::repository::rdb::create_connection,
};

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
}

type Router = AxumRouter<AppState>;

pub async fn serve() -> anyhow::Result<()> {
    // let db_conn = create_connection().await?;
    // let api = Router::new()
    //     .nest(
    //         "/api",
    //         Router::new().nest(
    //             "/v1",
    //             Router::new().route("/users", routing::get(get_users)),
    //         ),
    //     )
    //     .with_state(AppState { db_conn });
    let db_conn = create_connection().await?;

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], 3000)))
        .serve(api(AppState { db_conn }).await?.into_make_service())
        // .serve(api.into_make_service())
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

async fn get_users(State(_conn): State<DatabaseConnection>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(vec![User {
            id: UserId(1),
            name: "test name".to_owned(),
            age: 100,
        }]),
    )
}

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, TcpListener};

    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[tokio::test]
    async fn test_api_v1_get_users() -> anyhow::Result<()> {
        let state = AppState {
            db_conn: create_connection().await?,
        };
        let listner = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>()?)?;
        let addr = dbg!(listner.local_addr()?);

        tokio::spawn(async move {
            axum::Server::from_tcp(listner)
                .unwrap()
                .serve(api(state).await.unwrap().into_make_service())
                .await
                .unwrap();
        });
        let client = hyper::Client::new();
        let res = client
            .request(
                Request::builder()
                    .uri(format!("http://{}/api/v1/users", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await?;

        assert_eq!(res.status(), StatusCode::OK);

        let body: serde_json::Value =
            serde_json::from_slice(&hyper::body::to_bytes(res.into_body()).await?)?;
        assert_eq!(body, json!([{ "id": 1, "name": "test name", "age": 100}]));
        Ok(())
    }
}
