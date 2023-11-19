use actix_web::web;

use super::handlers::{delete_product_by_id, get_product_by_id, save_product, save_stash_item};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::post().to(save_product))
            .route("/{product_id}", web::get().to(get_product_by_id))
            .route("/{product_id}", web::delete().to(delete_product_by_id))
            .service(
                web::scope("/{product_id}/stash-items")
                    .route("/add", web::post().to(save_stash_item)),
            ),
    );
}
