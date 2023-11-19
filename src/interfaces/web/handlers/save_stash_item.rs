use actix_web::{web, HttpResponse};

use crate::{
    application::{services::ProductService, use_cases::SaveStashItem},
    domain::{entities::StashItem, value_objects::ProductId},
    interfaces::web::dtos::StashItemDTO,
};

pub async fn save_stash_item(
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
        Err(err) => return HttpResponse::BadRequest().body(format!("{}", err)),
    };

    match product_service.save_stash_item(&product_id, stash_item) {
        Ok(()) => HttpResponse::Ok().body("Stash item saved"),
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
