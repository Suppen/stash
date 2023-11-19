use crate::domain::{entities::Product, errors::ProductRepositoryError, value_objects::ProductId};

pub trait UpdateProductById {
    /// Updates a product by its ID
    ///
    /// # Parameters
    /// - `id` - The ID of the product to update
    /// - `product` - The product to update
    ///
    /// # Returns
    /// `Ok(())` if the product was updated successfully
    /// `Err(String)` if the product could not be updated
    fn update_product_by_id(
        &self,
        id: &ProductId,
        product: Product,
    ) -> Result<(), ProductRepositoryError>;
}
