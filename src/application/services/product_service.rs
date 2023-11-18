use std::sync::Arc;

use crate::{
    application::usecases::{DeleteProductById, GetProductById, SaveProduct},
    domain::product::{Product, ProductId},
    repositories::ProductRepository,
};

pub struct ProductService<E: std::error::Error> {
    product_repository: Arc<Box<dyn ProductRepository<E>>>,
}

impl<E: std::error::Error + Send + Sync> ProductService<E> {
    pub fn new(product_repository: Arc<Box<dyn ProductRepository<E>>>) -> Self {
        Self { product_repository }
    }
}

impl<E: std::error::Error + Send + Sync> GetProductById<E> for ProductService<E> {
    fn get_product_by_id(&self, id: &ProductId) -> Result<Option<Product>, E> {
        self.product_repository.find_by_id(id)
    }
}

impl<E: std::error::Error + Send + Sync> SaveProduct<E> for ProductService<E> {
    fn save_product(&self, product: Product) -> Result<(), E> {
        self.product_repository.save(product)
    }
}

impl<E: std::error::Error + Send + Sync> DeleteProductById<E> for ProductService<E> {
    fn delete_product_by_id(&self, id: &ProductId) -> Result<(), E> {
        self.product_repository.delete_by_id(id)
    }
}

#[cfg(test)]
mod test {
    use mockall::predicate::eq;

    use crate::repositories::MockProductRepository;

    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct TestError;
    impl std::error::Error for TestError {}
    impl std::fmt::Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }

    #[test]
    fn test_get_product_by_id() {
        let product_id: ProductId = "ID".parse().unwrap();
        let product = Product::new(product_id.clone(), "BRAND".parse().unwrap(), "NAME");
        let returned_product = product.clone();

        let mut product_repository = MockProductRepository::<TestError>::new();
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

        let mut product_repository = MockProductRepository::<TestError>::new();
        product_repository
            .expect_find_by_id()
            .with(eq(product_id.clone()))
            .returning(|_| Ok(None));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let found_product = product_service.get_product_by_id(&product_id).unwrap();

        assert!(found_product.is_none());
    }

    #[test]
    fn test_save_product() {
        let product = Product::new("ID".parse().unwrap(), "BRAND".parse().unwrap(), "NAME");

        let mut product_repository = MockProductRepository::<TestError>::new();
        product_repository
            .expect_save()
            .with(eq(product.clone()))
            .returning(|_| Ok(()));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let saved_product = product_service.save_product(product);

        assert!(saved_product.is_ok());
    }

    #[test]
    fn test_delete_product_by_id() {
        let product_id: ProductId = "ID".parse().unwrap();

        let mut product_repository = MockProductRepository::<TestError>::new();
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

        let mut product_repository = MockProductRepository::<TestError>::new();
        product_repository
            .expect_delete_by_id()
            .with(eq(product_id.clone()))
            .returning(|_| Ok(()));

        let product_service = ProductService::new(Arc::new(Box::new(product_repository)));

        let deleted_product = product_service.delete_product_by_id(&product_id);

        assert!(deleted_product.is_ok());
    }
}
