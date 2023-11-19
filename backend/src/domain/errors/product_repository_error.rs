use super::{
    BrandError, DuplicateExpiryDateError, ProductIdError, QuantityError, StashItemDoesntExistError,
    StashItemExistsError,
};

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
    /// Error signalling that a product with the same expiry date already exists
    DuplicateExpiryDateError,
    /// Product already exists
    ProductAlreadyExists,
    /// Product not found
    ProductNotFound,
    /// The stash item already exists
    StashItemExists,
    /// The stash item does not exist
    StashItemNotFound,
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
            ProductRepositoryError::DuplicateExpiryDateError => {
                write!(f, "Duplicate expiry date")
            }
            ProductRepositoryError::ProductAlreadyExists => write!(f, "Product already exists"),
            ProductRepositoryError::ProductNotFound => write!(f, "Product not found"),
            ProductRepositoryError::StashItemExists => write!(f, "Stash item already exists"),
            ProductRepositoryError::StashItemNotFound => write!(f, "Stash item not found"),
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

impl From<DuplicateExpiryDateError> for ProductRepositoryError {
    fn from(_: DuplicateExpiryDateError) -> Self {
        Self::DuplicateExpiryDateError
    }
}

impl From<StashItemExistsError> for ProductRepositoryError {
    fn from(_: StashItemExistsError) -> Self {
        Self::StashItemExists
    }
}

impl From<StashItemDoesntExistError> for ProductRepositoryError {
    fn from(_: StashItemDoesntExistError) -> Self {
        Self::StashItemNotFound
    }
}
