use crate::domain::product::{Product, ProductId};

#[cfg_attr(test, mockall::automock(type E=TestError;))]
pub trait ProductRepository<E> {
    /// Returns one product by id
    fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, E>;

    /// Saves a product to the repository, or updates it if it already exists
    fn save(&self, product: Product) -> Result<(), E>;

    /// Deletes a product by id
    fn delete_by_id(&self, id: &ProductId) -> Result<(), E>;
}
