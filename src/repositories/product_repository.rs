use crate::domain::product::{Product, ProductId};

#[cfg_attr(test, mockall::automock(type E=TestError;))]
pub trait ProductRepository<E> {
    /// Returns one product by id
    ///
    /// # Parameters
    /// * `id` - The id of the product to get
    ///
    /// # Returns
    /// * `Ok(Some(product))` if the product was found
    /// * `Ok(None)` if the product was not found
    /// * `Err(_)` if the repository fails to get the product
    fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, E>;

    /// Saves a product to the repository, or updates it if it already exists
    ///
    /// # Parameters
    /// * `product` - The product to save
    ///
    /// # Returns
    /// * `Ok(())` if the product was saved
    /// * `Err(_)` if the repository fails to save the product
    fn save(&self, product: Product) -> Result<(), E>;

    /// Deletes a product by id
    ///
    /// # Parameters
    /// * `id` - The id of the product to delete
    ///
    /// # Returns
    /// * `Ok(())` if the product was deleted, or was not there in the first place
    /// * `Err(_)` if the repository fails to delete the product
    fn delete_by_id(&self, id: &ProductId) -> Result<(), E>;
}
