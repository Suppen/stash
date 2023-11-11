use std::sync::{Arc, Mutex};

use application::usecases::GetProductById;
use rusqlite::Connection;

mod application;
mod domain;
mod infrastructure;
mod repositories;

pub fn do_stuff() -> Result<(), String> {
    let connection = Connection::open_in_memory().unwrap();
    infrastructure::sqlite::db::setup_db(&connection).unwrap();

    let shared_connection = Arc::new(Mutex::new(connection));

    let product_repository =
        infrastructure::sqlite::ProductRepository::new(shared_connection.clone());
    let _stash_item_repository =
        infrastructure::sqlite::StashItemRepository::new(shared_connection.clone());

    let _shared_product_repository = Arc::new(product_repository);

    let product_service =
        application::services::ProductService::new(_shared_product_repository.clone());

    let product = product_service
        .get_product_by_id(&"ID".parse().unwrap())
        .unwrap();

    println!("{:?}", product);

    Ok(())
}
