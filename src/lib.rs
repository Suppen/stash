use std::sync::{Arc, Mutex};

use application::services::{ProductService, StashItemService};
use domain::repositories::{ProductRepository, StashItemRepository};
use infrastructure::persistence::sqlite::{
    db::setup_db, ProductRepositoryError, StashItemRepositoryError,
};

pub mod application;
pub mod domain;
pub mod infrastructure;

pub fn get_services() -> Result<
    (
        ProductService<ProductRepositoryError>,
        StashItemService<StashItemRepositoryError>,
    ),
    String,
> {
    let connection = rusqlite::Connection::open_in_memory().map_err(|e| e.to_string())?;
    setup_db(&connection).map_err(|e| e.to_string())?;

    let shared_connection = Arc::new(Mutex::new(connection));

    let product_repository: Box<dyn ProductRepository<ProductRepositoryError>> = Box::new(
        infrastructure::persistence::sqlite::ProductRepository::new(shared_connection.clone()),
    );
    let stash_item_repository: Box<dyn StashItemRepository<StashItemRepositoryError>> = Box::new(
        infrastructure::persistence::sqlite::StashItemRepository::new(shared_connection.clone()),
    );

    let shared_product_repository = Arc::new(product_repository);
    let shared_stash_item_repository = Arc::new(stash_item_repository);

    let product_service =
        application::services::ProductService::new(shared_product_repository.clone());

    let stash_item_service =
        application::services::StashItemService::new(shared_stash_item_repository.clone());

    Ok((product_service, stash_item_service))
}
