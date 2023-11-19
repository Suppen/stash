use actix_web::web;

use super::handlers::{
    delete_product_by_id, delete_stash_item_by_id, get_product_by_id, get_stash_item_by_id,
    get_stash_item_by_product_id_and_expiry_date, get_stash_items_by_product_id,
    get_stash_items_expiring_before, save_product, save_stash_item,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::post().to(save_product))
            .route("/{product_id}", web::get().to(get_product_by_id))
            .route("/{product_id}", web::delete().to(delete_product_by_id)),
    );

    cfg.service(
        web::scope("/stash_items")
            .route("", web::post().to(save_stash_item))
            .route("/{stash_item_id}", web::get().to(get_stash_item_by_id))
            .route(
                "/{stash_item_id}",
                web::delete().to(delete_stash_item_by_id),
            )
            .route(
                "/product/{product_id}/expiry_date/{expiry_date}",
                web::get().to(get_stash_item_by_product_id_and_expiry_date),
            )
            .route(
                "/product/{product_id}",
                web::get().to(get_stash_items_by_product_id),
            )
            .route(
                "/expiring_before/{date}",
                web::get().to(get_stash_items_expiring_before),
            ),
    );
}
