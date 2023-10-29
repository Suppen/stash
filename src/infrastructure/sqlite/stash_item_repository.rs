use chrono::NaiveDate;
use rusqlite::{named_params, Connection, OptionalExtension};
use std::sync::{Arc, Mutex};

use crate::domain::entity::Entity;
use crate::domain::stash_item::StashItemId;
use crate::domain::value_object::ValueObject;
use crate::domain::{product::ProductId, stash_item::StashItem};
use crate::repositories::StashItemRepository as StashItemRepositoryTrait;

/// A repository for [`StashItem`]s.
pub struct StashItemRepository {
    /// The connection to the database. This is wrapped in an [`Arc`] and a [`Mutex`] to allow multiple repos to use the
    /// same connection
    connection: Arc<Mutex<Connection>>,
}

impl StashItemRepository {
    /// Creates a new [`StashItemRepository`]. Creates the table in the database if it doesn't exist
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

impl StashItemRepositoryTrait for StashItemRepository {
    type Error = rusqlite::Error;

    fn find_by_id(&self, id: &StashItemId) -> Result<Option<StashItem>, Self::Error> {
        self.conn()
            .prepare(
                "SELECT id, product_id, quantity, expiry_date FROM stash_items WHERE id = :id",
            )?
            .query_row(named_params! { ":id": id.value() }, |row| {
                let id = row.get::<_, String>(0)?;
                let product_id = row.get::<_, String>(1)?;
                let quantity = row.get::<_, i64>(2)?;
                let expiry_date = row.get::<_, NaiveDate>(3)?;

                Ok(StashItem::new(
                    StashItemId::new(id).expect("Invalid stash item id"),
                    ProductId::new(product_id).expect("Invalid product id"),
                    quantity,
                    expiry_date,
                ))
            })
            .optional()
    }

    fn save(&self, stash_item: StashItem) -> Result<(), Self::Error> {
        self.conn().execute(
            "INSERT INTO stash_items (id, product_id, quantity, expiry_date) VALUES (:id, :product_id, :quantity, :expiry_date)
            ON CONFLICT(id) DO UPDATE SET product_id = :product_id, quantity = :quantity, expiry_date = :expiry_date",
            named_params! {
                ":id": stash_item.id().value(),
                ":product_id": stash_item.product_id().value(),
                ":quantity": stash_item.quantity(),
                ":expiry_date": stash_item.expiry_date().to_string(),
            },
        )?;

        Ok(())
    }

    fn delete(&self, id: &StashItemId) -> Result<(), Self::Error> {
        self.conn().execute(
            "DELETE FROM stash_items WHERE id = :id",
            named_params! { ":id": id.value() },
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::{brand::Brand, product::Product, value_object::ValueObject},
        infrastructure::sqlite::{setup_db, ProductRepository},
        repositories::ProductRepository as ProductRepositoryTrait,
    };

    use super::*;
    use chrono::NaiveDate;

    fn get_repo() -> StashItemRepository {
        // Create the database the repo(s) will use
        let connection = Connection::open_in_memory().unwrap();
        setup_db(&connection).unwrap();

        // Wrap the connection in an Arc and a Mutex so it can be shared between repos
        let shared_connection = Arc::new(Mutex::new(connection));

        // Create the repos
        let product_repository = ProductRepository::new(shared_connection.clone());
        let stash_item_repository = StashItemRepository::new(shared_connection.clone());

        // Save a dummy product so we don't get foreign key violations in the tests
        product_repository
            .save(&Product::new(
                ProductId::new(String::from("ID")).unwrap(),
                Brand::new(String::from("BRAND")).unwrap(),
                "NAME",
            ))
            .unwrap();

        stash_item_repository
    }

    #[test]
    fn test_find_by_id() {
        let repo = get_repo();

        let stash_item = StashItem::new(
            StashItemId::new(String::from("ID")).unwrap(),
            ProductId::new(String::from("ID")).unwrap(),
            1,
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        repo.save(stash_item.clone()).unwrap();

        let found_stash_item = repo.find_by_id(stash_item.id()).unwrap().unwrap();

        assert_eq!(stash_item, found_stash_item);
    }

    #[test]
    fn test_find_by_id_not_found() {
        let repo = get_repo();

        let stash_item_id = StashItemId::new(String::from("ID")).unwrap();

        let found_stash_item = repo.find_by_id(&stash_item_id).unwrap();

        assert!(found_stash_item.is_none());
    }

    #[test]
    fn test_save() {
        let repo = get_repo();

        let stash_item = StashItem::new(
            StashItemId::new(String::from("ID")).unwrap(),
            ProductId::new(String::from("ID")).unwrap(),
            1,
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        repo.save(stash_item.clone()).unwrap();

        let found_stash_item = repo.find_by_id(stash_item.id()).unwrap().unwrap();

        assert_eq!(stash_item, found_stash_item);
    }

    #[test]
    fn test_save_update() {
        let repo = get_repo();

        let stash_item = StashItem::new(
            StashItemId::new(String::from("ID")).unwrap(),
            ProductId::new(String::from("ID")).unwrap(),
            1,
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        repo.save(stash_item.clone()).unwrap();

        let updated_stash_item = StashItem::new(
            stash_item.id().clone(),
            ProductId::new(String::from("ID")).unwrap(),
            2,
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        repo.save(updated_stash_item.clone()).unwrap();

        let found_stash_item = repo.find_by_id(stash_item.id()).unwrap().unwrap();

        assert_eq!(updated_stash_item, found_stash_item);
    }

    #[test]
    fn test_save_bad_product_id() {
        let repo = get_repo();

        let stash_item = StashItem::new(
            StashItemId::new(String::from("ID")).unwrap(),
            ProductId::new(String::from("BAD_ID")).unwrap(),
            1,
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        let result = repo.save(stash_item);

        assert!(result.is_err());
    }

    #[test]
    fn test_delete() {
        let repo = get_repo();

        let stash_item = StashItem::new(
            StashItemId::new(String::from("ID")).unwrap(),
            ProductId::new(String::from("ID")).unwrap(),
            1,
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        repo.save(stash_item.clone()).unwrap();

        repo.delete(stash_item.id()).unwrap();

        let found_stash_item = repo.find_by_id(stash_item.id()).unwrap();

        assert!(found_stash_item.is_none());
    }

    #[test]
    fn test_delete_not_found() {
        let repo = get_repo();

        let stash_item_id = StashItemId::new(String::from("ID")).unwrap();

        let result = repo.delete(&stash_item_id);

        assert!(result.is_ok());
    }
}
