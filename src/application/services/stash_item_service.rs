use std::sync::Arc;

use crate::{
    application::usecases::{
        GetStashItemById, GetStashItemByProductIdAndExpiryDate, GetStashItemsExpiringBefore,
        SaveStashItem,
    },
    repositories::StashItemRepository,
};

pub struct StashItemService<E> {
    pub stash_item_repository: Arc<dyn StashItemRepository<E>>,
}

impl<E> StashItemService<E> {
    pub fn new(stash_item_repository: Arc<dyn StashItemRepository<E>>) -> Self {
        Self {
            stash_item_repository,
        }
    }
}

impl<E> GetStashItemById<E> for StashItemService<E> {
    fn get_stash_item_by_id(
        &self,
        id: &uuid::Uuid,
    ) -> Result<Option<crate::domain::stash_item::StashItem>, E> {
        self.stash_item_repository.find_by_id(id)
    }
}

impl<E> GetStashItemByProductIdAndExpiryDate<E> for StashItemService<E> {
    fn get_stash_item_by_product_id_and_expiry_date(
        &self,
        product_id: &crate::domain::product::ProductId,
        expiry_date: &chrono::NaiveDate,
    ) -> Result<Option<crate::domain::stash_item::StashItem>, E> {
        self.stash_item_repository
            .find_by_product_id_and_expiry_date(product_id, expiry_date)
    }
}

impl<E> GetStashItemsExpiringBefore<E> for StashItemService<E> {
    fn get_stash_items_expiring_before(
        &self,
        date: &chrono::NaiveDate,
    ) -> Result<Vec<crate::domain::stash_item::StashItem>, E> {
        self.stash_item_repository.find_all_expiring_before(date)
    }
}

impl<E> SaveStashItem<E> for StashItemService<E> {
    fn save_stash_item(&self, stash_item: crate::domain::stash_item::StashItem) -> Result<(), E> {
        self.stash_item_repository.save(stash_item)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::quantity::Quantity;
    use crate::domain::stash_item::StashItem;
    use crate::repositories::MockStashItemRepository;
    use chrono::NaiveDate;
    use mockall::predicate::*;
    use uuid::Uuid;

    #[derive(Debug)]
    struct TestError;

    #[test]
    fn test_get_stash_item_by_id() {
        let mut stash_item_repository = MockStashItemRepository::<TestError>::new();
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
        let service = StashItemService::new(Arc::new(stash_item_repository));
        let result = service.get_stash_item_by_id(&stash_item.id());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(stash_item));
    }

    #[test]
    fn test_get_stash_item_by_id_not_found() {
        let mut stash_item_repository = MockStashItemRepository::<TestError>::new();

        stash_item_repository
            .expect_find_by_id()
            .returning(move |_| Ok(None));
        let service = StashItemService::new(Arc::new(stash_item_repository));
        let result = service.get_stash_item_by_id(&Uuid::new_v4()).unwrap();

        assert_eq!(result, None);
    }

    #[test]
    fn test_get_stash_item_by_product_id_and_expiry_date() {
        let mut stash_item_repository = MockStashItemRepository::<TestError>::new();
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
        let service = StashItemService::new(Arc::new(stash_item_repository));
        let result = service.get_stash_item_by_product_id_and_expiry_date(
            stash_item.product_id(),
            stash_item.expiry_date(),
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(stash_item));
    }

    #[test]
    fn test_get_stash_item_by_product_id_and_expiry_date_not_found() {
        let mut stash_item_repository = MockStashItemRepository::<TestError>::new();

        stash_item_repository
            .expect_find_by_product_id_and_expiry_date()
            .returning(move |_, _| Ok(None));
        let service = StashItemService::new(Arc::new(stash_item_repository));
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

        let mut stash_item_repository = MockStashItemRepository::<TestError>::new();
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
        let service = StashItemService::new(Arc::new(stash_item_repository));
        let result = service.get_stash_items_expiring_before(&date_2).unwrap();

        assert_eq!(result, vec![stash_item]);
    }

    #[test]
    fn test_save_stash_item() {
        let mut stash_item_repository = MockStashItemRepository::<TestError>::new();
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
        let service = StashItemService::new(Arc::new(stash_item_repository));
        let result = service.save_stash_item(stash_item);

        assert!(result.is_ok());
    }
}
