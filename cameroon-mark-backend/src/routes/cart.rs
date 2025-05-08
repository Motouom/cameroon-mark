use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;

use crate::handlers::cart;
use crate::middlewares::auth::ExtractUserId;
use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(cart::get_cart))
        .route("/items", post(cart::add_to_cart))
        .route("/items/:id", put(cart::update_cart_item))
        .route("/items/:id", delete(cart::remove_from_cart))
        .route("/", delete(cart::clear_cart))
        .route_layer(axum::middleware::from_extractor::<ExtractUserId>())
}
