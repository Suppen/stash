use actix_web::{web, HttpResponse};
use chrono::NaiveDate;

use crate::{
    application::{services::ProductService, use_cases::GetProductsExpiringBefore},
    interfaces::web::v1::dtos::ProductDTO,
};

pub async fn get_products_expiring_before(
    product_service: web::Data<ProductService>,
    date: web::Path<String>,
) -> HttpResponse {
    let date = match NaiveDate::parse_from_str(date.into_inner().as_str(), "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            return HttpResponse::BadRequest()
                .body("Invalid date format. Date must be on form YYYY-MM-DD")
        }
    };

    match product_service.products_expiring_before(date) {
        Ok(products) => HttpResponse::Ok().json(
            products
                .into_iter()
                .map(ProductDTO::from)
                .collect::<Vec<_>>(),
        ),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get products"),
    }
}
