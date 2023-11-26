use std::sync::{Arc, Mutex};

use chrono::NaiveDate;
use rusqlite::{named_params, Connection};
use uuid::Uuid;

use crate::domain::{
    entities::{Product, StashItem},
    errors::ProductRepositoryError,
    repositories::ProductRepository as ProductRepositoryTrait,
    value_objects::{Brand, ProductId, Quantity},
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

        Ok(Product::new(id, brand, name, vec![]))
    }

    /// Converts a row into a [`StashItem`]
    ///
    /// # Errors
    ///
    /// This function will return an error if the row contains invalid data, or if the data cannot be parsed into a
    /// [`StashItem`]
    fn row_to_stash_item(row: &rusqlite::Row) -> Result<StashItem, ProductRepositoryError> {
        let id = row.get::<_, String>("id")?;
        let quantity = row.get::<_, Quantity>("quantity")?;
        let expiry_date = row.get::<_, NaiveDate>("expiry_date")?;

        Ok(StashItem::new(Uuid::parse_str(&id)?, quantity, expiry_date))
    }

    /// Gets all [`StashItem`]s for a given [`Product`]
    fn get_stash_items(
        connection: &Connection,
        product_id: &ProductId,
    ) -> Result<Vec<StashItem>, ProductRepositoryError> {
        let mut stmt = connection.prepare(
            "SELECT id, quantity, expiry_date FROM stash_items WHERE product_id = :product_id",
        )?;
        let mut rows = stmt.query(named_params! { ":product_id": product_id })?;

        let mut stash_items = vec![];

        while let Some(row) = rows.next()? {
            stash_items.push(ProductRepository::row_to_stash_item(row)?);
        }

        Ok(stash_items)
    }

    /// Fetches and adds all [`StashItem`]s for a given [`Product`] to the [`Product`]
    fn add_stash_items(
        connection: &Connection,
        mut product: Product,
    ) -> Result<Product, ProductRepositoryError> {
        let stash_items = ProductRepository::get_stash_items(connection, product.id())?;

        stash_items.into_iter().for_each(|stash_item| {
            product.add_stash_item(stash_item).expect(
                format!("Duplicate expiry dates in DB for product {}", product.id()).as_str(),
            );
        });

        Ok(product)
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
            let product = ProductRepository::row_to_product(row)?;

            ProductRepository::add_stash_items(&conn, product).map(Some)
        } else {
            Ok(None)
        }
    }

    fn exists_by_id(&self, id: &ProductId) -> Result<bool, ProductRepositoryError> {
        let conn = self.conn();
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM products WHERE id = :id LIMIT 1")?;
        let mut rows = stmt.query(named_params! { ":id": id.value() })?;

        let row = rows.next()?;

        if let Some(row) = row {
            let count: i64 = row.get(0)?;

            Ok(count > 0)
        } else {
            Ok(false)
        }
    }

    fn save(&self, product: Product) -> Result<(), ProductRepositoryError> {
        let mut conn = self.conn();

        // Make a transaction
        let tx = conn.transaction()?;

        // Save the product
        tx.execute(
            "INSERT INTO products (id, brand, name, created_at) VALUES (:id, :brand, :name, :now) ON CONFLICT(id) DO UPDATE SET brand = :brand, name = :name, updated_at = :now",
            named_params! {
                ":id": product.id(),
                ":brand": product.brand(),
                ":name": product.name(),
                ":now": chrono::Utc::now().naive_utc(),
            },
        )?;

        // Delete all stash items no longer in the product
        let placeholders = product
            .stash_items()
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>();

        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        params.push(Box::new(product.id()));
        for stash_item in product.stash_items() {
            let id = Box::new(stash_item.id().to_string());
            params.push(id);
        }
        let params = params.iter().map(|param| &**param).collect::<Vec<_>>();

        tx.execute(
            &format!(
                "DELETE FROM stash_items WHERE product_id = ? AND id NOT IN ({})",
                placeholders.join(", ")
            ),
            &params[..],
        )?;

        // Create and update all stash items
        for stash_item in product.stash_items() {
            tx.execute(
            "INSERT INTO stash_items (id, product_id, quantity, expiry_date, created_at) VALUES (:id, :product_id, :quantity, :expiry_date, :now) ON CONFLICT(id) DO UPDATE SET quantity = :quantity, expiry_date = :expiry_date, updated_at = :now"
            , named_params! {
                ":id": stash_item.id().to_string(),
                ":product_id": product.id(),
                ":quantity": stash_item.quantity(),
                ":expiry_date": stash_item.expiry_date(),
                ":now": chrono::Utc::now().naive_utc(),
            })?;
        }

        // Commit the transaction
        tx.commit()?;

        Ok(())
    }

    fn delete_by_id(&self, id: &ProductId) -> Result<(), ProductRepositoryError> {
        let mut conn = self.conn();

        // Make a transaction
        let tx = conn.transaction()?;

        // Delete all stash items related to the product
        tx.execute(
            "DELETE FROM stash_items WHERE product_id = :product_id",
            named_params! {
                ":product_id": id,
            },
        )?;

        // Delete the product
        tx.execute(
            "DELETE FROM products WHERE id = :id",
            named_params! {
                ":id": id,
            },
        )?;

        // Commit the transaction
        tx.commit()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::{
            entities::{FakeProduct, FakeStashItem},
            value_objects::ProductId,
        },
        infrastructure::persistence::sqlite::db::setup_db,
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

        let product = FakeProduct::new().build();
        let product_id = product.id().clone();
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
    fn test_exists_by_id_true() {
        let repo = get_repo();

        let product_id: ProductId = "ID".parse().unwrap();

        repo.save(Product::new(
            product_id.clone(),
            "BRAND".parse().unwrap(),
            "NAME".to_string(),
            vec![],
        ))
        .unwrap();

        let exists = repo.exists_by_id(&product_id).unwrap();

        assert!(exists);
    }

    #[test]
    fn test_exists_by_id_false() {
        let repo = get_repo();

        let product_id: ProductId = "ID".parse().unwrap();

        let exists = repo.exists_by_id(&product_id).unwrap();

        assert!(!exists);
    }

    #[test]
    fn test_save_new() {
        let repo = get_repo();

        let product = FakeProduct::new().build();
        let product_id = product.id().clone();
        repo.save(product.clone()).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap().unwrap();

        assert_eq!(found_product, product);
    }

    #[test]
    fn test_save_update() {
        let repo = get_repo();

        let mut product = FakeProduct::new().build();
        let product_id = product.id().clone();
        repo.save(product.clone()).unwrap();

        product.set_name("NEW NAME".to_string());

        repo.save(product.clone()).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap().unwrap();

        assert_eq!(found_product, product);
    }

    #[test]
    fn test_save_update_add_stash_item() {
        let repo = get_repo();

        let mut product = FakeProduct::new().build();
        let product_id = product.id().clone();
        repo.save(product.clone()).unwrap();

        product
            .add_stash_item(StashItem::new(
                Uuid::new_v4(),
                2.try_into().unwrap(),
                NaiveDate::from_ymd_opt(2021, 1, 2).unwrap(),
            ))
            .unwrap();

        repo.save(product.clone()).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap().unwrap();

        assert_eq!(found_product, product);
    }

    #[test]
    fn test_save_update_remove_stash_item() {
        let repo = get_repo();

        let stash_item_id = Uuid::new_v4();
        let mut product = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new().with_id(stash_item_id).build()])
            .build();
        let product_id = product.id().clone();

        repo.save(product.clone()).unwrap();

        product.remove_stash_item(&stash_item_id).unwrap();

        repo.save(product.clone()).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap().unwrap();

        assert_eq!(found_product, product);
    }

    #[test]
    fn test_save_all() {
        let repo = get_repo();

        let stash_item_to_remove = Uuid::new_v4();
        let stash_item_to_update = Uuid::new_v4();
        let mut product = FakeProduct::new()
            .with_stash_items(vec![
                FakeStashItem::new().with_id(stash_item_to_remove).build(),
                FakeStashItem::new().with_id(stash_item_to_update).build(),
            ])
            .build();
        let product_id = product.id().clone();

        repo.save(product.clone()).unwrap();

        product.set_name("NEW NAME".to_string());
        product.set_brand("NEW BRAND".parse().unwrap());
        product
            .add_stash_item(StashItem::new(
                Uuid::new_v4(),
                3.try_into().unwrap(),
                NaiveDate::from_ymd_opt(2021, 1, 3).unwrap(),
            ))
            .unwrap();
        product
            .update_stash_item(StashItem::new(
                stash_item_to_update,
                4.try_into().unwrap(),
                NaiveDate::from_ymd_opt(2021, 1, 4).unwrap(),
            ))
            .unwrap();
        product.remove_stash_item(&stash_item_to_remove).unwrap();

        repo.save(product.clone()).unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap().unwrap();

        assert_eq!(found_product, product);
    }

    #[test]
    fn test_delete_by_id() {
        let repo = get_repo();

        let product = FakeProduct::new().build();
        let product_id = product.id().clone();
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
