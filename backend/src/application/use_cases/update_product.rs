use crate::domain::{entities::Product, errors::ProductRepositoryError, value_objects::ProductId};

pub trait UpdateProduct {
    /// Updates a product by its ID
    ///
    /// # Parameters
    /// - `id` - The ID of the product to update
    /// - `product` - The product to update
    ///
    /// # Returns
    /// `Ok(Product)` if the product was updated
    /// `Err(String)` if the product could not be updated
    fn update_product(
        &self,
        id: &ProductId,
        product: Product,
    ) -> Result<Product, ProductRepositoryError>;
}
