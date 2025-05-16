use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::handlers;
use crate::AppState;
use crate::middlewares::auth::ExtractUserId;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/seller", get(handlers::analytics::get_seller_analytics))
        .route_layer(axum::middleware::from_extractor::<ExtractUserId>())
} 