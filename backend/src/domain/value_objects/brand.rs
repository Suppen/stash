use std::{ops::Deref, str::FromStr};

use crate::domain::errors::BrandError;

/// Brand of a product
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Brand(String);

/// The brand of a product
impl Brand {
    /// Create a new brand
    ///
    /// # Parameters
    /// * `value` - The value of the brand
    ///
    /// # Returns
    /// `Ok(Self)` if the brand could successfully be created
    /// `Err(BrandError::EmptyError)` if the value is empty
    pub fn new(value: String) -> Result<Self, BrandError> {
        if value.is_empty() {
            Err(BrandError::EmptyStringError)
        } else {
            Ok(Self(value))
        }
    }

    /// Get the value of the brand
    ///
    /// # Returns
    /// The value of the brand
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl Deref for Brand {
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

        assert!(matches!(brand, Err(BrandError::EmptyStringError)));
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
