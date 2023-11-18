use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::{product::ProductId, stash_item::StashItem};

#[cfg_attr(test, mockall::automock)]
pub trait StashItemRepository<E: std::error::Error + Send + Sync> {
    /// Returns one stash item by id
    ///
    /// # Parameters
    /// * `id` - The id of the stash item to get.
    ///
    /// # Returns
    /// `Ok(Some(stash_item))` if the stash item exists
    /// `Ok(None)` if the stash item does not exist
    /// `Err(_)` if the underlying data store fails to get the stash item
    fn find_by_id(&self, id: &Uuid) -> Result<Option<StashItem>, E>;

    /// Returns all stash items with the given product id
    ///
    /// # Parameters
    /// * `product_id` - The product id of the stash items.
    ///
    /// # Returns
    /// `Ok(stash_items)` All stash items with the given product id.
    /// `Err(_)` if the underlying data store fails to get the stash items
    fn find_all_by_product_id(&self, product_id: &ProductId) -> Result<Vec<StashItem>, E>;

    /// Returns one stash item by its unique combo of product id and expiry date
    ///
    /// # Parameters
    /// * `product_id` - The product id of the stash item.
    /// * `expiry_date` - The expiry date of the stash item.
    ///
    /// # Returns
    /// `Ok(Some(stash_item))` if the stash item exists
    /// `Ok(None)` if the stash item does not exist
    /// `Err(_)` if the underlying data store fails to get the stash item
    fn find_by_product_id_and_expiry_date(
        &self,
        product_id: &ProductId,
        expiry_date: &NaiveDate,
    ) -> Result<Option<StashItem>, E>;

    /// Returns all stash items expiring before a given date, excluding the given date
    ///
    /// # Parameters
    /// * `date` - The date before which the stash items expire, excluding the given date.
    ///
    /// # Returns
    /// `Ok(stash_items)` All stash items expiring before the given date.
    /// `Err(_)` if the underlying data store fails to get the stash items
    fn find_all_expiring_before(&self, date: &NaiveDate) -> Result<Vec<StashItem>, E>;

    /// Saves a stash item to the repository, or updates it if it already exists
    ///
    /// # Parameters
    /// * `stash_item` - The stash item to save
    ///
    /// # Returns
    /// * `Ok(())` if the stash item was saved
    /// * `Err(_)` if the repository fails to save the stash item
    fn save(&self, stash_item: StashItem) -> Result<(), E>;

    /// Deletes a stash item from the repository
    ///
    /// # Parameters
    /// * `id` - The id of the stash item to delete
    ///
    /// # Returns
    /// * `Ok(())` if the stash item was deleted, or was not there in the first place
    /// * `Err(_)` if the repository fails to delete the stash item
    fn delete(&self, id: &Uuid) -> Result<(), E>;
}
