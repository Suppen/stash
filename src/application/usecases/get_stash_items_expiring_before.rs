use chrono::NaiveDate;

use crate::domain::stash_item::StashItem;

pub trait GetStashItemsExpiringBefore<E> {
    /// Get all stash items expiring before a given date, excluding the given date.
    fn get_stash_items_expiring_before(&self, date: &NaiveDate) -> Result<Vec<StashItem>, E>;
}
