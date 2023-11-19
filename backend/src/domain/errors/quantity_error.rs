/// Errors that can occur when creating a quantity
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantityError {
    /// A quantity can not be zero
    ZeroError,
}

impl std::error::Error for QuantityError {}

impl std::fmt::Display for QuantityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuantityError::ZeroError => write!(f, "Quantity can not be zero"),
        }
    }
}
