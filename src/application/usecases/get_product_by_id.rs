use crate::domain::product::{Product, ProductId};

pub trait GetProductById<E> {
    /// Gets a product by id
    fn get_product_by_id(&self, id: &ProductId) -> Result<Option<Product>, E>;
}
