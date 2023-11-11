use crate::domain::stash_item::StashItem;

pub trait SaveStashItem<E> {
    /// Save a stash item.
    fn save_stash_item(&self, stash_item: StashItem) -> Result<(), E>;
}
