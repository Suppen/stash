use crate::domain::{entities::Product, errors::ProductRepositoryError};

pub trait CreateProduct {
    /// Creates a new product
    ///
    /// # Parameters
    /// - `product` - The product to create
    ///
    /// # Returns
    /// `Ok(ProductId)` if the product was created successfully
    /// `Err(String)` if the product could not be created
    fn create_product(&self, product: Product) -> Result<Product, ProductRepositoryError>;
}
