use std::str::FromStr;

use crate::domain::errors::ProductIdError;

/// ID of a product
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductId(String);

impl ProductId {
    /// Create a new product ID
    ///
    /// # Parameters
    /// - `value` - The value of the product ID
    ///
    /// # Errors
    /// - `ProductIdError::EmptyStringError` - The value is empty
    pub fn new(value: String) -> Result<Self, ProductIdError> {
        if value.len() == 0 {
            Err(ProductIdError::EmptyStringError)
        } else {
            Ok(ProductId(value))
        }
    }

    /// Create a random product ID, for testing purposes
    #[cfg(test)]
    pub fn random() -> Self {
        use rand::distributions::Alphanumeric;
        use rand::Rng;

        let rng = rand::thread_rng();
        let product_id = rng
            .sample_iter(Alphanumeric)
            .map(char::from)
            .take(10)
            .collect();
        Self(product_id)
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

impl std::ops::Deref for ProductId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for ProductId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl FromStr for ProductId {
    type Err = ProductIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let str = "ID";
        let product_id: ProductId = str.parse().unwrap();

        assert_eq!(product_id.value(), str);
    }

    #[test]
    fn test_new_empty() {
        let product_id = "".parse::<ProductId>();

        assert!(matches!(product_id, Err(ProductIdError::EmptyStringError)));
    }

    #[test]
    fn test_deref() {
        let id_str = "ID";
        let product_id: ProductId = id_str.parse().unwrap();

        let str: &str = &product_id;

        assert_eq!(str, id_str);
    }

    #[test]
    fn test_value() {
        let str = "ID";
        let product_id = str.parse::<ProductId>().unwrap();

        assert_eq!(product_id.value(), str);
    }

    #[test]
    fn test_to_string() {
        let str = "ID";
        let product_id: ProductId = str.parse().unwrap();

        assert_eq!(product_id.to_string(), str);
    }

    #[test]
    fn test_from_str() {
        let str = "ID";
        let product_id: ProductId = str.parse().unwrap();

        assert_eq!(product_id.value(), str);
    }
}
