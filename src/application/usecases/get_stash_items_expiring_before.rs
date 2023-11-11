use chrono::NaiveDate;

use crate::domain::stash_item::StashItem;

pub trait GetStashItemsExpiringBefore<E> {
    /// Get all stash items expiring before a given date, excluding the given date.
    ///
    /// # Parameters
    /// * `date` - The date before which the stash items expire.
    ///
    /// # Returns
    /// `Ok(stash_items)` All stash items expiring before the given date.
    /// `Err(_)` if the underlying data store fails to get the stash items
    fn get_stash_items_expiring_before(&self, date: &NaiveDate) -> Result<Vec<StashItem>, E>;
}
