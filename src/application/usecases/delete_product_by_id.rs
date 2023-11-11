use crate::domain::product::ProductId;

pub trait DeleteProductById<E> {
    /// Deletes a product by id
    fn delete_product_by_id(&self, id: &ProductId) -> Result<(), E>;
}
