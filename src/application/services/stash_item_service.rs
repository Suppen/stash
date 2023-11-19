use std::sync::Arc;

use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    application::usecases::{
        DeleteStashItemById, GetStashItemById, GetStashItemByProductIdAndExpiryDate,
        GetStashItemsByProductId, GetStashItemsExpiringBefore, SaveStashItem,
    },
    domain::{
        entities::StashItem, errors::StashItemRepositoryError, repositories::StashItemRepository,
        value_objects::ProductId,
    },
};

pub struct StashItemService {
    pub stash_item_repository: Arc<Box<dyn StashItemRepository>>,
}

impl StashItemService {
    pub fn new(stash_item_repository: Arc<Box<dyn StashItemRepository>>) -> Self {
        Self {
            stash_item_repository,
        }
    }
}

impl GetStashItemById for StashItemService {
    fn get_stash_item_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<StashItem>, StashItemRepositoryError> {
        self.stash_item_repository.find_by_id(id)
    }
}

impl GetStashItemsByProductId for StashItemService {
    fn get_stash_items_by_product_id(
        &self,
        product_id: &ProductId,
    ) -> Result<Vec<StashItem>, StashItemRepositoryError> {
        self.stash_item_repository
            .find_all_by_product_id(product_id)
    }
}

impl GetStashItemByProductIdAndExpiryDate for StashItemService {
    fn get_stash_item_by_product_id_and_expiry_date(
        &self,
        product_id: &ProductId,
        expiry_date: &NaiveDate,
    ) -> Result<Option<StashItem>, StashItemRepositoryError> {
        self.stash_item_repository
            .find_by_product_id_and_expiry_date(product_id, expiry_date)
    }
}

impl GetStashItemsExpiringBefore for StashItemService {
    fn get_stash_items_expiring_before(
        &self,
        date: &NaiveDate,
    ) -> Result<Vec<StashItem>, StashItemRepositoryError> {
        self.stash_item_repository.find_all_expiring_before(date)
    }
}

impl SaveStashItem for StashItemService {
    fn save_stash_item(&self, stash_item: StashItem) -> Result<(), StashItemRepositoryError> {
        self.stash_item_repository.save(stash_item)
    }
}

impl DeleteStashItemById for StashItemService {
    fn delete_stash_item_by_id(&self, id: &Uuid) -> Result<(), StashItemRepositoryError> {
        self.stash_item_repository.delete(id)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{repositories::MockStashItemRepository, value_objects::Quantity};

    use super::*;
    use chrono::NaiveDate;
    use mockall::predicate::*;
    use uuid::Uuid;

    #[test]
    fn test_get_stash_item_by_id() {
        let mut stash_item_repository = MockStashItemRepository::new();
        let stash_item = StashItem::new(
            uuid::Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(1).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );
        let returned_stash_item = stash_item.clone();

        stash_item_repository
            .expect_find_by_id()
            .with(eq(stash_item.id().clone()))
            .returning(move |_| Ok(Some(returned_stash_item.clone())));
        let service = StashItemService::new(Arc::new(Box::new(stash_item_repository)));
        let result = service.get_stash_item_by_id(&stash_item.id());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(stash_item));
    }

    #[test]
    fn test_get_stash_item_by_id_not_found() {
        let mut stash_item_repository = MockStashItemRepository::new();

        stash_item_repository
            .expect_find_by_id()
            .returning(move |_| Ok(None));
        let service = StashItemService::new(Arc::new(Box::new(stash_item_repository)));
        let result = service.get_stash_item_by_id(&Uuid::new_v4()).unwrap();

        assert_eq!(result, None);
    }

    #[test]
    fn test_get_stash_items_by_product_id() {
        let mut stash_item_repository = MockStashItemRepository::new();
        let stash_item = StashItem::new(
            uuid::Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(1).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );
        let returned_stash_item = stash_item.clone();

        stash_item_repository
            .expect_find_all_by_product_id()
            .with(eq(stash_item.product_id().clone()))
            .returning(move |_| Ok(vec![returned_stash_item.clone()]));
        let service = StashItemService::new(Arc::new(Box::new(stash_item_repository)));
        let result = service.get_stash_items_by_product_id(stash_item.product_id());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![stash_item]);
    }

    #[test]
    fn test_get_stash_item_by_product_id_and_expiry_date() {
        let mut stash_item_repository = MockStashItemRepository::new();
        let stash_item = StashItem::new(
            uuid::Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(1).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );
        let returned_stash_item = stash_item.clone();

        stash_item_repository
            .expect_find_by_product_id_and_expiry_date()
            .with(
                eq(stash_item.product_id().clone()),
                eq(stash_item.expiry_date().clone()),
            )
            .returning(move |_, _| Ok(Some(returned_stash_item.clone())));
        let service = StashItemService::new(Arc::new(Box::new(stash_item_repository)));
        let result = service.get_stash_item_by_product_id_and_expiry_date(
            stash_item.product_id(),
            stash_item.expiry_date(),
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(stash_item));
    }

    #[test]
    fn test_get_stash_item_by_product_id_and_expiry_date_not_found() {
        let mut stash_item_repository = MockStashItemRepository::new();

        stash_item_repository
            .expect_find_by_product_id_and_expiry_date()
            .returning(move |_, _| Ok(None));
        let service = StashItemService::new(Arc::new(Box::new(stash_item_repository)));
        let result = service
            .get_stash_item_by_product_id_and_expiry_date(
                &"ID".parse().unwrap(),
                &NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            )
            .unwrap();

        assert_eq!(result, None);
    }

    #[test]
    fn test_get_stash_items_expiring_before() {
        let date_1 = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let date_2 = NaiveDate::from_ymd_opt(2021, 1, 2).unwrap();

        let mut stash_item_repository = MockStashItemRepository::new();
        let stash_item = StashItem::new(
            uuid::Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(1).unwrap(),
            date_1.clone(),
        );
        let returned_stash_item = stash_item.clone();

        stash_item_repository
            .expect_find_all_expiring_before()
            .with(eq(date_2.clone()))
            .returning(move |_| Ok(vec![returned_stash_item.clone()]));
        let service = StashItemService::new(Arc::new(Box::new(stash_item_repository)));
        let result = service.get_stash_items_expiring_before(&date_2).unwrap();

        assert_eq!(result, vec![stash_item]);
    }

    #[test]
    fn test_save_stash_item() {
        let mut stash_item_repository = MockStashItemRepository::new();
        let stash_item = StashItem::new(
            uuid::Uuid::new_v4(),
            "ID".parse().unwrap(),
            Quantity::new(1).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );

        stash_item_repository
            .expect_save()
            .with(eq(stash_item.clone()))
            .returning(|_| Ok(()));
        let service = StashItemService::new(Arc::new(Box::new(stash_item_repository)));
        let result = service.save_stash_item(stash_item);

        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_stash_item_by_id() {
        let mut stash_item_repository = MockStashItemRepository::new();
        let id = Uuid::new_v4();

        stash_item_repository
            .expect_delete()
            .with(eq(id.clone()))
            .returning(|_| Ok(()));
        let service = StashItemService::new(Arc::new(Box::new(stash_item_repository)));
        let result = service.delete_stash_item_by_id(&id);

        assert!(result.is_ok());
    }
}
