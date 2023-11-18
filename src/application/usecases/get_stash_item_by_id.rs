use uuid::Uuid;

use crate::domain::{entities::StashItem, errors::StashItemRepositoryError};

pub trait GetStashItemById {
    /// Get a stash item by its id
    ///
    /// # Parameters
    /// * `id` - The id of the stash item to get.
    ///
    /// # Returns
    /// `Ok(Some(stash_item))` if the stash item exists
    /// `Ok(None)` if the stash item does not exist
    /// `Err(_)` if the underlying data store fails to get the stash item
    fn get_stash_item_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<StashItem>, StashItemRepositoryError>;
}
