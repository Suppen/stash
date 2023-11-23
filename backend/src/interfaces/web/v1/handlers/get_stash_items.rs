use actix_web::{web, HttpResponse};

use crate::{
    application::{services::ProductService, use_cases::GetStashItems},
    domain::{errors::ProductRepositoryError, value_objects::ProductId},
    interfaces::web::v1::dtos::StashItemDTO,
};

pub async fn get_stash_items(
    product_service: web::Data<ProductService>,
    path: web::Path<String>,
) -> HttpResponse {
    let product_id = match path.parse::<ProductId>() {
        Ok(product_id) => product_id,
        Err(err) => return HttpResponse::BadRequest().body(format!("Invalid product id: {}", err)),
    };

    match product_service.get_stash_items(&product_id) {
        Ok(stash_items) => HttpResponse::Ok().json(
            stash_items
                .into_iter()
                .map(StashItemDTO::from)
                .collect::<Vec<_>>(),
        ),
        Err(ProductRepositoryError::ProductNotFound) => {
            HttpResponse::NotFound().body("Product not found")
        }
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}
