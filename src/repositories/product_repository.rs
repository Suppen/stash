use crate::domain::product::{Product, ProductId};

pub trait ProductRepository {
    /// The error type that is returned by the repository
    type Error;

    /// Returns one product by id
    fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, Self::Error>;

    /// Saves a product to the repository, or updates it if it already exists
    fn save(&self, product: &Product) -> Result<(), Self::Error>;
}
