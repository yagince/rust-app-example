use validator::Validate;

use crate::domain::{
    repository::user_repository::UserRepository,
    user::{NewUser, User},
};

pub struct CreateUser<'a, R: UserRepository> {
    repo: &'a R,
}

impl<'a, R: UserRepository> CreateUser<'a, R> {
    pub fn new(repo: &'a R) -> Self {
        Self { repo }
    }

    pub async fn run(&self, user: NewUser) -> anyhow::Result<User> {
        user.validate()?;
        self.repo.create_user(user).await
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use mockall::predicate::eq;
    use validator::ValidationErrors;

    use crate::domain::{repository::user_repository::MockUserRepository, user::UserId};

    use super::*;

    #[tokio::test]
    async fn test_create_user() -> anyhow::Result<()> {
        let new_user = NewUser {
            name: "TestName".into(),
            age: 99,
        };

        let mut repo = MockUserRepository::new();
        repo.expect_create_user()
            .with(eq(new_user.clone()))
            .returning(|x| {
                Ok(User {
                    id: UserId(100),
                    name: x.name,
                    age: x.age,
                })
            });

        let usecase = CreateUser::new(&repo);
        let user = usecase.run(new_user).await?;

        assert_matches!(user, User { id, ..} => {
            assert_eq!(id, UserId(100));
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_create_user_if_validation_error() -> anyhow::Result<()> {
        let new_user = NewUser {
            name: "".into(),
            age: 99,
        };

        let mut repo = MockUserRepository::new();
        repo.expect_create_user()
            .with(eq(new_user.clone()))
            .returning(|x| {
                Ok(User {
                    id: UserId(100),
                    name: x.name,
                    age: x.age,
                })
            });

        let usecase = CreateUser::new(&repo);
        let res = usecase.run(new_user).await;

        assert_matches!(res, Err(e) => {
            match e.downcast::<ValidationErrors>() {
                Ok(e) => assert!(ValidationErrors::has_error(&Err(e), "name")),
                Err(e) => panic!("Not ValidationErrors: {:?}", e),
            }
        });

        Ok(())
    }
}
