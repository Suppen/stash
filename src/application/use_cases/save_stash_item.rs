use crate::domain::{
    entities::StashItem, errors::ProductRepositoryError, value_objects::ProductId,
};

pub trait SaveStashItem {
    /// Saves a stash item to a product
    ///
    /// # Parameters
    /// - `product_id` - The id of the product to save the stash item to
    /// - `stash_item` - The stash item to save
    ///
    /// # Returns
    /// `Ok(())` if the stash item was saved successfully
    /// `Err(String)` if the stash item could not be saved
    fn save_stash_item(
        &self,
        product_id: &ProductId,
        stash_item: StashItem,
    ) -> Result<(), ProductRepositoryError>;
}
