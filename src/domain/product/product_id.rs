use std::str::FromStr;

/// ID of a product
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductId(String);

/// Possible errors when creating a product ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProductIdError {
    /// The ID cannot be empty
    Empty,
}

impl ProductId {
    pub fn new(value: String) -> Result<Self, ProductIdError> {
        if value.len() == 0 {
            Err(ProductIdError::Empty)
        } else {
            Ok(ProductId(value))
        }
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

        assert!(matches!(product_id, Err(ProductIdError::Empty)));
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
