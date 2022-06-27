use crate::domain::user::{NewUser, User, UserId};
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_users(&self) -> anyhow::Result<Vec<User>>;
    async fn get_user(&self, id: &UserId) -> anyhow::Result<Option<User>>;
    async fn create_user(&self, user: NewUser) -> anyhow::Result<User>;
}
