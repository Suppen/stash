use std::collections::HashSet;

use crate::domain::{
    entities::StashItem, errors::ProductRepositoryError, value_objects::ProductId,
};

pub trait GetStashItems {
    /// Get all stash items for a product.
    ///
    /// # Parameters
    /// - `product_id` - The product id.
    ///
    /// # Returns
    /// The stash items if successful, otherwise an error is returned.
    fn get_stash_items(
        &self,
        product_id: &ProductId,
    ) -> Result<HashSet<StashItem>, ProductRepositoryError>;
}
