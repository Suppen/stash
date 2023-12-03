use actix_web::web;

use super::handlers::{
    add_stash_item, create_product, delete_product, delete_stash_item, get_product,
    get_product_by_stash_item_id, get_products_expiring_before, get_stash_items, update_product,
    update_stash_item,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/products")
            .route("", web::post().to(create_product))
            .route(
                "/by_stash_item_id/{stash_item_id}",
                web::get().to(get_product_by_stash_item_id),
            )
            .route(
                "/expiring_before/{date}",
                web::get().to(get_products_expiring_before),
            )
            .route("/{product_id}", web::get().to(get_product))
            .route("/{product_id}", web::put().to(update_product))
            .route("/{product_id}", web::delete().to(delete_product))
            .service(
                web::scope("/{product_id}/stash_items")
                    .route("", web::post().to(add_stash_item))
                    .route("", web::get().to(get_stash_items))
                    .route("/{stash_item_id}", web::put().to(update_stash_item))
                    .route("/{stash_item_id}", web::delete().to(delete_stash_item)),
            ),
    );
}
