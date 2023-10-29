use std::sync::{Arc, Mutex};

use crate::domain::brand::Brand;
use crate::domain::entity::Entity;
use crate::domain::product::{Product, ProductId};
use crate::domain::value_object::ValueObject;
use crate::repositories::ProductRepository as ProductRepositoryTrait;
use rusqlite::{named_params, Connection};

use super::product_repository_error::ProductRepositoryError;

/// A repository for [`Product`]s using SQLite as the underlying storage.
pub struct ProductRepository {
    /// The connection to the database. This is wrapped in an [`Arc`] and a [`Mutex`] to allow multiple repos to use the
    /// same connection
    connection: Arc<Mutex<Connection>>,
}

impl ProductRepository {
    /// Creates a new [`ProductRepository`]. Creates the table in the database if it doesn't exist
    ///
    /// # Errors
    ///
    /// This function will return an error if the table creation fails.
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self { connection }
    }

    /// Shortcut to get the connection to the database
    fn conn(&self) -> std::sync::MutexGuard<Connection> {
        self.connection.lock().unwrap()
    }
}

impl ProductRepositoryTrait for ProductRepository {
    type Error = ProductRepositoryError;

    fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, Self::Error> {
        let conn = self.conn();
        let mut stmt =
            conn.prepare("SELECT id, brand, name FROM products WHERE id = :id LIMIT 1")?;
        let mut rows = stmt.query(named_params! { ":id": id.value() })?;

        let row = rows.next()?;

        if let Some(row) = row {
            let id = row.get::<_, String>("id")?;
            let brand = row.get::<_, String>("brand")?;
            let name = row.get::<_, String>("name")?;

            Ok(Some(Product::new(
                ProductId::new(id)?,
                Brand::new(brand)?,
                &name,
            )))
        } else {
            Ok(None)
        }
    }

    fn save(&self, product: &Product) -> Result<(), Self::Error> {
        self.conn().execute(
            "INSERT INTO products (id, brand, name) VALUES (:id, :brand, :name) ON CONFLICT(id) DO UPDATE SET brand = :brand, name = :name",
            named_params! {
                ":id": product.id().value(),
                ":brand": product.brand().value(),
                ":name": product.name(),
            },
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{domain::product::ProductId, infrastructure::sqlite::setup_db};

    fn get_repo() -> ProductRepository {
        let connection = Connection::open_in_memory().unwrap();
        setup_db(&connection).unwrap();

        ProductRepository::new(Arc::new(Mutex::new(connection)))
    }

    #[test]
    fn test_find_by_id() {
        let repo = get_repo();

        let product_id = ProductId::new(String::from("ID")).unwrap();
        let product = Product::new(
            product_id.clone(),
            Brand::new(String::from("BRAND")).unwrap(),
            "NAME",
        );

        repo.save(&product).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap().unwrap();

        assert_eq!(found_product, product);
    }

    #[test]
    fn test_find_by_id_not_found() {
        let repo = get_repo();

        let product_id = ProductId::new(String::from("ID")).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap();

        assert!(found_product.is_none());
    }

    #[test]
    fn test_save() {
        let repo = get_repo();

        let product_id = ProductId::new(String::from("ID")).unwrap();
        let product = Product::new(
            product_id.clone(),
            Brand::new(String::from("BRAND")).unwrap(),
            "NAME",
        );

        repo.save(&product).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap().unwrap();

        assert_eq!(found_product, product);
    }

    #[test]
    fn test_save_update() {
        let repo = get_repo();

        let product_id = ProductId::new(String::from("ID")).unwrap();
        let product = Product::new(
            product_id.clone(),
            Brand::new(String::from("BRAND")).unwrap(),
            "NAME",
        );

        repo.save(&product).unwrap();

        let updated_product = Product::new(
            product_id.clone(),
            Brand::new(String::from("BRAND2")).unwrap(),
            "NAME2",
        );

        repo.save(&updated_product).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap().unwrap();

        assert_eq!(found_product, updated_product);
    }
}
