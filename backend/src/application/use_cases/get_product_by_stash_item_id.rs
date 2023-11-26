use uuid::Uuid;

use crate::domain::{entities::Product, errors::ProductRepositoryError};

pub trait GetProductByStashItemId {
    /// Get a product by the ID of a stash item that belongs to it
    ///
    /// # Parameters
    /// - `stash_item_id` - ID of the stash item
    ///
    /// # Returns
    /// The product that the stash item belongs to, if it exists
    fn get_product_by_stash_item_id(
        &self,
        stash_item_id: &Uuid,
    ) -> Result<Option<Product>, ProductRepositoryError>;
}
