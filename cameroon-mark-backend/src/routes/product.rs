use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;

use crate::handlers::product;
use crate::middlewares::auth::{ExtractUserId, ExtractUserRole, require_seller};
use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    let seller_routes = Router::new()
        .route("/", post(product::create_product))
        .route("/:id", put(product::update_product))
        .route("/:id", delete(product::delete_product))
        .layer(axum::middleware::from_fn(require_seller));

    Router::new()
        .route("/", get(product::get_products))
        .route("/:id", get(product::get_product))
        .merge(seller_routes)
}
