use uuid::Uuid;

use crate::domain::errors::StashItemRepositoryError;

pub trait DeleteStashItemById {
    /// Delete a stash item by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the stash item to delete.
    ///
    /// # Returns
    /// * `Ok(())` if the stash item was deleted successfully.
    /// * `Err(StashItemParseError)` if the stash item could not be deleted.
    fn delete_stash_item_by_id(&self, id: &Uuid) -> Result<(), StashItemRepositoryError>;
}
