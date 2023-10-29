use std::sync::{Arc, Mutex};

use rusqlite::Connection;

mod domain;
mod infrastructure;
mod repositories;

pub fn do_stuff() -> Result<(), String> {
    let connection = Connection::open_in_memory().unwrap();
    infrastructure::sqlite::setup_db(&connection).unwrap();

    let shared_connection = Arc::new(Mutex::new(connection));

    let _product_repository =
        infrastructure::sqlite::ProductRepository::new(shared_connection.clone());
    let _stash_item_repository =
        infrastructure::sqlite::StashItemRepository::new(shared_connection.clone());

    Ok(())
}
