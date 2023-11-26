use std::sync::{Arc, Mutex};

use chrono::NaiveDate;
use rusqlite::{named_params, Connection, ToSql, Transaction};
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

    fn find_by_ids(
        tx: &Transaction,
        ids: &[ProductId],
    ) -> Result<Vec<Product>, ProductRepositoryError> {
        // Create placeholders for the query. This becomes "?, ?, ?, ..."
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>();

        // TODO Clean up this mess
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        for id in ids {
            let id = Box::new(id.to_string());
            params.push(id);
        }
        let params = params.iter().map(|param| &**param).collect::<Vec<_>>();

        let mut stmt = tx.prepare(&format!(
            "SELECT id, brand, name FROM products WHERE id IN ({})",
            placeholders.join(", ")
        ))?;

        // TODO Somehow use query_map or query_and_then here?
        let mut rows = stmt.query(&params[..])?;

        let mut products = vec![];
        while let Some(row) = rows.next()? {
            let product = ProductRepository::row_to_product(row)?;
            let product = ProductRepository::add_stash_items(tx, product)?;
            products.push(product);
        }

        Ok(products)
    }

    /// Gets a product from the database by its ID
    ///
    /// # Parameters
    /// - `id`: The ID of the product to get
    ///
    /// # Returns
    /// The product, if found
    /// An error if the product could not be found
    fn find_by_id(
        tx: &Transaction,
        id: &ProductId,
    ) -> Result<Option<Product>, ProductRepositoryError> {
        ProductRepository::find_by_ids(tx, &[id.clone()]).map(|mut products| products.pop())
    }

    /// Finds a product by the ID of one of its [`StashItem`]s
    ///
    /// # Parameters
    /// - `tx`: The transaction to use
    /// - `stash_item_id`: The ID of the stash item to find the product for
    ///
    /// # Returns
    /// The product, if found
    fn find_by_stash_item_id(
        tx: &Transaction,
        stash_item_id: &Uuid,
    ) -> Result<Option<Product>, ProductRepositoryError> {
        let mut stmt =
            tx.prepare("SELECT product_id FROM stash_items WHERE id = :stash_item_id LIMIT 1")?;
        let mut rows = stmt.query(named_params! { ":stash_item_id": stash_item_id.to_string() })?;

        if let Some(row) = rows.next()? {
            let product_id = row.get::<_, ProductId>("product_id")?;

            ProductRepository::find_by_id(tx, &product_id)
        } else {
            Ok(None)
        }
    }

    /// Finds all products with at least one stash item expiring within the given date interval
    ///
    /// # Parameters
    /// - `tx`: The transaction to use
    /// - `after`: The start of the date range, inclusive
    /// - `before`: The end of the date range, exclusive
    ///
    /// # Returns
    /// A list of products with at least one stash item expiring within the given date interval
    fn find_expiring_in_interval(
        tx: &Transaction,
        after: Option<NaiveDate>,
        before: Option<NaiveDate>,
    ) -> Result<Vec<Product>, ProductRepositoryError> {
        // Hold the query and args for it outside of the match to ensure their lifetime is long enough
        let mut query = String::from("SELECT DISTINCT product_id FROM stash_items WHERE ");
        let mut args: Vec<Box<dyn ToSql>> = Vec::new();

        // Build the query
        match (after, before) {
            (Some(after), Some(before)) => {
                query.push_str("expiry_date >= ? AND expiry_date < ?");
                args.push(Box::new(after.to_string()));
                args.push(Box::new(before.to_string()));
            }
            (Some(after), None) => {
                query.push_str("expiry_date >= ?");
                args.push(Box::new(after.to_string()));
            }
            (None, Some(before)) => {
                query.push_str("expiry_date < ?");
                args.push(Box::new(before.to_string()));
            }
            (None, None) => {
                return Err(ProductRepositoryError::InvalidDateInterval);
            }
        };

        // Convert the args to something the query can use
        let args = args.iter().map(|arg| &**arg).collect::<Vec<_>>();

        let product_ids = tx
            .prepare(&query)?
            .query_map(&args[..], |row| {
                let product_id = row.get::<_, ProductId>("product_id")?;
                Ok(product_id)
            })?
            .collect::<Result<Vec<_>, _>>()?;

        // Get the products
        ProductRepository::find_by_ids(tx, &product_ids)
    }

    /// Saves a [`Product`] to the database. If the product already exists, it will be updated.
    ///
    /// # Parameters
    /// - `tx`: The transaction to use
    /// - `product`: The product to save
    fn save_product(tx: &Transaction, product: Product) -> Result<(), ProductRepositoryError> {
        // Save the product itself
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
        ProductRepository::delete_deleted_stash_items(tx, &product)?;

        // Create and update all stash items
        ProductRepository::save_stash_items(tx, &product)?;

        Ok(())
    }

    /// Deletes a product from the database, along with all its stash items
    ///
    /// # Parameters
    /// - `tx`: The transaction to use
    /// - `product_id`: ID of the product to delete
    fn delete_product(
        tx: &Transaction,
        product_id: &ProductId,
    ) -> Result<(), ProductRepositoryError> {
        // Delete all stash items related to the product
        tx.execute(
            "DELETE FROM stash_items WHERE product_id = :product_id",
            named_params! {
                ":product_id": product_id,
            },
        )?;

        // Delete the product itself
        tx.execute(
            "DELETE FROM products WHERE id = :id",
            named_params! {
                ":id": product_id,
            },
        )?;

        Ok(())
    }

    /// Deletes all [`StashItem`]s from the database that are no longer in the passed [`Product`]
    ///
    /// # Parameters
    /// - `tx`: The transaction to use
    /// - `product`: The product to delete the stash items from
    fn delete_deleted_stash_items(
        tx: &Transaction,
        product: &Product,
    ) -> Result<(), ProductRepositoryError> {
        // Create placeholders for the query. This becomes "?, ?, ?, ..."
        let placeholders = product
            .stash_items()
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>();

        // Make the list of parameters to pass to the query. This becomes [product_id, stash_item_id1, stash_item_id2, ...]
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        params.push(Box::new(product.id()));
        for stash_item in product.stash_items() {
            let id = Box::new(stash_item.id().to_string());
            params.push(id);
        }
        let params = params.iter().map(|param| &**param).collect::<Vec<_>>();

        // Delete all stash items no longer in the product
        tx.execute(
            &format!(
                "DELETE FROM stash_items WHERE product_id = ? AND id NOT IN ({})",
                placeholders.join(", ")
            ),
            &params[..],
        )?;

        Ok(())
    }

    /// Saves all [`StashItem`]s for a given [`Product`] to the database, updating existing ones
    /// and creating new ones
    ///
    /// # Parameters
    /// - `tx`: The transaction to use
    /// - `product`: The product to save the stash items for
    fn save_stash_items(tx: &Transaction, product: &Product) -> Result<(), ProductRepositoryError> {
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

        Ok(())
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
    ///
    /// # Parameters
    /// - `tx`: The transaction to use
    /// - `product_id`: The ID of the product to get the stash items for
    ///
    /// # Returns
    /// A vector of all [`StashItem`]s for the given [`Product`]
    fn get_stash_items(
        tx: &Transaction,
        product_id: &ProductId,
    ) -> Result<Vec<StashItem>, ProductRepositoryError> {
        let mut stmt = tx.prepare(
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
    ///
    /// # Parameters
    /// - `tx`: The transaction to use
    /// - `product`: The product to add the stash items to
    ///
    /// # Returns
    /// The product with the stash items added
    fn add_stash_items(
        tx: &Transaction,
        mut product: Product,
    ) -> Result<Product, ProductRepositoryError> {
        let stash_items = ProductRepository::get_stash_items(&tx, product.id())?;

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
        let mut conn = self.conn();
        let tx = conn.transaction()?;

        let product = ProductRepository::find_by_id(&tx, id)?;

        tx.commit()?;
        Ok(product)
    }

    fn find_by_ids(&self, ids: &[ProductId]) -> Result<Vec<Product>, ProductRepositoryError> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;

        let products = ProductRepository::find_by_ids(&tx, ids)?;

        tx.commit()?;
        Ok(products)
    }

    fn find_by_stash_item_id(
        &self,
        stash_item_id: &Uuid,
    ) -> Result<Option<Product>, ProductRepositoryError> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;

        let result = ProductRepository::find_by_stash_item_id(&tx, stash_item_id)?;

        tx.commit()?;
        Ok(result)
    }

    fn find_expiring_in_interval(
        &self,
        after: Option<NaiveDate>,
        before: Option<NaiveDate>,
    ) -> Result<Vec<Product>, ProductRepositoryError> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;

        let products = ProductRepository::find_expiring_in_interval(&tx, after, before)?;

        tx.commit()?;
        Ok(products)
    }

    fn exists_by_id(&self, id: &ProductId) -> Result<bool, ProductRepositoryError> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;

        let exists = match ProductRepository::find_by_id(&tx, id)? {
            Some(_) => Ok(true),
            None => Ok(false),
        };

        tx.commit()?;
        exists
    }

    fn save(&self, product: Product) -> Result<(), ProductRepositoryError> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;

        ProductRepository::save_product(&tx, product)?;

        tx.commit()?;

        Ok(())
    }

    fn delete_by_id(&self, id: &ProductId) -> Result<(), ProductRepositoryError> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;

        ProductRepository::delete_product(&tx, id)?;

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
    fn test_find_by_ids() {
        let repo = get_repo();

        let product1 = FakeProduct::new().build();
        let product1_id = product1.id().clone();
        let product2 = FakeProduct::new().build();
        let product2_id = product2.id().clone();
        let product3 = FakeProduct::new().build();
        repo.save(product1.clone()).unwrap();
        repo.save(product2.clone()).unwrap();
        repo.save(product3.clone()).unwrap();

        let found_products = repo
            .find_by_ids(&[product1_id.clone(), product2_id.clone()])
            .unwrap();

        assert_eq!(found_products.len(), 2);
        assert!(found_products.contains(&product1));
        assert!(found_products.contains(&product2));
    }

    #[test]
    fn test_find_by_id_not_found() {
        let repo = get_repo();

        let product_id: ProductId = "ID".parse().unwrap();

        let found_product = repo.find_by_id(&product_id).unwrap();

        assert!(found_product.is_none());
    }

    #[test]
    fn test_find_by_stash_item_id() {
        let repo = get_repo();

        let stash_item = FakeStashItem::new().build();
        let stash_item_id = stash_item.id().clone();
        let product = FakeProduct::new()
            .with_stash_items(vec![stash_item])
            .build();
        repo.save(product.clone()).unwrap();

        let found_product = repo.find_by_stash_item_id(&stash_item_id).unwrap().unwrap();

        assert_eq!(found_product, product);
    }

    #[test]
    fn test_find_expiring_in_interval_after() {
        let repo = get_repo();

        let product_1 = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new()
                .with_expiry_date(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())
                .build()])
            .build();
        let product_2 = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new()
                .with_expiry_date(NaiveDate::from_ymd_opt(2023, 1, 2).unwrap())
                .build()])
            .build();
        let product_3 = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new()
                .with_expiry_date(NaiveDate::from_ymd_opt(2023, 1, 3).unwrap())
                .build()])
            .build();

        repo.save(product_1.clone()).unwrap();
        repo.save(product_2.clone()).unwrap();
        repo.save(product_3.clone()).unwrap();

        let found_products = repo
            .find_expiring_in_interval(Some(NaiveDate::from_ymd_opt(2023, 1, 2).unwrap()), None)
            .unwrap();

        assert_eq!(found_products.len(), 2);
        assert!(found_products.contains(&product_2));
        assert!(found_products.contains(&product_3));
    }

    #[test]
    fn test_find_expiring_in_interval_before() {
        let repo = get_repo();

        let product_1 = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new()
                .with_expiry_date(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())
                .build()])
            .build();
        let product_2 = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new()
                .with_expiry_date(NaiveDate::from_ymd_opt(2023, 1, 2).unwrap())
                .build()])
            .build();
        let product_3 = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new()
                .with_expiry_date(NaiveDate::from_ymd_opt(2023, 1, 3).unwrap())
                .build()])
            .build();

        repo.save(product_1.clone()).unwrap();
        repo.save(product_2.clone()).unwrap();
        repo.save(product_3.clone()).unwrap();

        let found_products = repo
            .find_expiring_in_interval(None, Some(NaiveDate::from_ymd_opt(2023, 1, 3).unwrap()))
            .unwrap();

        assert_eq!(found_products.len(), 2);
        assert!(found_products.contains(&product_1));
        assert!(found_products.contains(&product_2));
    }

    #[test]
    fn test_find_expiring_in_interval_both() {
        let repo = get_repo();

        let product_1 = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new()
                .with_expiry_date(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())
                .build()])
            .build();
        let product_2 = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new()
                .with_expiry_date(NaiveDate::from_ymd_opt(2023, 1, 2).unwrap())
                .build()])
            .build();
        let product_3 = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new()
                .with_expiry_date(NaiveDate::from_ymd_opt(2023, 1, 3).unwrap())
                .build()])
            .build();

        repo.save(product_1.clone()).unwrap();
        repo.save(product_2.clone()).unwrap();
        repo.save(product_3.clone()).unwrap();

        let found_products = repo
            .find_expiring_in_interval(
                Some(NaiveDate::from_ymd_opt(2023, 1, 2).unwrap()),
                Some(NaiveDate::from_ymd_opt(2023, 1, 3).unwrap()),
            )
            .unwrap();

        assert_eq!(found_products.len(), 1);
        assert!(found_products.contains(&product_2));
    }

    #[test]
    fn test_find_expiring_in_interval_none() {
        let repo = get_repo();

        let product_1 = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new()
                .with_expiry_date(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())
                .build()])
            .build();

        repo.save(product_1).unwrap();

        let result = repo.find_expiring_in_interval(None, None);

        assert_eq!(
            result.unwrap_err(),
            ProductRepositoryError::InvalidDateInterval
        );
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
