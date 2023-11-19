use actix_web::{web, HttpResponse};

use crate::{
    application::{services::StashItemService, usecases::SaveStashItem},
    domain::entities::StashItem,
    interfaces::web::dtos::StashItemDTO,
};

pub async fn save_stash_item(
    stash_item_service: web::Data<StashItemService>,
    stash_item_dto: web::Json<StashItemDTO>,
) -> HttpResponse {
    let stash_item = match StashItem::try_from(stash_item_dto.into_inner()) {
        Ok(stash_item) => stash_item,
        Err(err) => return HttpResponse::BadRequest().body(format!("{}", err)),
    };

    match stash_item_service.save_stash_item(stash_item) {
        Ok(()) => HttpResponse::Ok().body("Stash item saved"),
        Err(crate::domain::errors::StashItemRepositoryError::ProductDoesNotExist) => {
            HttpResponse::NotFound().body("Product does not exist")
        }
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
