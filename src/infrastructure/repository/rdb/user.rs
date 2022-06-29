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
        Err(anyhow::anyhow!(""))
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
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;
    use sea_orm::{ActiveModelTrait, TransactionTrait};

    use crate::infrastructure::repository::rdb::{entity, get_connection, DB};

    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let db = DB.get_or_try_init(get_connection).await?;

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
}
