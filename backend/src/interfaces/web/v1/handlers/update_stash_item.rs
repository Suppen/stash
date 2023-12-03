use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::{
    application::{services::ProductService, use_cases::UpdateStashItem},
    domain::{entities::StashItem, errors::ProductRepositoryError, value_objects::ProductId},
    interfaces::web::v1::dtos::StashItemDTO,
};

pub async fn update_stash_item(
    product_service: web::Data<ProductService>,
    path: web::Path<(String, String)>,
    stash_item_dto: web::Json<StashItemDTO>,
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

    let stash_item = match StashItem::try_from(stash_item_dto.into_inner()) {
        Ok(stash_item) => stash_item,
        Err(err) => return HttpResponse::BadRequest().body(format!("Invalid stash item: {}", err)),
    };

    if stash_item.id() != &stash_item_id {
        return HttpResponse::BadRequest().body("Stash item id does not match");
    }

    match product_service.update_stash_item(&product_id, stash_item) {
        Ok(stash_item) => HttpResponse::Ok().json(StashItemDTO::from(stash_item)),
        Err(ProductRepositoryError::StashItemNotFound) => {
            HttpResponse::NotFound().body("Stash item not found")
        }
        Err(ProductRepositoryError::ProductNotFound) => {
            HttpResponse::NotFound().body("Product not found")
        }
        Err(ProductRepositoryError::DuplicateExpiryDateError) => {
            HttpResponse::Conflict().body("Duplicate expiry date")
        }
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}
