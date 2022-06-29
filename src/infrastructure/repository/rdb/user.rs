use sea_orm::{ConnectionTrait, EntityTrait};

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
        Err(anyhow::anyhow!(""))
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

    use crate::infrastructure::repository::rdb::{entity, get_connection};

    use super::*;

    #[tokio::test]
    async fn test_get_users() -> anyhow::Result<()> {
        let db = get_connection().await?;

        let tx = db.begin().await?;

        entity::users::ActiveModel {
            name: sea_orm::ActiveValue::Set("name".into()),
            age: sea_orm::ActiveValue::Set(Some(100)),
            ..Default::default()
        }
        .save(&tx)
        .await?;

        let repo = RdbRepository::new(&tx);

        let users = repo.get_users().await?;

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
        let db = get_connection().await?;

        let tx = db.begin().await.context("get conn")?;

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
            assert_matches!(user.id, UserId(x) => {
                assert!(x > 0);
            });
            assert_eq!(user.name, "name");
            assert_eq!(user.age, 100);
        });
        Ok(())
    }
}
