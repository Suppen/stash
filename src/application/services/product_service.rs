use std::sync::Arc;

use crate::{
    application::usecases::{DeleteProductById, GetProductById, SaveProduct},
    domain::product::{Product, ProductId},
    repositories::ProductRepository,
};

pub struct ProductService<E> {
    product_repository: Arc<dyn ProductRepository<E>>,
}

impl<E> ProductService<E> {
    pub fn new(product_repository: Arc<dyn ProductRepository<E>>) -> Self {
        Self { product_repository }
    }
}

impl<E> GetProductById<E> for ProductService<E> {
    fn get_product_by_id(&self, id: &ProductId) -> Result<Option<Product>, E> {
        self.product_repository.find_by_id(id)
    }
}

impl<E> SaveProduct<E> for ProductService<E> {
    fn save_product(&self, product: Product) -> Result<(), E> {
        self.product_repository.save(product)
    }
}

impl<E> DeleteProductById<E> for ProductService<E> {
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

    #[test]
    fn test_get_product_by_id() {
        let product_id: ProductId = "ID".parse().unwrap();
        let product = Product::new(product_id.clone(), "BRAND".parse().unwrap(), "NAME");
        let returned_product = product.clone();

        let mut mock_product_repository = MockProductRepository::<TestError>::new();
        mock_product_repository
            .expect_find_by_id()
            .with(eq(product_id.clone()))
            .returning(move |_| Ok(Some(returned_product.clone())));

        let product_service = ProductService::new(Arc::new(mock_product_repository));

        let found_product = product_service
            .get_product_by_id(&product_id)
            .unwrap()
            .unwrap();

        assert_eq!(found_product, product);
    }

    #[test]
    fn test_get_product_by_id_not_found() {
        let product_id: ProductId = "ID".parse().unwrap();

        let mut mock_product_repository = MockProductRepository::<TestError>::new();
        mock_product_repository
            .expect_find_by_id()
            .with(eq(product_id.clone()))
            .returning(|_| Ok(None));

        let product_service = ProductService::new(Arc::new(mock_product_repository));

        let found_product = product_service.get_product_by_id(&product_id).unwrap();

        assert!(found_product.is_none());
    }

    #[test]
    fn test_save_product() {
        let product = Product::new("ID".parse().unwrap(), "BRAND".parse().unwrap(), "NAME");

        let mut mock_product_repository = MockProductRepository::<TestError>::new();
        mock_product_repository
            .expect_save()
            .with(eq(product.clone()))
            .returning(|_| Ok(()));

        let product_service = ProductService::new(Arc::new(mock_product_repository));

        let saved_product = product_service.save_product(product);

        assert!(saved_product.is_ok());
    }

    #[test]
    fn test_delete_product_by_id() {
        let product_id: ProductId = "ID".parse().unwrap();

        let mut mock_product_repository = MockProductRepository::<TestError>::new();
        mock_product_repository
            .expect_delete_by_id()
            .with(eq(product_id.clone()))
            .returning(|_| Ok(()));

        let product_service = ProductService::new(Arc::new(mock_product_repository));

        let deleted_product = product_service.delete_product_by_id(&product_id);

        assert!(deleted_product.is_ok());
    }

    #[test]
    fn test_delete_product_by_id_not_found() {
        let product_id: ProductId = "ID".parse().unwrap();

        let mut mock_product_repository = MockProductRepository::<TestError>::new();
        mock_product_repository
            .expect_delete_by_id()
            .with(eq(product_id.clone()))
            .returning(|_| Ok(()));

        let product_service = ProductService::new(Arc::new(mock_product_repository));

        let deleted_product = product_service.delete_product_by_id(&product_id);

        assert!(deleted_product.is_ok());
    }
}
