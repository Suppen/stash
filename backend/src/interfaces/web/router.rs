use actix_web::web;

use super::handlers::{
    create_product, delete_product_by_id, get_product_by_id, update_product_by_id,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::post().to(create_product))
            .route("/{product_id}", web::get().to(get_product_by_id))
            .route("/{product_id}", web::put().to(update_product_by_id))
            .route("/{product_id}", web::delete().to(delete_product_by_id))
            .service(web::scope("/{product_id}/stash-items")),
    );
}