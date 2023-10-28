use crate::domain::stash_item::{StashItem, StashItemId};

pub trait StashItemRepository {
    type Error;

    /// Returns one stash item by id
    fn find_by_id(&self, id: &StashItemId) -> Result<Option<StashItem>, Self::Error>;

    /// Saves a stash item to the repository, or updates it if it already exists
    fn save(&self, stash_item: StashItem) -> Result<(), Self::Error>;
}
