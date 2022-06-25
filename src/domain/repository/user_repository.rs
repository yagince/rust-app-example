use crate::domain::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn get_users() -> anyhow::Result<User>;
}
