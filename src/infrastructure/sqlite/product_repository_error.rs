use crate::domain::{brand::BrandError, product::ProductIdError};

/// The error type that is returned by the repository
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
