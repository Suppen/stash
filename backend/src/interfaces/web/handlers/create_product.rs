use actix_web::{web, HttpResponse};

use crate::{
    application::{services::ProductService, use_cases::CreateProduct},
    domain::{entities::Product, errors::ProductRepositoryError},
    interfaces::web::dtos::ProductDTO,
};

pub async fn create_product(
    product_service: web::Data<ProductService>,
    product_dto: web::Json<ProductDTO>,
) -> HttpResponse {
    let product = match Product::try_from(product_dto.into_inner()) {
        Ok(product) => product,
        Err(err) => return HttpResponse::BadRequest().body(format!("{}", err)),
    };

    match product_service.create_product(product) {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(ProductRepositoryError::ProductAlreadyExists) => {
            HttpResponse::Conflict().body("Product already exists")
        }
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
