use axum::{
    routing::{get, post, delete},
    Router,
};
use std::sync::Arc;

use crate::handlers::saved_item;
use crate::middlewares::auth::ExtractUserId;
use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(saved_item::get_saved_items))
        .route("/", post(saved_item::add_saved_item))
        .route("/:product_id", delete(saved_item::remove_saved_item))
        .route_layer(axum::middleware::from_extractor::<ExtractUserId>())
}