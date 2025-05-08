use axum::{
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;

use crate::handlers::order;
use crate::middlewares::auth::{ExtractUserId, ExtractUserRole};
use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    let seller_routes = Router::new()
        .route("/:id/status", put(order::update_order_status))
        .route_layer(axum::middleware::from_extractor::<ExtractUserRole>());
        
    Router::new()
        .route("/", post(order::create_order))
        .route("/", get(order::get_orders))
        .route("/:id", get(order::get_order))
        .merge(seller_routes)
        .route_layer(axum::middleware::from_extractor::<ExtractUserId>())
}
