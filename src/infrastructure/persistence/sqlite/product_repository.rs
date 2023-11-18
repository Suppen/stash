use std::sync::{Arc, Mutex};

use rusqlite::{named_params, Connection};

use crate::domain::{
    entities::Product,
    errors::ProductRepositoryError,
    repositories::ProductRepository as ProductRepositoryTrait,
    value_objects::{Brand, ProductId},
};

/// A repository for [`Product`]s using SQLite as the underlying storage.
pub struct ProductRepository {
    /// Connection to the database
    connection: Arc<Mutex<Connection>>,
}

impl ProductRepository {
    /// Creates a new [`ProductRepository`]
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self { connection }
    }

    /// Shortcut to get the connection to the database
    fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.connection.lock().unwrap()
    }

    /// Converts a raw database row into a [`Product`]
    ///
    /// # Errors
    ///
    /// This function will return an error if the row contains invalid data, or if the data cannot be parsed into a
    /// [`Product`]
    fn row_to_product(row: &rusqlite::Row) -> Result<Product, ProductRepositoryError> {
        let id = row.get::<_, ProductId>("id")?;
        let brand = row.get::<_, Brand>("brand")?;
        let name = row.get::<_, String>("name")?;

        Ok(Product::new(id, brand, &name))
    }
}

impl ProductRepositoryTrait for ProductRepository {
    fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, ProductRepositoryError> {
        let conn = self.conn();
        let mut stmt =
            conn.prepare("SELECT id, brand, name FROM products WHERE id = :id LIMIT 1")?;
        let mut rows = stmt.query(named_params! { ":id": id.value() })?;

        let row = rows.next()?;

        if let Some(row) = row {
            ProductRepository::row_to_product(&row).map(Some)
        } else {
            Ok(None)
        }
    }

    fn save(&self, product: Product) -> Result<(), ProductRepositoryError> {
        let conn = self.conn();
        conn.execute(
            "INSERT INTO products (id, brand, name, created_at) VALUES (:id, :brand, :name, :now) ON CONFLICT(id) DO UPDATE SET brand = :brand, name = :name, updated_at = :now",
            named_params! {
                ":id": product.id(),
                ":brand": product.brand(),
                ":name": product.name(),
                ":now": chrono::Utc::now().naive_utc(),
            },
        )?;

        Ok(())
    }

    fn delete_by_id(&self, id: &ProductId) -> Result<(), ProductRepositoryError> {
        let conn = self.conn();
        conn.execute(
            "DELETE FROM products WHERE id = :id",
            named_params! { ":id": id.value() },
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::value_objects::ProductId, infrastructure::persistence::sqlite::db::setup_db,
    };

    fn get_repo() -> ProductRepository {
        // Create an in-memory database
        let connection = Connection::open_in_memory().unwrap();

        // Create the tables in the database
        setup_db(&connection).unwrap();

        ProductRepository::new(Arc::new(Mutex::new(connection)))
    }

    #[test]
    fn test_find_by_id() {
        let repo = get_repo();

        let product_id: ProductId = "ID".parse().unwrap();
        let product = Product::new(product_id.clone(), "BRAND".parse().unwrap(), "NAME");

        repo.save(product.clone()).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap().unwrap();

        assert_eq!(found_product, product);
    }

    #[test]
    fn test_find_by_id_not_found() {
        let repo = get_repo();

        let product_id: ProductId = "ID".parse().unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap();

        assert!(found_product.is_none());
    }

    #[test]
    fn test_save() {
        let repo = get_repo();

        let product_id: ProductId = "ID".parse().unwrap();
        let product = Product::new(product_id.clone(), "BRAND".parse().unwrap(), "NAME");

        repo.save(product.clone()).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap().unwrap();

        assert_eq!(found_product, product);
    }

    #[test]
    fn test_save_update() {
        let repo = get_repo();

        let product_id: ProductId = "ID".parse().unwrap();
        let product = Product::new(product_id.clone(), "BRAND".parse().unwrap(), "NAME");

        repo.save(product).unwrap();

        let updated_product = Product::new(product_id.clone(), "BRAND".parse().unwrap(), "NAME2");

        repo.save(updated_product.clone()).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap().unwrap();

        assert_eq!(found_product, updated_product);
    }

    #[test]
    fn test_delete_by_id() {
        let repo = get_repo();

        let product_id: ProductId = "ID".parse().unwrap();
        let product = Product::new(product_id.clone(), "BRAND".parse().unwrap(), "NAME");

        repo.save(product.clone()).unwrap();

        repo.delete_by_id(&product_id).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap();

        assert!(found_product.is_none());
    }

    #[test]
    fn test_delete_by_id_not_found() {
        let repo = get_repo();

        let product_id: ProductId = "ID".parse().unwrap();

        repo.delete_by_id(&product_id).unwrap();
    }
}
