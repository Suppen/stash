use crate::domain::{product::ProductIdError, stash_item::StashItemIdError};

/// The error type that is returned by the stash item repository
#[derive(Debug)]
pub enum StashItemRepositoryError {
    /// The stash item id is invalid
    InvalidStashItemId(StashItemIdError),
    /// The product id is invalid
    InvalidProductId(ProductIdError),
    /// The database returned an error
    DatabaseError(rusqlite::Error),
}

impl From<StashItemIdError> for StashItemRepositoryError {
    fn from(error: StashItemIdError) -> Self {
        Self::InvalidStashItemId(error)
    }
}

impl From<ProductIdError> for StashItemRepositoryError {
    fn from(error: ProductIdError) -> Self {
        Self::InvalidProductId(error)
    }
}

impl From<rusqlite::Error> for StashItemRepositoryError {
    fn from(error: rusqlite::Error) -> Self {
        Self::DatabaseError(error)
    }
}
