use crate::domain::errors::{BrandError, ProductIdError};

/// Errors that can occur when parsing a Product from a ProuctDTO
#[derive(Debug, PartialEq, Eq)]
pub enum ProductParseError {
    /// Parsing the product ID failed
    ProductIdError(ProductIdError),
    /// Parsing the brand failed
    BrandError(BrandError),
}

impl std::fmt::Display for ProductParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProductIdError(error) => error.fmt(f),
            Self::BrandError(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for ProductParseError {}

impl From<ProductIdError> for ProductParseError {
    fn from(error: ProductIdError) -> Self {
        Self::ProductIdError(error)
    }
}

impl From<BrandError> for ProductParseError {
    fn from(error: BrandError) -> Self {
        Self::BrandError(error)
    }
}
