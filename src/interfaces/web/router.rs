use actix_web::web;

use super::handlers::{delete_product_by_id, get_product_by_id, save_product};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/product/{id}", web::get().to(get_product_by_id));
    cfg.route("/product", web::post().to(save_product));
    cfg.route("/product/{id}", web::delete().to(delete_product_by_id));
}
