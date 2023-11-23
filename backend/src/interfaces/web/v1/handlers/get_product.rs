use actix_web::{web, HttpResponse};

use crate::{
    application::{services::ProductService, use_cases::GetProduct},
    domain::value_objects::ProductId,
    interfaces::web::v1::dtos::ProductDTO,
};

pub async fn get_product(
    product_service: web::Data<ProductService>,
    path: web::Path<String>,
) -> HttpResponse {
    let product_id = match path.parse::<ProductId>() {
        Ok(product_id) => product_id,
        Err(err) => return HttpResponse::BadRequest().body(format!("Invalid product id: {}", err)),
    };

    match product_service.get_product(&product_id) {
        Ok(None) => HttpResponse::NotFound().body("Product not found"),
        Ok(Some(product)) => {
            let product_dto = ProductDTO::from(product);
            HttpResponse::Ok().json(product_dto)
        }
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
