use super::{BrandError, ProductIdError, QuantityError};

/// Error type for ProductRepository
#[derive(Debug, PartialEq, Eq)]
pub enum ProductRepositoryError {
    /// Error related to ProductId
    ProductIdError(ProductIdError),
    /// Error related to stash item ID
    StashItemIdError(uuid::Error),
    /// Error related to Brand
    BrandError(BrandError),
    /// Error related to quantity
    QuantityError(QuantityError),
    /// Error related to expiry date
    ExpiryDateError(chrono::ParseError),
    /// Error related to the implementation of the repository
    PersisteneError(String),
}

impl std::fmt::Display for ProductRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductRepositoryError::ProductIdError(error) => error.fmt(f),
            ProductRepositoryError::StashItemIdError(error) => error.fmt(f),
            ProductRepositoryError::BrandError(error) => error.fmt(f),
            ProductRepositoryError::QuantityError(error) => error.fmt(f),
            ProductRepositoryError::ExpiryDateError(error) => {
                write!(f, "Expiry date error: {}", error)
            }
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

impl From<uuid::Error> for ProductRepositoryError {
    fn from(error: uuid::Error) -> Self {
        Self::StashItemIdError(error)
    }
}

impl From<BrandError> for ProductRepositoryError {
    fn from(error: BrandError) -> Self {
        Self::BrandError(error)
    }
}

impl From<QuantityError> for ProductRepositoryError {
    fn from(error: QuantityError) -> Self {
        Self::QuantityError(error)
    }
}

impl From<chrono::ParseError> for ProductRepositoryError {
    fn from(error: chrono::ParseError) -> Self {
        Self::ExpiryDateError(error)
    }
}
