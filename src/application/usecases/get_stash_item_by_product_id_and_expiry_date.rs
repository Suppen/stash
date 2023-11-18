use chrono::NaiveDate;

use crate::domain::{entities::StashItem, value_objects::ProductId};

pub trait GetStashItemByProductIdAndExpiryDate<E> {
    /// Get a stash item by its product id and expiry date. This uniquely identifies a stash item,
    /// so there will only be one stash item returned.
    ///
    /// # Parameters
    /// * `product_id` - The product id of the stash item.
    /// * `expiry_date` - The expiry date of the stash item.
    ///
    /// # Returns
    /// `Ok(Some(stash_item))` if the stash item exists
    /// `Ok(None)` if the stash item does not exist
    /// `Err(_)` if the underlying data store fails to get the stash item
    fn get_stash_item_by_product_id_and_expiry_date(
        &self,
        product_id: &ProductId,
        expiry_date: &NaiveDate,
    ) -> Result<Option<StashItem>, E>;
}
