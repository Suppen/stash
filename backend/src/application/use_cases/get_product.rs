use crate::domain::{entities::Product, errors::ProductRepositoryError, value_objects::ProductId};

pub trait GetProduct {
    /// Gets a product by id
    ///
    /// # Parameters
    /// * `id` - The id of the product to get
    ///
    /// # Returns
    /// * `Ok(Some(product))` if the product was found
    /// * `Ok(None)` if the product was not found
    /// * `Err(_)` if the underlying data store fails to get the product
    fn get_product(&self, id: &ProductId) -> Result<Option<Product>, ProductRepositoryError>;
}
