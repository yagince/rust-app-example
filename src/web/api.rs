use std::net::SocketAddr;

use axum::{
    extract::State, http::StatusCode, response::IntoResponse, routing, Json, Router as AxumRouter,
};
use sea_orm::DatabaseConnection;

use crate::{
    infrastructure::repository::rdb::{create_connection, RdbRepository},
    interface::controller::users,
};

use super::AppState;

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
    users::get_users(&repo)
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
    use crate::infrastructure::repository::rdb::entity::users;
    use anyhow::Context;
    use assert_json_diff::assert_json_include;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use pretty_assertions::assert_eq;
    use sea_orm::ActiveModelTrait;
    use serde_json::json;

    #[tokio::test]
    async fn test_api_v1_get_users() -> anyhow::Result<()> {
        let conn = create_connection().await?;

        let state = AppState {
            db_conn: conn.clone(),
        };

        let x = users::ActiveModel {
            name: sea_orm::ActiveValue::Set("name".into()),
            age: sea_orm::ActiveValue::Set(Some(100)),
            ..Default::default()
        }
        .save(&conn)
        .await
        .context("insert fixture")?;

        let res: anyhow::Result<_> = async {
            use tower::ServiceExt;
            let app = api(state).await?;
            Ok(app
                .oneshot(
                    Request::builder()
                        .method(axum::http::Method::GET)
                        .uri("/api/v1/users")
                        .body(Body::empty())?,
                )
                .await?)
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
