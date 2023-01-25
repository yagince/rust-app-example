use std::net::SocketAddr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Json, Router as AxumRouter,
};
use sea_orm::DatabaseConnection;

use crate::{
    domain::{repository::user_repository::UserRepository, user::UserId},
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
    Router::new()
        .route("/users", routing::get(get_users))
        .route("/users/:id", routing::get(get_user))
}

async fn get_users(State(conn): State<DatabaseConnection>) -> impl IntoResponse {
    let repo = RdbRepository::new(&conn);
    users::get_users(&repo)
        .await
        .map(|users| (StatusCode::OK, Json(users)))
        .map_err(internal_error)
}

async fn get_user(
    State(conn): State<DatabaseConnection>,
    Path(user_id): Path<i64>,
) -> impl IntoResponse {
    let repo = RdbRepository::new(&conn);
    repo.get_user(&UserId(user_id))
        .await
        .map(|x| (StatusCode::OK, Json(x)))
        .map_err(internal_error)
}

fn internal_error(err: anyhow::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        fixture,
        infrastructure::repository::rdb::{
            entity::{self, users},
            fixtures,
        },
    };
    use anyhow::Context;
    use assert_json_diff::assert_json_include;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use pretty_assertions::assert_eq;
    use sea_orm::{ActiveModelTrait, EntityTrait};
    use serde_json::json;
    use tower::ServiceExt;

    macro_rules! parse_json {
        ($res:expr) => {
            serde_json::from_slice::<serde_json::Value>(
                &hyper::body::to_bytes($res.into_body()).await?,
            )?
        };
    }

    async fn connection() -> anyhow::Result<(DatabaseConnection, AppState)> {
        let conn = create_connection().await?;

        let state = AppState {
            db_conn: conn.clone(),
        };
        Ok((conn, state))
    }

    async fn fixture_user(conn: &DatabaseConnection) -> anyhow::Result<users::ActiveModel> {
        Ok(fixture!(
            conn,
            users::ActiveModel {
                name: sea_orm::ActiveValue::Set("name".into()),
                ..fixtures::user()
            }
        )?)
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn test_api_v1_get_users() -> anyhow::Result<()> {
        let (conn, state) = connection().await?;

        let x = fixture_user(&conn).await.context("create user")?;

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

        let body = parse_json!(res);

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

    #[serial_test::serial]
    #[tokio::test]
    async fn test_api_v1_get_user() -> anyhow::Result<()> {
        let (conn, state) = connection().await?;

        let x = fixture_user(&conn).await.context("create user")?;

        let res: anyhow::Result<_> = async {
            let app = api(state).await?;
            Ok(app
                .oneshot(
                    Request::builder()
                        .method(axum::http::Method::GET)
                        .uri(format!("/api/v1/users/{}", x.id.clone().unwrap()))
                        .body(Body::empty())?,
                )
                .await?)
        }
        .await;

        entity::prelude::Users::delete(x.clone())
            .exec(&conn)
            .await?;

        let res = res?;

        assert_eq!(res.status(), StatusCode::OK);

        let body = parse_json!(res);

        assert_json_include!(
            actual: body,
            expected: json!({
                "name": x.name.unwrap(),
                "age": x.age.unwrap(),
            }),
        );
        Ok(())
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn test_api_v1_get_user_400() -> anyhow::Result<()> {
        let (_, state) = connection().await?;

        let res: anyhow::Result<_> = async {
            let app = api(state).await?;
            Ok(app
                .oneshot(
                    Request::builder()
                        .method(axum::http::Method::GET)
                        .uri("/api/v1/users/id")
                        .body(Body::empty())?,
                )
                .await?)
        }
        .await;

        let res = res?;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        Ok(())
    }
}
