use uuid::Uuid;

use crate::domain::{entities::Product, errors::ProductRepositoryError, value_objects::ProductId};

#[cfg_attr(test, mockall::automock)]
pub trait ProductRepository: Sync + Send {
    /// Returns one product by id
    ///
    /// # Parameters
    /// * `id` - The id of the product to get
    ///
    /// # Returns
    /// * `Ok(Some(product))` if the product was found
    /// * `Ok(None)` if the product was not found
    /// * `Err(_)` if the repository fails to get the product
    fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, ProductRepositoryError>;

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
