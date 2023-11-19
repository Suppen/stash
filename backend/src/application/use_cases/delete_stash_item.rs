use uuid::Uuid;

use crate::domain::{errors::ProductRepositoryError, value_objects::ProductId};

pub trait DeleteStashItem {
    /// Delete a stash item from a product.
    ///
    /// # Parameters
    /// - `product_id` - The product id.
    /// - `stash_item_id` - The stash item id.
    ///
    /// # Returns
    /// Nothing if successful, otherwise an error is returned.
    fn delete_stash_item(
        &self,
        product_id: &ProductId,
        stash_item_id: &Uuid,
    ) -> Result<(), ProductRepositoryError>;
}
