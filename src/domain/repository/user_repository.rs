use crate::domain::user::{NewUser, User, UserId};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn get_users(&self) -> anyhow::Result<Vec<User>>;
    async fn get_user(&self, id: &UserId) -> anyhow::Result<Option<User>>;
    async fn create_user(&mut self, user: &NewUser) -> anyhow::Result<User>;
}
