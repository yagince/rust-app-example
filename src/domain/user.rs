use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UserId(pub i64);

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
pub struct User {
    pub id: UserId,
    #[validate(length(min = 1))]
    pub name: String,
    pub age: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 1))]
    pub name: String,
    pub age: u32,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use serde_json::json;
    use validator::ValidationErrors;

    use super::*;

    impl Default for User {
        fn default() -> Self {
            Self {
                id: UserId(0),
                name: Default::default(),
                age: 0,
            }
        }
    }

    #[test]
    fn test_deserialize_from_json() -> anyhow::Result<()> {
        let data = json!({
            "id": 1234567890,
            "name": "Name Name",
            "age": 100,
        })
        .to_string();

        let user: User = serde_json::from_str(&data)?;

        assert_eq!(
            user,
            User {
                id: UserId(1234567890),
                name: "Name Name".into(),
                age: 100,
            }
        );

        Ok(())
    }

    #[rstest]
    #[case("", true)]
    #[case("a", false)]
    fn test_validate_name(#[case] name: &str, #[case] has_error: bool) {
        let user = User {
            name: name.into(),
            ..Default::default()
        };

        let res = user.validate();

        assert_eq!(ValidationErrors::has_error(&res, "name"), has_error);
    }
}
