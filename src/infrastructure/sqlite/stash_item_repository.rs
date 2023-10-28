use std::sync::{Arc, Mutex};

use crate::domain::stash_item::StashItemId;
use crate::domain::{product::ProductId, stash_item::StashItem};
use crate::repositories::StashItemRepository as StashItemRepositoryTrait;

use chrono::NaiveDate;
use rusqlite::{named_params, params, Connection, OptionalExtension};

/// A repository for [`StashItem`]s.
struct StashItemRepository {
    // The connection to the database
    connection: Arc<Mutex<Connection>>,
}

impl StashItemRepository {
    /// Creates a new [`StashItemRepository`]. Creates the table in the database if it doesn't exist
    ///
    /// # Errors
    ///
    /// This function will return an error if the table creation fails.
    pub fn new(connection: Arc<Mutex<Connection>>) -> Result<StashItemRepository, rusqlite::Error> {
        let repo = Self { connection };

        // Create the table if it doesn't exist
        repo.create_table_if_not_exists()?;

        Ok(repo)
    }

    /// Creates the table for this [`StashItemRepository`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the table creation fails.
    fn create_table_if_not_exists(&self) -> Result<(), rusqlite::Error> {
        self.connection.lock().unwrap().execute(
            "CREATE TABLE IF NOT EXISTS stash_items (
                    id TEXT PRIMARY KEY,
                    product_id TEXT NOT NULL,
                    quantity INTEGER NOT NULL,
                    expiry_date TEXT NOT NULL
                )",
            params![],
        )?;

        Ok(())
    }
}

impl StashItemRepositoryTrait for StashItemRepository {
    type Error = rusqlite::Error;

    fn find_by_id(&self, id: &StashItemId) -> Result<Option<StashItem>, Self::Error> {
        self.connection
            .lock()
            .unwrap()
            .prepare(
                "SELECT id, product_id, quantity, expiry_date FROM stash_items WHERE id = :id",
            )?
            .query_row(named_params! { ":id": id.as_str() }, |row| {
                let id = row.get::<_, String>(0)?;
                let product_id = row.get::<_, String>(1)?;
                let quantity = row.get::<_, i64>(2)?;
                let expiry_date = row.get::<_, NaiveDate>(3)?;

                Ok(StashItem::new(
                    StashItemId::new(&id).expect("Invalid stash item id"),
                    ProductId::new(&product_id).expect("Invalid product id"),
                    quantity,
                    expiry_date,
                ))
            })
            .optional()
    }

    fn save(&self, stash_item: StashItem) -> Result<(), Self::Error> {
        self.connection.lock().unwrap().execute(
            "INSERT INTO stash_items (id, product_id, quantity, expiry_date) VALUES (:id, :product_id, :quantity, :expiry_date)
            ON CONFLICT(id) DO UPDATE SET product_id = :product_id, quantity = :quantity, expiry_date = :expiry_date",
            named_params! {
                ":id": stash_item.id().as_str(),
                ":product_id": stash_item.product_id().as_str(),
                ":quantity": stash_item.quantity(),
                ":expiry_date": stash_item.expiry_date().to_string(),
            },
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn get_repo() -> StashItemRepository {
        let connection = Connection::open_in_memory().unwrap();
        StashItemRepository::new(Arc::new(Mutex::new(connection))).unwrap()
    }

    #[test]
    fn test_find_by_id() {
        let repo = get_repo();

        let stash_item = StashItem::new(
            StashItemId::new("ID").unwrap(),
            ProductId::new("ID").unwrap(),
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

        let stash_item_id = StashItemId::new("ID").unwrap();

        let found_stash_item = repo.find_by_id(&stash_item_id).unwrap();

        assert!(found_stash_item.is_none());
    }

    #[test]
    fn test_save() {
        let repo = get_repo();

        let stash_item = StashItem::new(
            StashItemId::new("ID").unwrap(),
            ProductId::new("ID").unwrap(),
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
            StashItemId::new("ID").unwrap(),
            ProductId::new("ID").unwrap(),
            1,
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        repo.save(stash_item.clone()).unwrap();

        let updated_stash_item = StashItem::new(
            stash_item.id().clone(),
            ProductId::new("ID").unwrap(),
            2,
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        repo.save(updated_stash_item.clone()).unwrap();

        let found_stash_item = repo.find_by_id(stash_item.id()).unwrap().unwrap();

        assert_eq!(updated_stash_item, found_stash_item);
    }
}
