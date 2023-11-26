use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::{
    application::{services::ProductService, use_cases::GetProductByStashItemId},
    interfaces::web::v1::dtos::ProductDTO,
};

pub async fn get_product_by_stash_item_id(
    product_service: web::Data<ProductService>,
    stash_item_id: web::Path<String>,
) -> HttpResponse {
    let stash_item_id = match Uuid::parse_str(&stash_item_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let product = product_service.get_product_by_stash_item_id(&stash_item_id);

    match product {
        Ok(None) => HttpResponse::NotFound().finish(),
        Ok(Some(product)) => HttpResponse::Ok().json(ProductDTO::from(product)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
