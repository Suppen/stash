use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::{
    application::{services::StashItemService, usecases::GetStashItemById},
    interfaces::web::dtos::StashItemDTO,
};

pub async fn get_stash_item_by_id(
    stash_item_service: web::Data<StashItemService>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = match path.parse::<Uuid>() {
        Ok(id) => id,
        Err(err) => return HttpResponse::BadRequest().body(format!("Invalid ID: {}", err)),
    };

    match stash_item_service.get_stash_item_by_id(&id) {
        Ok(None) => HttpResponse::NotFound().body("Stash item does not exist"),
        Ok(Some(stash_item)) => HttpResponse::Ok().json(StashItemDTO::from(stash_item)),
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
