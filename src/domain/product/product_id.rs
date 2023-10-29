use crate::domain::value_object::ValueObject;

/// ID of a product
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductId(String);

/// Possible errors when creating a product ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProductIdError {
    /// The ID cannot be empty
    Empty,
}

impl ValueObject<String> for ProductId {
    type Error = ProductIdError;

    fn new(value: String) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            Err(ProductIdError::Empty)
        } else {
            Ok(ProductId(value))
        }
    }

    fn value(&self) -> &String {
        &self.0
    }
}

impl std::fmt::Display for ProductId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let product_id = ProductId::new(String::from("ID")).unwrap();

        assert_eq!(product_id.value(), "ID");
    }

    #[test]
    fn test_new_empty() {
        let product_id = ProductId::new(String::from(""));

        assert!(product_id.is_err());
    }

    #[test]
    fn test_as_str() {
        let product_id = ProductId::new(String::from("ID")).unwrap();

        assert_eq!(product_id.value(), "ID");
    }
}
