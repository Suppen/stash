use crate::domain::{entities::StashItem, errors::StashItemRepositoryError};

pub trait SaveStashItem {
    /// Save a stash item. This creates a new stash item if it does not exist, or updates an existing one.
    ///
    /// # Parameters
    /// * `stash_item` - The stash item to save.
    ///
    /// # Returns
    /// * `Ok(())` if the stash item was saved.
    /// * `Err(_)` if the underlying data store fails to save the stash item.
    fn save_stash_item(&self, stash_item: StashItem) -> Result<(), StashItemRepositoryError>;
}
