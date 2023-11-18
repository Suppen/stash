use crate::domain::{errors::ProductRepositoryError, value_objects::ProductId};

pub trait DeleteProductById {
    /// Deletes a product by id
    ///
    /// # Parameters
    /// * `id` - The id of the product to delete
    ///
    /// # Returns
    /// * `Ok(())` if the product was deleted, or was not there in the first place
    /// * `Err(_)` if the underlying data store fails to delete the product
    fn delete_product_by_id(&self, id: &ProductId) -> Result<(), ProductRepositoryError>;
}
