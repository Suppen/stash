use actix_web::{web, HttpResponse};

use crate::{
    application::{services::ProductService, use_cases::UpdateProductById},
    domain::{entities::Product, errors::ProductRepositoryError, value_objects::ProductId},
    interfaces::web::dtos::ProductDTO,
};

pub async fn update_product_by_id(
    product_service: web::Data<ProductService>,
    path: web::Path<String>,
    product_dto: web::Json<ProductDTO>,
) -> HttpResponse {
    let product_id = match path.parse::<ProductId>() {
        Ok(product_id) => product_id,
        Err(err) => return HttpResponse::BadRequest().body(format!("{}", err)),
    };

    let product = match Product::try_from(product_dto.into_inner()) {
        Ok(product) => product,
        Err(err) => return HttpResponse::BadRequest().body(format!("{}", err)),
    };

    if &product_id != product.id() {
        return HttpResponse::BadRequest().body("Product id mismatch");
    }

    match product_service.update_product_by_id(&product_id, product) {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(ProductRepositoryError::ProductNotFound) => {
            HttpResponse::NotFound().body("Product not found")
        }
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
