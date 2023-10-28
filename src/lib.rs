use std::sync::{Arc, Mutex};

use rusqlite::Connection;

mod domain;
mod infrastructure;
mod repositories;

pub fn do_stuff() -> Result<(), String> {
    let connection = Connection::open_in_memory().unwrap();
    let shared_connection = Arc::new(Mutex::new(connection));

    infrastructure::sqlite::setup_db(&shared_connection.lock().unwrap()).unwrap();

    let product_repository =
        infrastructure::sqlite::ProductRepository::new(shared_connection.clone());
    let stash_item_repository =
        infrastructure::sqlite::StashItemRepository::new(shared_connection.clone());

    Ok(())
}
