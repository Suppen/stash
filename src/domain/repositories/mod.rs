mod product_repository;

pub use product_repository::ProductRepository;

#[cfg(test)]
pub use product_repository::MockProductRepository;
