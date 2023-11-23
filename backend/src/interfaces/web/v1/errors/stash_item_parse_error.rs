use crate::domain::errors::{ProductIdError, QuantityError};

/// Errors that can occur when parsing a StashItem from a StashItemDTO
#[derive(Debug, PartialEq, Eq)]
pub enum StashItemParseError {
    /// Parsing the ID failed
    IdError(uuid::Error),
    /// Parsing the product ID failed
    ProductIdError(ProductIdError),
    /// Parsing the quantity failed
    QuantityError(QuantityError),
    /// Parsing the expiry date failed
    ExpiryDateError(chrono::ParseError),
}

impl std::fmt::Display for StashItemParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdError(error) => error.fmt(f),
            Self::ProductIdError(error) => error.fmt(f),
            Self::QuantityError(error) => error.fmt(f),
            Self::ExpiryDateError(error) => write!(f, "Expiry date error: {}", error),
        }
    }
}

impl std::error::Error for StashItemParseError {}

impl From<uuid::Error> for StashItemParseError {
    fn from(error: uuid::Error) -> Self {
        Self::IdError(error)
    }
}

impl From<ProductIdError> for StashItemParseError {
    fn from(error: ProductIdError) -> Self {
        Self::ProductIdError(error)
    }
}

impl From<QuantityError> for StashItemParseError {
    fn from(error: QuantityError) -> Self {
        Self::QuantityError(error)
    }
}

impl From<chrono::ParseError> for StashItemParseError {
    fn from(error: chrono::ParseError) -> Self {
        Self::ExpiryDateError(error)
    }
}
