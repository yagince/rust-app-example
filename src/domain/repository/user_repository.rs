use crate::domain::user::{User, UserId};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn get_users(&self) -> anyhow::Result<Vec<User>>;
    async fn get_user(&self, id: &UserId) -> anyhow::Result<Option<User>>;
}
