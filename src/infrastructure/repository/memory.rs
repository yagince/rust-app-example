use crate::domain::{
    repository::user_repository::UserRepository,
    user::{User, UserId},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct OnMemoryRepository {
    users: Vec<User>,
}

impl OnMemoryRepository {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait::async_trait]
impl UserRepository for OnMemoryRepository {
    async fn get_users(&self) -> anyhow::Result<Vec<User>> {
        Ok(self.users.clone())
    }

    async fn get_user(&self, id: &UserId) -> anyhow::Result<Option<User>> {
        Ok(self
            .users
            .iter()
            .filter(|x| dbg!(x).id == *id)
            .cloned()
            .take(1)
            .collect::<Vec<_>>()
            .pop())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::domain::user::UserId;

    use super::*;

    #[tokio::test]
    async fn test_get_users() -> anyhow::Result<()> {
        let users = vec![User {
            id: UserId(100),
            name: "Name".into(),
            age: 100,
        }];
        let repo = OnMemoryRepository {
            users: users.clone(),
        };

        let res = repo.get_users().await?;

        assert_eq!(res, users);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_user() -> anyhow::Result<()> {
        let users = vec![
            User {
                id: UserId(10),
                name: "Name".into(),
                age: 100,
            },
            User {
                id: UserId(10),
                name: "Name 2".into(),
                age: 100,
            },
        ];
        let repo = OnMemoryRepository {
            users: users.clone(),
        };

        let res = repo.get_user(&users[0].id).await?;

        assert_eq!(res, users.first().cloned());

        Ok(())
    }
}
