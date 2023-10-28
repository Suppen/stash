#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductId(String);

impl ProductId {
    pub fn new(id: &str) -> Result<ProductId, String> {
        if id.len() == 0 {
            Err("Product id cannot be empty".to_string())
        } else {
            Ok(ProductId(id.to_string()))
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
        let product_id = ProductId::new("ID").unwrap();

        assert_eq!(product_id.as_str(), "ID");
    }

    #[test]
    fn test_new_empty() {
        let product_id = ProductId::new("");

        assert!(product_id.is_err());
    }

    #[test]
    fn test_as_str() {
        let product_id = ProductId::new("ID").unwrap();

        assert_eq!(product_id.as_str(), "ID");
    }
}
