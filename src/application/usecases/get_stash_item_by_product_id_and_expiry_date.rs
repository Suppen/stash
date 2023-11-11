use chrono::NaiveDate;

use crate::domain::{product::ProductId, stash_item::StashItem};

pub trait GetStashItemByProductIdAndExpiryDate<E> {
    /// Get a stash item by its product id and expiry date.
    fn get_stash_item_by_product_id_and_expiry_date(
        &self,
        product_id: &ProductId,
        expiry_date: &NaiveDate,
    ) -> Result<Option<StashItem>, E>;
}
