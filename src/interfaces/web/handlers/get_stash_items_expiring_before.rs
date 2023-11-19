use actix_web::{web, HttpResponse};

use crate::application::{services::StashItemService, usecases::GetStashItemsExpiringBefore};

pub async fn get_stash_items_expiring_before(
    stash_item_service: web::Data<StashItemService>,
    path: web::Path<String>,
) -> HttpResponse {
    let date = match path.parse::<chrono::NaiveDate>() {
        Ok(date) => date,
        Err(err) => return HttpResponse::BadRequest().body(format!("{}", err)),
    };

    match stash_item_service.get_stash_items_expiring_before(&date) {
        Ok(stash_items) => HttpResponse::Ok().json(
            stash_items
                .into_iter()
                .map(|stash_item| stash_item.into())
                .collect::<Vec<crate::interfaces::web::dtos::StashItemDTO>>(),
        ),
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
