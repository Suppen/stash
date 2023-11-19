use super::{ProductIdError, QuantityError};

/// Error type for StashItemRepository
#[derive(Debug, PartialEq, Eq)]
pub enum StashItemRepositoryError {
    /// Error related to ID
    IdError(uuid::Error),
    /// Error related to ProductId
    ProductIdError(ProductIdError),
    /// Error related to Quantity
    QuantityError(QuantityError),
    /// Error related to the implementation of the repository
    PersistenceError(String),
    /// The product the stash item is an instance of does not exist
    ProductDoesNotExist,
}

impl std::fmt::Display for StashItemRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StashItemRepositoryError::IdError(error) => error.fmt(f),
            StashItemRepositoryError::ProductIdError(error) => error.fmt(f),
            StashItemRepositoryError::QuantityError(error) => error.fmt(f),
            StashItemRepositoryError::PersistenceError(error) => write!(f, "{}", error),
            StashItemRepositoryError::ProductDoesNotExist => {
                write!(f, "No product with the given ID exists")
            }
        }
    }
}

impl std::error::Error for StashItemRepositoryError {}

impl From<uuid::Error> for StashItemRepositoryError {
    fn from(error: uuid::Error) -> Self {
        Self::IdError(error)
    }
}

impl From<ProductIdError> for StashItemRepositoryError {
    fn from(error: ProductIdError) -> Self {
        Self::ProductIdError(error)
    }
}

impl From<QuantityError> for StashItemRepositoryError {
    fn from(error: QuantityError) -> Self {
        Self::QuantityError(error)
    }
}
