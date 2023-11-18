use std::sync::{Arc, Mutex};

use chrono::NaiveDate;
use rusqlite::{named_params, Connection};
use uuid::Uuid;

use crate::domain::product::ProductId;
use crate::domain::quantity::Quantity;
use crate::domain::stash_item::StashItem;
use crate::repositories::StashItemRepository as StashItemRepositoryTrait;

use super::StashItemRepositoryError;

/// A repository for [`StashItem`]s.
pub struct StashItemRepository {
    connection: Arc<Mutex<Connection>>,
}

impl StashItemRepository {
    /// Creates a new [`StashItemRepository`]
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self { connection }
    }

    /// Shortcut to get the connection to the database
    fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.connection.lock().unwrap()
    }

    /// Converts a raw database row into a [`StashItem`]
    fn row_to_stash_item(row: &rusqlite::Row) -> Result<StashItem, StashItemRepositoryError> {
        let id = row.get::<_, String>("id")?;
        let product_id = row.get::<_, ProductId>("product_id")?;
        let quantity = row.get::<_, Quantity>("quantity")?;
        let expiry_date = row.get::<_, NaiveDate>("expiry_date")?;

        Ok(StashItem::new(
            Uuid::parse_str(&id)?,
            product_id,
            quantity,
            expiry_date,
        ))
    }

    /// Converts raw database rows into [`StashItem`]s
    fn rows_to_stash_items(
        rows: &mut rusqlite::Rows,
    ) -> Result<Vec<StashItem>, StashItemRepositoryError> {
        let mut stash_items = Vec::new();

        while let Some(row) = rows.next()? {
            stash_items.push(StashItemRepository::row_to_stash_item(&row)?);
        }

        Ok(stash_items)
    }
}

