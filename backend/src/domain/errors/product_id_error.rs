/// Possible errors when creating a product ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProductIdError {
    /// The ID cannot be empty
    EmptyStringError,
}

impl std::error::Error for ProductIdError {}

impl std::fmt::Display for ProductIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductIdError::EmptyStringError => write!(f, "The product ID cannot be empty"),
        }
    }
}
