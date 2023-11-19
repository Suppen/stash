/// Error returned when a stash item already exists.
#[derive(Debug, PartialEq, Eq)]
pub struct StashItemExistsError;

impl std::fmt::Display for StashItemExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stash item already exists")
    }
}

impl std::error::Error for StashItemExistsError {}
