use actix_web::{web, HttpResponse};

use crate::{
    application::{services::ProductService, usecases::SaveProduct},
    domain::entities::Product,
    interfaces::web::dtos::ProductDTO,
};

pub async fn save_product(
    product_service: web::Data<ProductService>,
    body: web::Json<ProductDTO>,
) -> HttpResponse {
    let product = match Product::try_from(body.into_inner()) {
        Ok(product) => product,
        Err(err) => return HttpResponse::BadRequest().body(format!("Invalid product: {}", err)),
    };

    match product_service.save_product(product) {
        Ok(()) => HttpResponse::Ok().body("Product saved"),
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
