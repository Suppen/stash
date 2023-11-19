use actix_web::{web, HttpResponse};

use crate::application::{services::StashItemService, usecases::DeleteStashItemById};

pub async fn delete_stash_item_by_id(
    stash_item_service: web::Data<StashItemService>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = match path.parse::<uuid::Uuid>() {
        Ok(id) => id,
        Err(err) => return HttpResponse::BadRequest().body(format!("Invalid ID: {}", err)),
    };

    match stash_item_service.delete_stash_item_by_id(&id) {
        Ok(()) => HttpResponse::Ok().body("Stash item deleted"),
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
