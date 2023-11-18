use actix_web::{web::Data, App, HttpServer};
use rsstash::{
    application::{services::ProductService, usecases::GetProductById},
    domain::errors::ProductRepositoryError,
    get_services,
};

#[actix_web::get("/")]
async fn index(product_service: Data<ProductService>) -> String {
    let result = product_service.get_product_by_id(&"1".parse().unwrap());

    match result {
        Ok(None) => "Product not found".to_string(),
        Ok(Some(product)) => format!("{}, {}", product.brand(), product.name()),
        Err(ProductRepositoryError::ProductIdError(_)) => "Invalid product id".to_string(),
        Err(ProductRepositoryError::BrandError(_)) => "Invalid brand".to_string(),
        Err(ProductRepositoryError::PersisteneError(error)) => error,
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (product_service, stash_item_service) = get_services().unwrap();

    let product_service = Data::new(product_service);
    let stash_item_service = Data::new(stash_item_service);

    HttpServer::new(move || {
        App::new()
            .app_data(product_service.clone())
            .app_data(stash_item_service.clone())
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
