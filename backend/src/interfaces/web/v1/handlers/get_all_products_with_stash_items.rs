use actix_web::{web, HttpResponse};

use crate::{
    application::{services::ProductService, use_cases::GetAllProductsWithStashItems},
    interfaces::web::v1::dtos::ProductDTO,
};

pub async fn get_all_products_with_stash_items(
    product_service: web::Data<ProductService>,
) -> HttpResponse {
    match product_service.get_all_products_with_stash_items() {
        Ok(products) => HttpResponse::Ok().json(
            products
                .into_iter()
                .map(ProductDTO::from)
                .collect::<Vec<_>>(),
        ),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
