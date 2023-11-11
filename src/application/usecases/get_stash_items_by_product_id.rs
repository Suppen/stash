use crate::domain::{product::ProductId, stash_item::StashItem};

pub trait GetStashItemsByProductId<E> {
    /// Get all stash items by product id.
    fn get_stash_items_by_product_id(&self, product_id: &ProductId) -> Result<Vec<StashItem>, E>;
}
