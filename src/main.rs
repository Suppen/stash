//fn main() {
//println!("Hello, world!");
//}
use std::sync::Mutex;

use actix_web::{web::Data, App, HttpServer};
use rsstash::get_services;

#[actix_web::get("/")]
async fn index(cake: Data<Mutex<usize>>) -> String {
    // Incease the cake count
    let mut val = cake.lock().unwrap();
    *val += 1;
    format!("Hello world! {}", val.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (product_service, stash_item_service) = get_services().unwrap();

    let shared_product_service = Data::new(Mutex::new(product_service));

    let cake = Data::new(Mutex::new(0usize));

    HttpServer::new(move || {
        App::new()
            .app_data(cake.clone())
            .app_data(shared_product_service.clone())
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
