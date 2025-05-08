use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::handlers::category;
use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(category::get_categories))
}
