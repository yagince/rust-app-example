use crate::domain::{repository::user_repository::UserRepository, user::User};

pub async fn get_users(repo: &impl UserRepository) -> anyhow::Result<Vec<User>> {
    repo.get_users().await
}
