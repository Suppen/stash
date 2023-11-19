use crate::domain::{
    entities::StashItem, errors::ProductRepositoryError, value_objects::ProductId,
};

pub trait AddStashItem {
    /// Add a stash item to a product.
    ///
    /// # Parameters
    /// - `product_id` - The product id.
    /// - `stash_item` - The stash item to add.
    ///
    /// # Returns
    /// Nothing if successful, otherwise an error is returned.
    fn add_stash_item(
        &self,
        product_id: &ProductId,
        stash_item: StashItem,
    ) -> Result<(), ProductRepositoryError>;
}
