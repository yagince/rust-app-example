use crate::domain::{
    repository::user_repository::UserRepository,
    user::{NewUser, User, UserId},
};

use super::RdbRepository;

#[async_trait::async_trait]
impl UserRepository for RdbRepository {
    async fn get_users(&self) -> anyhow::Result<Vec<User>> {
        Err(anyhow::anyhow!(""))
    }
    async fn get_user(&self, id: &UserId) -> anyhow::Result<Option<User>> {
        Err(anyhow::anyhow!(""))
    }
    async fn create_user(&self, user: NewUser) -> anyhow::Result<User> {
        Err(anyhow::anyhow!(""))
    }
}
