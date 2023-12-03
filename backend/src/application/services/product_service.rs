use std::{collections::HashSet, sync::Arc};

use crate::{
    application::use_cases::{
        AddStashItem, CreateProduct, DeleteProduct, DeleteStashItem, GetProduct,
        GetProductByStashItemId, GetProductsExpiringBefore, GetStashItems, UpdateProduct,
        UpdateStashItem,
    },
    domain::{
        entities::{Product, StashItem},
        errors::ProductRepositoryError,
        repositories::ProductRepository,
        value_objects::ProductId,
    },
};

pub struct ProductService {
    product_repository: Arc<Box<dyn ProductRepository>>,
}

impl ProductService {
    pub fn new(product_repository: Arc<Box<dyn ProductRepository>>) -> Self {
        Self { product_repository }
    }
}

impl GetProduct for ProductService {
    fn get_product(&self, id: &ProductId) -> Result<Option<Product>, ProductRepositoryError> {
        self.product_repository.find_by_id(id)
    }
}

impl CreateProduct for ProductService {
    fn create_product(&self, product: Product) -> Result<Product, ProductRepositoryError> {
        // Clone the ID so we can use it to fetch the product after saving it
        let product_id = product.id().clone();

        if self.product_repository.exists_by_id(&product.id())? {
            return Err(ProductRepositoryError::ProductAlreadyExists);
        }

        match self.product_repository.save(product) {
            Ok(()) => match self.product_repository.find_by_id(&product_id) {
                Ok(Some(product)) => Ok(product),
                // This should never happen; we just created it!
                Ok(None) => panic!("Product not found after saving"),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

impl UpdateProduct for ProductService {
    fn update_product(
        &self,
        id: &ProductId,
        product: Product,
    ) -> Result<Product, ProductRepositoryError> {
        // Clone the ID so we can use it to fetch the product after saving it
        let product_id = product.id().clone();

        if !self.product_repository.exists_by_id(id)? {
            return Err(ProductRepositoryError::ProductNotFound);
        }

        match self.product_repository.save(product) {
            Ok(()) => match self.product_repository.find_by_id(&product_id) {
                Ok(Some(product)) => Ok(product),
                // This should never happen; we just created it!
                Ok(None) => panic!("Product not found after saving"),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

impl DeleteProduct for ProductService {
    fn delete_product(&self, id: &ProductId) -> Result<(), ProductRepositoryError> {
        self.product_repository.delete_by_id(id)
    }
}

impl AddStashItem for ProductService {
    fn add_stash_item(
        &self,
        product_id: &ProductId,
        stash_item: StashItem,
    ) -> Result<(), ProductRepositoryError> {
        let mut product = match self.product_repository.find_by_id(product_id)? {
            Some(product) => product,
            None => return Err(ProductRepositoryError::ProductNotFound),
        };

        product.add_stash_item(stash_item)?;

        self.product_repository.save(product)?;

        // TODO No return value?
        Ok(())
    }
}

impl UpdateStashItem for ProductService {
    fn update_stash_item(
        &self,
        product_id: &ProductId,
        stash_item: StashItem,
    ) -> Result<StashItem, ProductRepositoryError> {
        // Clone the ID so we can find the stash item after saving it
        let stash_item_id = stash_item.id().clone();

        let mut product = match self.product_repository.find_by_id(product_id)? {
            Some(product) => product,
            None => return Err(ProductRepositoryError::ProductNotFound),
        };

        product.update_stash_item(stash_item)?;

        self.product_repository.save(product)?;

        match self.product_repository.find_by_id(product_id) {
            Ok(Some(product)) => {
                let si = product
                    .stash_items()
                    .iter()
                    .find(|x| x.id() == &stash_item_id)
                    .map(|x| *x);

                match si {
                    Some(stash_item) => Ok(stash_item.clone()),
                    None => panic!("Stash item not found after saving"),
                }
            }
            Ok(None) => panic!("Product not found after saving"),
            Err(e) => Err(e),
        }
    }
}

impl DeleteStashItem for ProductService {
    fn delete_stash_item(
        &self,
        product_id: &ProductId,
        stash_item_id: &uuid::Uuid,
    ) -> Result<(), ProductRepositoryError> {
        let mut product = match self.product_repository.find_by_id(product_id)? {
            Some(product) => product,
            None => return Err(ProductRepositoryError::ProductNotFound),
        };

        product.remove_stash_item(stash_item_id)?;

        self.product_repository.save(product)?;

        Ok(())
    }
}

impl GetStashItems for ProductService {
    fn get_stash_items(
        &self,
        product_id: &ProductId,
    ) -> Result<HashSet<StashItem>, ProductRepositoryError> {
        let product = match self.product_repository.find_by_id(product_id)? {
            Some(product) => product,
            None => return Err(ProductRepositoryError::ProductNotFound),
        };

        let stash_items = product
            .stash_items()
            .into_iter()
            .map(|x| x.clone())
            .collect();

        Ok(stash_items)
    }
}

impl GetProductByStashItemId for ProductService {
    fn get_product_by_stash_item_id(
        &self,
        stash_item_id: &uuid::Uuid,
    ) -> Result<Option<Product>, ProductRepositoryError> {
        self.product_repository.find_by_stash_item_id(stash_item_id)
    }
}

impl GetProductsExpiringBefore for ProductService {
    fn products_expiring_before(
        &self,
        before: chrono::NaiveDate,
    ) -> Result<Vec<Product>, ProductRepositoryError> {
        self.product_repository
            .find_expiring_in_interval(None, Some(before))
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use mockall::predicate::eq;
    use uuid::Uuid;

    use crate::domain::{
        entities::{FakeProduct, FakeStashItem},
        repositories::MockProductRepository,
    };

    use super::*;

    #[test]
    fn test_get_product_by_id() {
        let product = FakeProduct::new().build();
        let product_id = product.id().clone();
        let returned_product = product.clone();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_find_by_id()
            .with(eq(product_id.clone()))
            .returning(move |_| Ok(Some(returned_product.clone())));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let found_product = product_service.get_product(&product_id).unwrap().unwrap();

        assert_eq!(found_product, product);
    }

    #[test]
    fn test_get_product_by_id_not_found() {
        let product_id: ProductId = "ID".parse().unwrap();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_find_by_id()
            .with(eq(product_id.clone()))
            .returning(|_| Ok(None));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let found_product = product_service.get_product(&product_id).unwrap();

        assert!(found_product.is_none());
    }

    #[test]
    fn test_create_product() {
        let product = FakeProduct::new().build();
        let product_id = product.id().clone();
        let returned_product = product.clone();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_exists_by_id()
            .with(eq(product_id.clone()))
            .returning(|_| Ok(false));
        product_repository
            .expect_save()
            .with(eq(product.clone()))
            .returning(|_| Ok(()));
        product_repository
            .expect_find_by_id()
            .with(eq(product_id.clone()))
            .returning(move |_| Ok(Some(returned_product.clone())));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let created_product = product_service.create_product(product.clone()).unwrap();

        assert_eq!(created_product, product);
    }

    #[test]
    fn test_create_product_already_exists() {
        let product = FakeProduct::new().build();
        let product_id = product.id().clone();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_exists_by_id()
            .with(eq(product_id.clone()))
            .returning(|_| Ok(true));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let created_product = product_service.create_product(product.clone());

        assert_eq!(
            created_product.unwrap_err(),
            ProductRepositoryError::ProductAlreadyExists
        );
    }

    #[test]
    fn test_update_product() {
        let product = FakeProduct::new().build();
        let product_id = product.id().clone();
        let returned_product = product.clone();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_exists_by_id()
            .with(eq(product_id.clone()))
            .returning(|_| Ok(true));
        product_repository
            .expect_save()
            .with(eq(product.clone()))
            .returning(|_| Ok(()));
        product_repository
            .expect_find_by_id()
            .with(eq(product_id.clone()))
            .returning(move |_| Ok(Some(returned_product.clone())));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let updated_product = product_service
            .update_product(&product_id, product.clone())
            .unwrap();

        assert_eq!(updated_product, product);
    }

    #[test]
    fn test_update_product_not_found() {
        let product = FakeProduct::new().build();
        let product_id = product.id().clone();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_exists_by_id()
            .with(eq(product_id.clone()))
            .returning(|_| Ok(false));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let updated_product = product_service.update_product(&product_id, product.clone());

        assert_eq!(
            updated_product.unwrap_err(),
            ProductRepositoryError::ProductNotFound
        );
    }

    #[test]
    fn test_delete_product() {
        let product_id: ProductId = "ID".parse().unwrap();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_delete_by_id()
            .with(eq(product_id.clone()))
            .returning(|_| Ok(()));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let deleted_product = product_service.delete_product(&product_id);

        assert!(deleted_product.is_ok());
    }

    #[test]
    fn test_delete_product_not_found() {
        let product_id: ProductId = "ID".parse().unwrap();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_delete_by_id()
            .with(eq(product_id.clone()))
            .returning(|_| Ok(()));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let deleted_product = product_service.delete_product(&product_id);

        assert!(deleted_product.is_ok());
    }

    #[test]
    fn test_add_stash_item() {
        let product = FakeProduct::new().with_stash_items(Vec::new()).build();
        let product_id = product.id().clone();
        let stash_item = FakeStashItem::new().build();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_find_by_id()
            .with(eq(product_id.clone()))
            .returning(move |_| Ok(Some(product.clone())));
        product_repository
            .expect_save()
            .withf(move |product| product.stash_items().len() == 1)
            .returning(|_| Ok(()));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let result = product_service.add_stash_item(&product_id, stash_item);

        assert!(result.is_ok());
    }

    #[test]
    fn test_update_stash_item() {
        let stash_item_id = Uuid::new_v4();
        let product = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new().with_id(stash_item_id).build()])
            .build();
        let product_id = product.id().clone();
        let stash_item = FakeStashItem::new().with_id(stash_item_id).build();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_find_by_id()
            .with(eq(product_id.clone()))
            .returning(move |_| Ok(Some(product.clone())));
        product_repository.expect_save().returning(|_| Ok(()));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let result = product_service.update_stash_item(&product_id, stash_item.clone());

        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_stash_item() {
        let stash_item_id = Uuid::new_v4();
        let product = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new().with_id(stash_item_id).build()])
            .build();
        let product_id = product.id().clone();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_find_by_id()
            .with(eq(product_id.clone()))
            .returning(move |_| Ok(Some(product.clone())));
        product_repository
            .expect_save()
            .withf(move |product| product.stash_items().len() == 0)
            .returning(|_| Ok(()));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let result = product_service.delete_stash_item(&product_id, &stash_item_id);

        assert!(result.is_ok());
    }

    #[test]
    fn test_get_stash_items() {
        let expected_stash_items = vec![FakeStashItem::new().build(), FakeStashItem::new().build()];
        let product = FakeProduct::new()
            .with_stash_items(expected_stash_items.clone())
            .build();
        let product_id = product.id().clone();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_find_by_id()
            .with(eq(product_id.clone()))
            .returning(move |_| Ok(Some(product.clone())));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let result = product_service.get_stash_items(&product_id);

        assert!(result.is_ok());

        let stash_items = result.unwrap();

        for stash_item in expected_stash_items {
            assert!(stash_items.contains(&stash_item));
        }
    }

    #[test]
    fn test_get_product_by_stash_item_id() {
        let stash_item = FakeStashItem::new().build();
        let stash_item_id = stash_item.id().clone();
        let product = FakeProduct::new()
            .with_stash_items(vec![stash_item])
            .build();
        let returned_product = product.clone();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_find_by_stash_item_id()
            .with(eq(stash_item_id.clone()))
            .returning(move |_| Ok(Some(returned_product.clone())));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let result = product_service.get_product_by_stash_item_id(&stash_item_id);

        let found_product = result.unwrap().unwrap();

        assert_eq!(found_product, product);
    }

    #[test]
    fn test_get_products_expiring_before() {
        let product = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new()
                .with_expiry_date(NaiveDate::from_ymd_opt(2023, 01, 01).unwrap())
                .build()])
            .build();
        let returned_product = product.clone();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_find_expiring_in_interval()
            .with(
                eq(None),
                eq(Some(NaiveDate::from_ymd_opt(2023, 01, 02).unwrap())),
            )
            .returning(move |_, _| Ok(vec![returned_product.clone()]));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let result = product_service
            .products_expiring_before(NaiveDate::from_ymd_opt(2023, 01, 02).unwrap())
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], product);
    }
}
