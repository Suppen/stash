use actix_web::{web, HttpResponse};

use crate::{
    application::{services::ProductService, use_cases::AddStashItem},
    domain::{entities::StashItem, errors::ProductRepositoryError, value_objects::ProductId},
    interfaces::web::v1::dtos::StashItemDTO,
};

pub async fn add_stash_item(
    product_service: web::Data<ProductService>,
    path: web::Path<String>,
    stash_item_dto: web::Json<StashItemDTO>,
) -> HttpResponse {
    let product_id = match path.parse::<ProductId>() {
        Ok(product_id) => product_id,
        Err(err) => return HttpResponse::BadRequest().body(format!("Invalid product id: {}", err)),
    };

    let stash_item = match StashItem::try_from(stash_item_dto.into_inner()) {
        Ok(stash_item) => stash_item,
        Err(err) => return HttpResponse::BadRequest().body(format!("Invalid stash item: {}", err)),
    };

    match product_service.add_stash_item(&product_id, stash_item) {
        // TODO Return 201 Created and the stash item
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(ProductRepositoryError::StashItemExists) => {
            HttpResponse::Conflict().body("Stash item already exists")
        }
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}
