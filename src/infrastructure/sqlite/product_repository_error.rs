use crate::domain::{brand::BrandError, product::ProductIdError};

/// The error type that is returned by the product repository
#[derive(Debug)]
pub enum ProductRepositoryError {
    /// The product id is invalid
    InvalidProductId(ProductIdError),
    /// The brand is invalid
    InvalidBrand(BrandError),
    /// The database returned an error
    DatabaseError(rusqlite::Error),
}

impl From<ProductIdError> for ProductRepositoryError {
    fn from(error: ProductIdError) -> Self {
        Self::InvalidProductId(error)
    }
}

impl From<BrandError> for ProductRepositoryError {
    fn from(error: BrandError) -> Self {
        Self::InvalidBrand(error)
    }
}

impl From<rusqlite::Error> for ProductRepositoryError {
    fn from(error: rusqlite::Error) -> Self {
        Self::DatabaseError(error)
    }
}

impl std::error::Error for ProductRepositoryError {}

impl std::fmt::Display for ProductRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductRepositoryError::InvalidProductId(error) => write!(f, "{}", error),
            ProductRepositoryError::InvalidBrand(error) => write!(f, "{}", error),
            ProductRepositoryError::DatabaseError(error) => write!(f, "{}", error),
        }
    }
}
