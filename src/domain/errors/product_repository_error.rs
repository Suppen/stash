use super::{BrandError, ProductIdError};

/// Error type for ProductRepository
#[derive(Debug, PartialEq, Eq)]
pub enum ProductRepositoryError {
    /// Error related to ProductId
    ProductIdError(ProductIdError),
    /// Error related to Brand
    BrandError(BrandError),
    /// Error related to the implementation of the repository
    PersisteneError(String),
}

impl std::fmt::Display for ProductRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductRepositoryError::ProductIdError(error) => error.fmt(f),
            ProductRepositoryError::BrandError(error) => error.fmt(f),
            ProductRepositoryError::PersisteneError(error) => write!(f, "{}", error),
        }
    }
}

impl std::error::Error for ProductRepositoryError {}

impl From<ProductIdError> for ProductRepositoryError {
    fn from(error: ProductIdError) -> Self {
        Self::ProductIdError(error)
    }
}

impl From<BrandError> for ProductRepositoryError {
    fn from(error: BrandError) -> Self {
        Self::BrandError(error)
    }
}
