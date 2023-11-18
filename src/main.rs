use std::sync::{Arc, Mutex};

use actix_web::{web::Data, App, HttpServer};
use rsstash::{
    application::services::{ProductService, StashItemService},
    domain::repositories::{
        ProductRepository as ProductRepositoryTrait,
        StashItemRepository as StashItemRepositoryTrait,
    },
    infrastructure::persistence::sqlite::{db::setup_db, ProductRepository, StashItemRepository},
    interfaces::web::router::configure_routes,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up the connection to the database
    let connection = rusqlite::Connection::open_in_memory().unwrap();
    setup_db(&connection).unwrap();
    let shared_connection = Arc::new(Mutex::new(connection));

    // Create the repositories
    let product_repository: Box<dyn ProductRepositoryTrait> =
        Box::new(ProductRepository::new(shared_connection.clone()));
    let stash_item_repository: Box<dyn StashItemRepositoryTrait> =
        Box::new(StashItemRepository::new(shared_connection.clone()));

    // Make the repositories shareable
    let product_repository = Arc::new(product_repository);
    let stash_item_repository = Arc::new(stash_item_repository);

    // Create the services
    let product_service = ProductService::new(product_repository.clone());
    let stash_item_service = StashItemService::new(stash_item_repository.clone());

    // Create the web server state
    let product_service = Data::new(product_service);
    let stash_item_service = Data::new(stash_item_service);

    // Spin up the web server
    HttpServer::new(move || {
        App::new()
            .app_data(product_service.clone())
            .app_data(stash_item_service.clone())
            .configure(configure_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
