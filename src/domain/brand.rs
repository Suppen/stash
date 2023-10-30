/// Brand of a product
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Brand(String);

/// Possible errors when creating a brand
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BrandError {
    /// The brand cannot be empty
    Empty,
}

impl Brand {
    pub fn new(value: String) -> Result<Self, BrandError> {
        if value.is_empty() {
            Err(BrandError::Empty)
        } else {
            Ok(Self(value))
        }
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

impl std::fmt::Display for Brand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let brand = Brand::new(String::from("Brand")).unwrap();

        assert_eq!(brand.to_string(), "Brand");
    }

    #[test]
    fn test_new_empty() {
        let brand = Brand::new(String::from(""));

        assert!(brand.is_err());
    }

    #[test]
    fn test_value() {
        let str = String::from("Brand");
        let brand = Brand::new(str.clone()).unwrap();

        assert_eq!(brand.value(), &str);
    }

    #[test]
    fn test_to_string() {
        let brand = Brand::new(String::from("Brand")).unwrap();

        assert_eq!(brand.to_string(), "Brand");
    }
}
