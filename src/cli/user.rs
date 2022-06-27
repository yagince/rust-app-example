use crate::{
    domain::user::{NewUser, User},
    infrastructure::repository::memory::OnMemoryRepository,
    usecase::user::create::CreateUser,
};

pub async fn create_user(name: String, age: u32) -> anyhow::Result<User> {
    let repo = OnMemoryRepository::new();
    CreateUser::new(&repo)
        .run(NewUser {
            name: name,
            age: age,
        })
        .await
}
