use std::sync::Arc;

use crate::{
    application::use_cases::{CreateProduct, DeleteProductById, GetProductById, UpdateProductById},
    domain::{
        entities::Product, errors::ProductRepositoryError, repositories::ProductRepository,
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

impl GetProductById for ProductService {
    fn get_product_by_id(&self, id: &ProductId) -> Result<Option<Product>, ProductRepositoryError> {
        self.product_repository.find_by_id(id)
    }
}

impl CreateProduct for ProductService {
    fn create_product(&self, product: Product) -> Result<(), ProductRepositoryError> {
        if self.product_repository.exists_by_id(&product.id())? {
            return Err(ProductRepositoryError::ProductAlreadyExists);
        }

        self.product_repository.save(product)
    }
}

impl UpdateProductById for ProductService {
    fn update_product_by_id(
        &self,
        id: &ProductId,
        product: Product,
    ) -> Result<(), ProductRepositoryError> {
        if !self.product_repository.exists_by_id(id)? {
            return Err(ProductRepositoryError::ProductNotFound);
        }

        self.product_repository.save(product)
    }
}

impl DeleteProductById for ProductService {
    fn delete_product_by_id(&self, id: &ProductId) -> Result<(), ProductRepositoryError> {
        self.product_repository.delete_by_id(id)
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::domain::repositories::MockProductRepository;

    use super::*;

    #[test]
    fn test_get_product_by_id() {
        let product_id: ProductId = "ID".parse().unwrap();
        let product = Product::new(product_id.clone(), "BRAND".parse().unwrap(), "NAME", vec![]);
        let returned_product = product.clone();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_find_by_id()
            .with(eq(product_id.clone()))
            .returning(move |_| Ok(Some(returned_product.clone())));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let found_product = product_service
            .get_product_by_id(&product_id)
            .unwrap()
            .unwrap();

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

        let found_product = product_service.get_product_by_id(&product_id).unwrap();

        assert!(found_product.is_none());
    }

    #[test]
    fn test_delete_product_by_id() {
        let product_id: ProductId = "ID".parse().unwrap();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_delete_by_id()
            .with(eq(product_id.clone()))
            .returning(|_| Ok(()));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let deleted_product = product_service.delete_product_by_id(&product_id);

        assert!(deleted_product.is_ok());
    }

    #[test]
    fn test_delete_product_by_id_not_found() {
        let product_id: ProductId = "ID".parse().unwrap();

        let mut product_repository = MockProductRepository::new();
        product_repository
            .expect_delete_by_id()
            .with(eq(product_id.clone()))
            .returning(|_| Ok(()));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let deleted_product = product_service.delete_product_by_id(&product_id);

        assert!(deleted_product.is_ok());
    }
}
