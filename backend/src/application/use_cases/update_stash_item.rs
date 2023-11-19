use crate::domain::{
    entities::StashItem, errors::ProductRepositoryError, value_objects::ProductId,
};

pub trait UpdateStashItem {
    /// Update a stash item in a product.
    ///
    /// # Parameters
    /// - `product_id` - The product id.
    /// - `stash_item` - The stash item to update.
    ///
    /// # Returns
    /// Nothing if successful, otherwise an error is returned.
    /// If the stash item does not exist, a `ProductRepositoryError::StashItemNotFound` is returned.
    fn update_stash_item(
        &self,
        product_id: &ProductId,
        stash_item: StashItem,
    ) -> Result<(), ProductRepositoryError>;
}
