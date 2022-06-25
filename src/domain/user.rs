use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UserId(pub i64);

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub age: u32,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

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
}
