/// Possible errors when creating a brand
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BrandError {
    /// The brand cannot be empty
    EmptyError,
}

impl std::fmt::Display for BrandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for BrandError {}
