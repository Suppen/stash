use actix_web::web;

use super::handlers::{delete_product_by_id, get_product_by_id, save_product, save_stash_item};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/product").route(web::post().to(save_product)))
        .service(
            web::resource("/product/{product_id}")
                .route(web::get().to(get_product_by_id))
                .route(web::delete().to(delete_product_by_id)),
        )
        .service(web::resource("/stash_item").route(web::post().to(save_stash_item)));
}
