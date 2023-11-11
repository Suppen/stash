use crate::domain::product::Product;

pub trait SaveProduct<E> {
    /// Saves a product
    fn save_product(&self, product: Product) -> Result<(), E>;
}
