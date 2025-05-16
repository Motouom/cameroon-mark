use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;

use crate::handlers::category;
use crate::AppState;
use crate::middlewares::auth::ExtractUserId;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(category::get_categories))
        .route("/:id", get(category::get_category))
        // Protected routes that require authentication
        .route("/", post(category::create_category))
        .route("/:id", put(category::update_category))
        .route("/:id", delete(category::delete_category))
        // Add authentication middleware only to protected routes
        .route_layer(axum::middleware::from_extractor::<ExtractUserId>())
}
