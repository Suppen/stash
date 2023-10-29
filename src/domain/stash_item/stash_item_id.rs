use crate::domain::value_object::ValueObject;

/// ID of a stash item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StashItemId(String);

/// Possible errors when creating a stash item ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StashItemIdError {
    /// The ID cannot be empty
    Empty,
}

impl ValueObject<String> for StashItemId {
    type Error = StashItemIdError;

    fn new(value: String) -> Result<StashItemId, Self::Error> {
        if value.len() == 0 {
            Err(StashItemIdError::Empty)
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
