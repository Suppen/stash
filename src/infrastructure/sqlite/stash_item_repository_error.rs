use crate::domain::{product::ProductIdError, quantity::QuantityError};

/// The error type that is returned by the stash item repository
#[derive(Debug)]
pub enum StashItemRepositoryError {
    /// The stash item id is invalid
    InvalidStashItemId(uuid::Error),
    /// The product id is invalid
    InvalidProductId(ProductIdError),
    /// The quantity is invalid
    InvalidQuantity(QuantityError),
    /// The database returned an error
    DatabaseError(rusqlite::Error),
}

impl From<uuid::Error> for StashItemRepositoryError {
    fn from(error: uuid::Error) -> Self {
        Self::InvalidStashItemId(error)
    }
}

impl From<ProductIdError> for StashItemRepositoryError {
    fn from(error: ProductIdError) -> Self {
        Self::InvalidProductId(error)
    }
}

impl From<QuantityError> for StashItemRepositoryError {
    fn from(error: QuantityError) -> Self {
        Self::InvalidQuantity(error)
    }
}

impl From<rusqlite::Error> for StashItemRepositoryError {
    fn from(error: rusqlite::Error) -> Self {
        Self::DatabaseError(error)
    }
}

impl std::error::Error for StashItemRepositoryError {}

impl std::fmt::Display for StashItemRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StashItemRepositoryError::InvalidStashItemId(error) => {
                write!(f, "Invalid stash item id: {}", error)
            }
            StashItemRepositoryError::InvalidProductId(error) => {
                write!(f, "Invalid product id: {}", error)
            }
            StashItemRepositoryError::InvalidQuantity(error) => {
                write!(f, "Invalid quantity: {}", error)
            }
            StashItemRepositoryError::DatabaseError(error) => {
                write!(f, "Database error: {}", error)
            }
        }
    }
}