impl StashItemRepositoryTrait<StashItemRepositoryError> for StashItemRepository {
    fn find_by_id(&self, id: &Uuid) -> Result<Option<StashItem>, StashItemRepositoryError> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT id, product_id, quantity, expiry_date FROM stash_items WHERE id = :id LIMIT 1",
        )?;
        let mut rows = stmt.query(named_params! { ":id": id.to_string() })?;

        let row = rows.next()?;

        if let Some(row) = row {
            StashItemRepository::row_to_stash_item(&row).map(Some)
        } else {
            Ok(None)
        }
    }

    fn find_all_by_product_id(
        &self,
        product_id: &ProductId,
    ) -> Result<Vec<StashItem>, StashItemRepositoryError> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT id, product_id, quantity, expiry_date FROM stash_items WHERE product_id = :product_id",
        )?;
        let mut rows = stmt.query(named_params! { ":product_id": product_id })?;

        Ok(StashItemRepository::rows_to_stash_items(&mut rows)?)
    }

    fn find_by_product_id_and_expiry_date(
        &self,
        product_id: &ProductId,
        expiry_date: &chrono::NaiveDate,
    ) -> Result<Option<StashItem>, StashItemRepositoryError> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT id, product_id, quantity, expiry_date FROM stash_items WHERE product_id = :product_id AND expiry_date = :expiry_date LIMIT 1",
        )?;
        let mut rows = stmt.query(named_params! {
            ":product_id": product_id.to_string(),
            ":expiry_date": expiry_date,
        })?;

        let row = rows.next()?;

        if let Some(row) = row {
            StashItemRepository::row_to_stash_item(&row).map(Some)
        } else {
            Ok(None)
        }
    }

    fn find_all_expiring_before(
        &self,
        date: &NaiveDate,
    ) -> Result<Vec<StashItem>, StashItemRepositoryError> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT id, product_id, quantity, expiry_date FROM stash_items WHERE expiry_date < :date",
        )?;
        let mut rows = stmt.query(named_params! { ":date": date })?;

        Ok(StashItemRepository::rows_to_stash_items(&mut rows)?)
    }

    fn save(&self, stash_item: StashItem) -> Result<(), StashItemRepositoryError> {
        let conn = self.conn();
        conn.execute(
            "INSERT INTO stash_items (id, product_id, quantity, expiry_date, created_at) VALUES (:id, :product_id, :quantity, :expiry_date, :now)
            ON CONFLICT(id) DO UPDATE SET product_id = :product_id, quantity = :quantity, expiry_date = :expiry_date, updated_at = :now",
            named_params! {
                ":id": stash_item.id().to_string(),
                ":product_id": stash_item.product_id().value(),
                ":quantity": stash_item.quantity(),
                ":expiry_date": stash_item.expiry_date(),
                ":now": chrono::Utc::now().naive_utc(),
            },
        )?;

        Ok(())
    }

    fn delete(&self, id: &Uuid) -> Result<(), StashItemRepositoryError> {
        let conn = self.conn();
        conn.execute(
            "DELETE FROM stash_items WHERE id = :id",
            named_params! { ":id": id.to_string() },
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::product::Product,
        infrastructure::sqlite::{db::setup_db, ProductRepository},
        repositories::ProductRepository as ProductRepositoryTrait,
    };

    use super::*;
    use chrono::NaiveDate;

    fn get_repo() -> StashItemRepository {
        // Create an in-memory database
        let connection = Connection::open_in_memory().unwrap();

        // Initialize the database
        setup_db(&connection).unwrap();

        // Share the connection between the repos
        let shared_connection = Arc::new(Mutex::new(connection));

        // Create the repos
        let product_repository = ProductRepository::new(shared_connection.clone());
        let stash_item_repository = StashItemRepository::new(shared_connection.clone());

        // Save a dummy product so we don't get foreign key violations in the tests
        product_repository
            .save(Product::new(
                "ID".parse().unwrap(),
                "BRAND".parse().unwrap(),
                "NAME",
            ))
            .unwrap();

        stash_item_repository
    }

    #[test]
    fn test_find_by_id() {
        let repo = get_repo();

        let stash_item = StashItem::new(
            Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        repo.save(stash_item.clone()).unwrap();

        let found_stash_item = repo.find_by_id(stash_item.id()).unwrap().unwrap();

        assert_eq!(stash_item, found_stash_item);
    }

    #[test]
    fn test_find_by_id_not_found() {
        let repo = get_repo();

        let stash_item_id = Uuid::new_v4();

        let found_stash_item = repo.find_by_id(&stash_item_id).unwrap();

        assert!(found_stash_item.is_none());
    }

    #[test]
    fn test_find_by_product_id() {
        let repo = get_repo();

        let stash_item_1 = StashItem::new(
            Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );
        let stash_item_2 = StashItem::new(
            Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(2).unwrap(),
            NaiveDate::from_ymd_opt(2020, 1, 2).unwrap(),
        );

        repo.save(stash_item_1.clone()).unwrap();
        repo.save(stash_item_2.clone()).unwrap();

        let found_stash_items = repo
            .find_all_by_product_id(stash_item_1.product_id())
            .unwrap();

        assert_eq!(found_stash_items.len(), 2);
        assert!(found_stash_items.contains(&stash_item_1));
        assert!(found_stash_items.contains(&stash_item_2));
    }

    #[test]
    fn test_find_by_product_id_and_expiry_date() {
        let repo = get_repo();

        let stash_item = StashItem::new(
            Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        repo.save(stash_item.clone()).unwrap();

        let found_stash_item = repo
            .find_by_product_id_and_expiry_date(stash_item.product_id(), stash_item.expiry_date())
            .unwrap()
            .unwrap();

        assert_eq!(stash_item, found_stash_item);
    }

    #[test]
    fn test_find_by_product_id_and_expiry_date_not_found() {
        let repo = get_repo();

        let found_stash_item = repo
            .find_by_product_id_and_expiry_date(
                &"ID".parse().unwrap(),
                &NaiveDate::from_ymd_opt(2020, 1, 2).unwrap(),
            )
            .unwrap();

        assert!(found_stash_item.is_none());
    }

    #[test]
    fn test_find_all_expiring_before() {
        let repo = get_repo();

        let stash_item_1 = StashItem::new(
            Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(1).unwrap(),
            NaiveDate::from_ymd_opt(2019, 12, 31).unwrap(),
        );
        let stash_item_2 = StashItem::new(
            Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(2).unwrap(),
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );
        let stash_item_3 = StashItem::new(
            Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(3).unwrap(),
            NaiveDate::from_ymd_opt(2020, 1, 2).unwrap(),
        );

        repo.save(stash_item_1.clone()).unwrap();
        repo.save(stash_item_2.clone()).unwrap();
        repo.save(stash_item_3.clone()).unwrap();

        let found_stash_items = repo
            .find_all_expiring_before(&NaiveDate::from_ymd_opt(2020, 1, 2).unwrap())
            .unwrap();

        assert_eq!(found_stash_items.len(), 2);
        assert!(found_stash_items.contains(&stash_item_1));
        assert!(found_stash_items.contains(&stash_item_2));
    }

    #[test]
    fn test_save() {
        let repo = get_repo();

        let stash_item = StashItem::new(
            Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(1).unwrap(),
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
            Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        repo.save(stash_item.clone()).unwrap();

        let updated_stash_item = StashItem::new(
            stash_item.id().clone(),
            "ID".parse().unwrap(),
            Quantity::new(2).unwrap(),
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
            Uuid::new_v4(),
            "BAD ID".parse().unwrap(),
            Quantity::new(1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        let result = repo.save(stash_item);

        assert!(result.is_err());
    }

    #[test]
    fn test_save_duplicate_product_id_and_expiry_date() {
        let repo = get_repo();

        let stash_item_1 = StashItem::new(
            Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );
        let stash_item_2 = StashItem::new(
            Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(2).unwrap(),
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        repo.save(stash_item_1.clone()).unwrap();

        let result = repo.save(stash_item_2);

        assert!(result.is_err());
    }

    #[test]
    fn test_delete() {
        let repo = get_repo();

        let stash_item = StashItem::new(
            Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(1).unwrap(),
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

        let stash_item_id = Uuid::new_v4();

        let result = repo.delete(&stash_item_id);

        assert!(result.is_ok());
    }
}
