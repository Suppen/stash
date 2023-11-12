use std::sync::{Arc, Mutex};

use application::services::{ProductService, StashItemService};
use infrastructure::sqlite::{ProductRepositoryError, StashItemRepositoryError};
use repositories::{ProductRepository, StashItemRepository};
use rusqlite::Connection;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod repositories;

pub fn get_services() -> Result<
    (
        ProductService<ProductRepositoryError>,
        StashItemService<StashItemRepositoryError>,
    ),
    String,
> {
    let connection = Connection::open_in_memory().unwrap();
    infrastructure::sqlite::db::setup_db(&connection).unwrap();

    let shared_connection = Arc::new(Mutex::new(connection));

    let product_repository: Box<dyn ProductRepository<ProductRepositoryError>> = Box::new(
        infrastructure::sqlite::ProductRepository::new(shared_connection.clone()),
    );
    let stash_item_repository: Box<dyn StashItemRepository<StashItemRepositoryError>> = Box::new(
        infrastructure::sqlite::StashItemRepository::new(shared_connection.clone()),
    );

    let shared_product_repository = Arc::new(Mutex::new(product_repository));
    let shared_stash_item_repository = Arc::new(Mutex::new(stash_item_repository));

    let product_service =
        application::services::ProductService::new(shared_product_repository.clone());

    let stash_item_service =
        application::services::StashItemService::new(shared_stash_item_repository.clone());

    Ok((product_service, stash_item_service))
}
