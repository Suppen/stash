use crate::domain::value_object::ValueObject;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StashItemId(String);

impl ValueObject<String> for StashItemId {
    fn new(value: String) -> Result<StashItemId, Box<dyn std::error::Error>> {
        if value.len() == 0 {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Stash item ID cannot be empty",
            )))
        } else {
            Ok(StashItemId(value))
        }
    }

    fn value(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let stash_item_id = StashItemId::new(String::from("ID")).unwrap();

        assert_eq!(stash_item_id.value(), "ID");
    }

    #[test]
    fn test_new_empty() {
        let stash_item_id = StashItemId::new(String::from(""));

        assert!(stash_item_id.is_err());
    }

    #[test]
    fn test_as_str() {
        let stash_item_id = StashItemId::new(String::from("ID")).unwrap();

        assert_eq!(stash_item_id.value(), "ID");
    }
}
