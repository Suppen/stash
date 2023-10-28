#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StashItemId(String);

impl StashItemId {
    pub fn new(id: &str) -> Result<StashItemId, String> {
        if id.len() == 0 {
            Err("Product id cannot be empty".to_string())
        } else {
            Ok(StashItemId(id.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let stash_item_id = StashItemId::new("ID").unwrap();

        assert_eq!(stash_item_id.as_str(), "ID");
    }

    #[test]
    fn test_new_empty() {
        let stash_item_id = StashItemId::new("");

        assert!(stash_item_id.is_err());
    }

    #[test]
    fn test_as_str() {
        let stash_item_id = StashItemId::new("ID").unwrap();

        assert_eq!(stash_item_id.as_str(), "ID");
    }
}
