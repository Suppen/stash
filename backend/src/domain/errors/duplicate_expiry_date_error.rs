/// Error signalling that a stash item with the same expiry date already exists for a product
#[derive(Debug, PartialEq, Eq)]
pub struct DuplicateExpiryDateError;

impl std::fmt::Display for DuplicateExpiryDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Duplicate expiry date")
    }
}

impl std::error::Error for DuplicateExpiryDateError {}
