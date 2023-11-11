use std::sync::Arc;

use crate::{application::usecases::GetStashItemById, repositories::StashItemRepository};

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::quantity::Quantity;
    use crate::domain::stash_item::StashItem;
    use crate::repositories::MockStashItemRepository;
    use mockall::predicate::*;

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
}
