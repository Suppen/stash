use uuid::Uuid;

use crate::domain::stash_item::StashItem;

pub trait GetStashItemById<E> {
    /// Get a stash item by its id.
    fn get_stash_item_by_id(&self, id: &Uuid) -> Result<Option<StashItem>, E>;
}
