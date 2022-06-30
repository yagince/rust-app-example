use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait};
use validator::Validate;

use crate::domain::{
    repository::user_repository::UserRepository,
    user::{NewUser, User, UserId},
};

use super::{
    entity::{self, users},
    RdbRepository,
};

#[async_trait::async_trait]
impl<'a, C: ConnectionTrait> UserRepository for RdbRepository<'a, C> {
    async fn get_users(&self) -> anyhow::Result<Vec<User>> {
        Ok(entity::prelude::Users::find()
            .all(__self.conn)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn get_user(&self, id: &UserId) -> anyhow::Result<Option<User>> {
        Ok(entity::prelude::Users::find_by_id(id.0)
            .one(__self.conn)
            .await?
            .map(Into::into))
    }

    async fn create_user(&self, user: NewUser) -> anyhow::Result<User> {
        user.validate()?;
        Ok(entity::users::ActiveModel {
            name: sea_orm::ActiveValue::Set(user.name),
            age: sea_orm::ActiveValue::Set(user.age.try_into().ok()),
            ..Default::default()
        }
        .insert(self.conn)
        .await?
        .into())
    }
}

impl From<users::Model> for User {
    fn from(x: users::Model) -> Self {
        Self {
            id: UserId(x.id),
            name: x.name,
            age: x
                .age
                .map(|x| x.try_into().unwrap_or_default())
                .unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;
    use sea_orm::{ActiveModelTrait, TransactionTrait};
    use validator::ValidationErrors;

    use crate::infrastructure::repository::rdb::{create_connection, entity};

    use super::*;

    macro_rules! transaction {
        () => {{
            create_connection()
                .await?
                .begin()
                .await
                .context("begin transaction")?
        }};
    }

    #[tokio::test]
    async fn test_get_users() -> anyhow::Result<()> {
        let tx = transaction!();

        entity::users::ActiveModel {
            name: sea_orm::ActiveValue::Set("name".into()),
            age: sea_orm::ActiveValue::Set(Some(100)),
            ..Default::default()
        }
        .save(&tx)
        .await
        .context("insert fixture")?;

        let repo = RdbRepository::new(&tx);

        let users = repo.get_users().await.context("get_users")?;

        assert_matches!(&users[..], [user] => {
            assert_matches!(user.id, UserId(x) => {
                assert!(x > 0);
            });
            assert_eq!(user.name, "name");
            assert_eq!(user.age, 100);
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_get_user_exists() -> anyhow::Result<()> {
        let tx = transaction!();

        let mut user = entity::users::ActiveModel {
            name: sea_orm::ActiveValue::Set("name".into()),
            age: sea_orm::ActiveValue::Set(Some(100)),
            ..Default::default()
        }
        .save(&tx)
        .await
        .context("insert fixture")?;

        let repo = RdbRepository::new(&tx);

        let user = repo
            .get_user(&UserId(user.id.take().unwrap()))
            .await
            .context("get_user")?;

        assert_matches!(user, Some(user) => {
            assert_matches!(user.id, UserId(x) if x > 0);
            assert_eq!(user.name, "name");
            assert_eq!(user.age, 100);
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_create_user() -> anyhow::Result<()> {
        let tx = transaction!();

        let repo = RdbRepository::new(&tx);

        let user = repo
            .create_user(NewUser {
                name: "name".into(),
                age: 100,
            })
            .await
            .context("create_user")?;

        assert_matches!(user, User { id: UserId(id), name, age} => {
            assert!(id > 0);
            assert_eq!(name, "name");
            assert_eq!(age, 100);
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_create_user_if_validation_error() -> anyhow::Result<()> {
        let tx = transaction!();

        let repo = RdbRepository::new(&tx);

        let res = repo
            .create_user(NewUser {
                name: "".into(),
                age: 100,
            })
            .await;

        assert_matches!(res, Err(e) => {
            match e.downcast::<ValidationErrors>() {
                Ok(e) => assert!(ValidationErrors::has_error(&Err(e), "name")),
                Err(e) => panic!("Not ValidationErrors: {:?}", e),
            }
        });

        Ok(())
    }
}
