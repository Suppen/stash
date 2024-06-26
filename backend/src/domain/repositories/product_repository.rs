use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::{entities::Product, errors::ProductRepositoryError, value_objects::ProductId};

#[cfg_attr(test, mockall::automock)]
pub trait ProductRepository: Sync + Send {
    /// Gets all products with stash items
    ///
    /// # Returns
    /// * `Ok(products)` if the products were found
    /// * `Err(_)` if the repository fails to get the products
    fn find_all_with_stash_items(&self) -> Result<Vec<Product>, ProductRepositoryError>;

    /// Gets one product by id, if it exists
    ///
    /// # Parameters
    /// * `id` - The id of the product to get
    ///
    /// # Returns
    /// * `Ok(Some(product))` if the product was found
    /// * `Ok(None)` if the product was not found
    /// * `Err(_)` if the repository fails to get the product
    fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, ProductRepositoryError>;

    /// Gets a list of products by their ids
    ///
    /// # Parameters
    /// * `ids` - The ids of the products to get
    ///
    /// # Returns
    /// * `Ok(products)` if the products were found
    /// * `Err(_)` if the repository fails to get the products
    fn find_by_ids(&self, ids: &[ProductId]) -> Result<Vec<Product>, ProductRepositoryError>;

    /// Finds a product by the ID of a stash item that belongs to it
    ///
    /// # Parameters
    /// - `stash_item_id` - ID of the stash item
    ///
    /// # Returns
    /// The product that the stash item belongs to, if it exists
    fn find_by_stash_item_id(
        &self,
        stash_item_id: &Uuid,
    ) -> Result<Option<Product>, ProductRepositoryError>;

    /// Finds all products with at least one stash item expiring within the given date interval
    ///
    /// # Parameters
    /// - `after` - The start of the date range, inclusive
    /// - `before` - The end of the date range, exclusive
    ///
    /// # Returns
    /// A list of products with at least one stash item expiring within the given date interval
    fn find_expiring_in_interval(
        &self,
        after: Option<NaiveDate>,
        before: Option<NaiveDate>,
    ) -> Result<Vec<Product>, ProductRepositoryError>;

    /// Returns whether a product exists
    ///
    /// # Parameters
    /// * `id` - The id of the product to check
    ///
    /// # Returns
    /// * `Ok(true)` if the product exists
    /// * `Ok(false)` if the product does not exist
    /// * `Err(_)` if the repository fails to check the product
    fn exists_by_id(&self, id: &ProductId) -> Result<bool, ProductRepositoryError>;

    /// Saves a product to the repository, or updates it if it already exists
    ///
    /// # Parameters
    /// * `product` - The product to save
    ///
    /// # Returns
    /// * `Ok(())` if the product was saved
    /// * `Err(_)` if the repository fails to save the product
    fn save(&self, product: Product) -> Result<(), ProductRepositoryError>;

    /// Deletes a product by id
    ///
    /// # Parameters
    /// * `id` - The id of the product to delete
    ///
    /// # Returns
    /// * `Ok(())` if the product was deleted, or was not there in the first place
    /// * `Err(_)` if the repository fails to delete the product
    fn delete_by_id(&self, id: &ProductId) -> Result<(), ProductRepositoryError>;
}
