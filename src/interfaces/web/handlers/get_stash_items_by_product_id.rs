use actix_web::{web, HttpResponse};

use crate::{
    application::{services::StashItemService, usecases::GetStashItemsByProductId},
    domain::value_objects::ProductId,
    interfaces::web::dtos::StashItemDTO,
};

pub async fn get_stash_items_by_product_id(
    stash_item_service: web::Data<StashItemService>,
    path: web::Path<String>,
) -> HttpResponse {
    let product_id = match path.parse::<ProductId>() {
        Ok(product_id) => product_id,
        Err(err) => return HttpResponse::BadRequest().body(format!("{}", err)),
    };

    match stash_item_service.get_stash_items_by_product_id(&product_id) {
        Ok(stash_items) => HttpResponse::Ok().json(
            stash_items
                .into_iter()
                .map(|stash_item| stash_item.into())
                .collect::<Vec<StashItemDTO>>(),
        ),
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
