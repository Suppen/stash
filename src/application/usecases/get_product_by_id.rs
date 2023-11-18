use crate::domain::{entities::Product, value_objects::ProductId};

pub trait GetProductById<E> {
    /// Gets a product by id
    ///
    /// # Parameters
    /// * `id` - The id of the product to get
    ///
    /// # Returns
    /// * `Ok(Some(product))` if the product was found
    /// * `Ok(None)` if the product was not found
    /// * `Err(_)` if the underlying data store fails to get the product
    fn get_product_by_id(&self, id: &ProductId) -> Result<Option<Product>, E>;
}
