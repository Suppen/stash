use std::sync::{Arc, Mutex};

use crate::domain::product::{Product, ProductId};
use crate::repositories::ProductRepository as ProductRepositoryTrait;
use rusqlite::{named_params, params, Connection, OptionalExtension};

/// A repository for [`Product`]s using SQLite as the underlying storage.
pub struct ProductRepository {
    // The connection to the database
    connection: Arc<Mutex<Connection>>,
}

impl ProductRepository {
    /// Creates a new [`ProductRepository`]. Creates the table in the database if it doesn't exist
    ///
    /// # Errors
    ///
    /// This function will return an error if the table creation fails.
    pub fn new(connection: Arc<Mutex<Connection>>) -> Result<ProductRepository, rusqlite::Error> {
        let repo = Self { connection };

        // Create the table if it doesn't exist
        repo.create_table_if_not_exists()?;

        Ok(repo)
    }

    /// Creates the table for this [`ProductRepository`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the table creation fails.
    fn create_table_if_not_exists(&self) -> Result<(), rusqlite::Error> {
        self.connection.lock().unwrap().execute(
            "CREATE TABLE IF NOT EXISTS products (
                    id TEXT PRIMARY KEY,
                    brand TEXT NOT NULL,
                    name TEXT NOT NULL
                )",
            params![],
        )?;

        Ok(())
    }
}

impl ProductRepositoryTrait for ProductRepository {
    type Error = rusqlite::Error;

    fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, Self::Error> {
        self.connection
            .lock()
            .unwrap()
            .prepare("SELECT id, brand, name FROM products WHERE id = :id")?
            .query_row(named_params! { ":id": id.as_str() }, |row| {
                let id = row.get::<_, String>(0)?;
                let brand = row.get::<_, String>(1)?;
                let name = row.get::<_, String>(2)?;

                Ok(Product::new(
                    ProductId::new(&id).expect("Invalid product id"),
                    &brand,
                    &name,
                ))
            })
            .optional()
    }

    fn save(&self, product: &Product) -> Result<(), Self::Error> {
        self.connection.lock().unwrap().execute(
            "INSERT INTO products (id, brand, name) VALUES (:id, :brand, :name) ON CONFLICT(id) DO UPDATE SET brand = :brand, name = :name",
            named_params! {
                ":id": product.id().as_str(),
                ":brand": product.brand(),
                ":name": product.name(),
            },
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::product::ProductId;

    fn get_repo() -> ProductRepository {
        let connection = Connection::open_in_memory().unwrap();
        ProductRepository::new(Arc::new(Mutex::new(connection))).unwrap()
    }

    #[test]
    fn test_find_by_id() {
        let repo = get_repo();

        let product_id = ProductId::new("ID").unwrap();
        let product = Product::new(product_id.clone(), "BRAND", "NAME");

        repo.save(&product).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap().unwrap();

        assert_eq!(found_product, product);
    }

    #[test]
    fn test_find_by_id_not_found() {
        let repo = get_repo();

        let product_id = ProductId::new("ID").unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap();

        assert!(found_product.is_none());
    }

    #[test]
    fn test_save() {
        let repo = get_repo();

        let product_id = ProductId::new("ID").unwrap();
        let product = Product::new(product_id.clone(), "BRAND", "NAME");

        repo.save(&product).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap().unwrap();

        assert_eq!(found_product, product);
    }

    #[test]
    fn test_save_update() {
        let repo = get_repo();

        let product_id = ProductId::new("ID").unwrap();
        let product = Product::new(product_id.clone(), "BRAND", "NAME");

        repo.save(&product).unwrap();

        let updated_product = Product::new(product_id.clone(), "BRAND2", "NAME2");

        repo.save(&updated_product).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap().unwrap();

        assert_eq!(found_product, updated_product);
    }
}
