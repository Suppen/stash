use uuid::Uuid;

use crate::domain::{product::ProductId, stash_item::StashItem};

pub trait StashItemRepository<E> {
    /// Returns one stash item by id
    fn find_by_id(&self, id: &Uuid) -> Result<Option<StashItem>, E>;

    /// Returns all stash items with the given product id
    fn find_all_by_product_id(&self, product_id: &ProductId) -> Result<Vec<StashItem>, E>;

    /// Returns one stash item by its unique combo of product id and expiry date
    fn find_by_product_id_and_expiry_date(
        &self,
        product_id: &ProductId,
        expiry_date: &chrono::NaiveDate,
    ) -> Result<Option<StashItem>, E>;

    /// Saves a stash item to the repository, or updates it if it already exists
    fn save(&self, stash_item: StashItem) -> Result<(), E>;

    /// Deletes a stash item from the repository
    fn delete(&self, id: &Uuid) -> Result<(), E>;
}
