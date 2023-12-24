use crate::domain::{entities::Product, errors::ProductRepositoryError};

pub trait GetAllProductsWithStashItems {
    /// Gets all products with stash items
    fn get_all_products_with_stash_items(&self) -> Result<Vec<Product>, ProductRepositoryError>;
}
