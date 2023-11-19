use actix_web::web;

use super::handlers::{
    delete_product_by_id, delete_stash_item_by_id, get_product_by_id, get_stash_item_by_id,
    get_stash_item_by_product_id_and_expiry_date, get_stash_items_by_product_id,
    get_stash_items_expiring_before, save_product, save_stash_item,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/product").route(web::post().to(save_product)))
        .service(
            web::resource("/product/{product_id}")
                .route(web::get().to(get_product_by_id))
                .route(web::delete().to(delete_product_by_id)),
        )
        .service(web::resource("/stash_item").route(web::post().to(save_stash_item)))
        .service(
            web::resource("/stash_item/{stash_item_id}")
                .route(web::get().to(get_stash_item_by_id))
                .route(web::delete().to(delete_stash_item_by_id)),
        )
        .service(
            web::resource("/stash_item/product/{product_id}/expiry_date/{expiry_date}")
                .route(web::get().to(get_stash_item_by_product_id_and_expiry_date)),
        )
        .service(
            web::resource("/stash_items/product/{product_id}")
                .route(web::get().to(get_stash_items_by_product_id)),
        )
        .service(
            web::resource("/stash_items/expiring_before/{date}")
                .route(web::get().to(get_stash_items_expiring_before)),
        );
}
