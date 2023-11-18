use crate::domain::{
    entities::StashItem, errors::StashItemRepositoryError, value_objects::ProductId,
};

pub trait GetStashItemsByProductId {
    /// Get all stash items by product id.
    ///
    /// # Parameters
    /// * `product_id` - The product id of the stash items.
    ///
    /// # Returns
    /// `Ok(stash_items)` All stash items with the given product id.
    /// `Err(_)` if the underlying data store fails to get the stash items
    fn get_stash_items_by_product_id(
        &self,
        product_id: &ProductId,
    ) -> Result<Vec<StashItem>, StashItemRepositoryError>;
}
