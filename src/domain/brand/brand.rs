use crate::domain::value_object::ValueObject;

/// Brand of a product
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Brand(String);

impl ValueObject<String> for Brand {
    fn new(value: String) -> Result<Self, Box<dyn std::error::Error>> {
        if value.is_empty() {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Brand cannot be empty",
            )))
        } else {
            Ok(Self(value))
        }
    }

    fn value(&self) -> &String {
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

        assert_eq!(brand.value(), "Brand");
    }

    #[test]
    fn test_new_empty() {
        let brand = Brand::new(String::from(""));

        assert!(brand.is_err());
    }

    #[test]
    fn test_as_str() {
        let brand = Brand::new(String::from("Brand")).unwrap();

        assert_eq!(brand.value(), "Brand");
    }
}
