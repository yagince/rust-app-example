use rand::random;

use crate::domain::{
    repository::user_repository::UserRepository,
    user::{NewUser, User, UserId},
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
            .filter(|x| x.id == *id)
            .cloned()
            .take(1)
            .collect::<Vec<_>>()
            .pop())
    }

    async fn create_user(&self, user: NewUser) -> anyhow::Result<User> {
        let user = User {
            id: UserId(random::<u32>() as i64),
            name: user.name,
            age: user.age,
        };
        self.users.push(user.clone());

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::user::{NewUser, UserId};
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]
    async fn test_get_users() -> anyhow::Result<()> {
        let users = vec![User {
            id: UserId(100),
            name: "Name".into(),
            age: 100,
        }];
        let mut repo = OnMemoryRepository {
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
        let mut repo = OnMemoryRepository {
            users: users.clone(),
        };

        let res = repo.get_user(&users[0].id).await?;

        assert_eq!(res, users.first().cloned());

        Ok(())
    }

    #[tokio::test]
    async fn test_create_user() -> anyhow::Result<()> {
        let mut repo = OnMemoryRepository::new();

        let user = repo
            .create_user(NewUser {
                name: "Name".into(),
                age: 100,
            })
            .await?;

        assert_matches!(user, User { id: UserId(id), .. } => {
            assert!(id >= 0);
        });

        Ok(())
    }
}
