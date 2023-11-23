use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::{
    application::{services::ProductService, use_cases::DeleteStashItem},
    domain::{errors::ProductRepositoryError, value_objects::ProductId},
};

pub async fn delete_stash_item(
    product_service: web::Data<ProductService>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let product_id = match path.0.parse::<ProductId>() {
        Ok(product_id) => product_id,
        Err(err) => return HttpResponse::BadRequest().body(format!("Invalid product id: {}", err)),
    };

    let stash_item_id = match Uuid::parse_str(path.1.as_str()) {
        Ok(stash_item_id) => stash_item_id,
        Err(err) => {
            return HttpResponse::BadRequest().body(format!("Invalid stash item id: {}", err))
        }
    };

    match product_service.delete_stash_item(&product_id, &stash_item_id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(ProductRepositoryError::ProductNotFound) => {
            HttpResponse::NotFound().body("Product not found")
        }
        Err(ProductRepositoryError::StashItemNotFound) => {
            HttpResponse::NotFound().body("Stash item not found")
        }
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}
