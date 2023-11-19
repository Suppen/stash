use actix_web::{web, HttpResponse};

use crate::{
    application::{services::ProductService, use_cases::DeleteProduct},
    domain::value_objects::ProductId,
};

pub async fn delete_product(
    product_service: web::Data<ProductService>,
    path: web::Path<String>,
) -> HttpResponse {
    let product_id = match path.parse::<ProductId>() {
        Ok(product_id) => product_id,
        Err(err) => return HttpResponse::BadRequest().body(format!("Invalid product id: {}", err)),
    };

    match product_service.delete_product(&product_id) {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
