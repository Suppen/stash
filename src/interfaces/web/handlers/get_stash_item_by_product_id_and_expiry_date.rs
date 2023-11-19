use actix_web::{web, HttpResponse};

use crate::{
    application::{services::StashItemService, usecases::GetStashItemByProductIdAndExpiryDate},
    domain::value_objects::ProductId,
    interfaces::web::dtos::StashItemDTO,
};

pub async fn get_stash_item_by_product_id_and_expiry_date(
    stash_item_service: web::Data<StashItemService>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let product_id = match path.0.parse::<ProductId>() {
        Ok(product_id) => product_id,
        Err(err) => return HttpResponse::BadRequest().body(format!("{}", err)),
    };

    let expiry_date = match path.1.parse::<chrono::NaiveDate>() {
        Ok(expiry_date) => expiry_date,
        Err(err) => return HttpResponse::BadRequest().body(format!("{}", err)),
    };

    match stash_item_service.get_stash_item_by_product_id_and_expiry_date(&product_id, &expiry_date)
    {
        Ok(None) => HttpResponse::NotFound().body("Stash item does not exist"),
        Ok(Some(stash_item)) => HttpResponse::Ok().json(StashItemDTO::from(stash_item)),
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
