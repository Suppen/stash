use std::sync::{Arc, Mutex};

use actix_web::{web::Data, App, HttpServer};
use rsstash::{
    application::services::ProductService,
    domain::repositories::ProductRepository as ProductRepositoryTrait,
    infrastructure::persistence::sqlite::{db::setup_db, ProductRepository},
    interfaces::web::v1::router::configure_routes,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create the database connection
    let connection = match std::env::var("STASH_DB_PATH") {
        Ok(path) => {
            println!("Using database at {}", path);
            rusqlite::Connection::open(path).unwrap()
        }
        Err(_) => {
            eprintln!("No database path provided, using in-memory database");
            rusqlite::Connection::open_in_memory().unwrap()
        }
    };

    // Setup the database
    setup_db(&connection).unwrap();

    // Make the connection shareable
    let shared_connection = Arc::new(Mutex::new(connection));

    // Create the repositories
    let product_repository: Box<dyn ProductRepositoryTrait> =
        Box::new(ProductRepository::new(shared_connection.clone()));

    // Make the repositories shareable
    let product_repository = Arc::new(product_repository);

    // Create the services
    let product_service = ProductService::new(product_repository.clone());

    // Create the web server state
    let product_service = Data::new(product_service);

    // Spin up the web server
    HttpServer::new(move || {
        App::new()
            .app_data(product_service.clone())
            .configure(configure_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
