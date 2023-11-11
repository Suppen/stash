use crate::domain::product::Product;

pub trait SaveProduct<E> {
    /// Saves a product
    ///
    /// # Parameters
    /// * `product` - The product to save
    ///
    /// # Returns
    /// * `Ok(())` if the product was saved
    /// * `Err(_)` if the underlying data store fails to save the product
    fn save_product(&self, product: Product) -> Result<(), E>;
}
