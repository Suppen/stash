use std::str::FromStr;

use super::BrandError;

/// Brand of a product
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Brand(String);

impl Brand {
    pub fn new(value: String) -> Result<Self, BrandError> {
        if value.is_empty() {
            Err(BrandError::EmptyError)
        } else {
            Ok(Self(value))
        }
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

impl std::ops::Deref for Brand {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Brand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl FromStr for Brand {
    type Err = BrandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let brand: Brand = "Brand".parse().unwrap();

        assert_eq!(brand.to_string(), "Brand");
    }

    #[test]
    fn test_new_empty() {
        let brand = "".parse::<Brand>();

        assert!(matches!(brand, Err(BrandError::EmptyError)));
    }

    #[test]
    fn test_deref() {
        let brand_str = "Brand";
        let brand: Brand = brand_str.parse().unwrap();

        let str: &str = &brand;

        assert_eq!(str, brand_str);
    }

    #[test]
    fn test_value() {
        let str = "Brand";
        let brand: Brand = str.parse().unwrap();

        assert_eq!(brand.value(), &str);
    }

    #[test]
    fn test_to_string() {
        let str = "Brand";
        let brand: Brand = str.parse().unwrap();

        assert_eq!(brand.to_string(), str);
    }

    #[test]
    fn test_from_str() {
        let str = "Brand";
        let brand: Brand = str.parse().unwrap();

        assert_eq!(brand.to_string(), str);
    }
}
