use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::handlers::admin::*;  // Using glob import to include all admin handlers
use crate::middlewares::auth::{ExtractUserId, RequireAdmin};
use crate::app_state::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    // Admin-only routes
    let admin_routes = Router::new()
        .route("/dashboard", get(get_dashboard_stats))
        .route("/sellers/pending", get(get_pending_sellers))
        .route("/sellers/:seller_id/approve", post(approve_seller))
        .route("/sellers/:seller_id/reject", post(reject_seller))
        .route("/reports", get(get_reported_items))
        .route("/reports/:report_id/delete", post(delete_reported_item))
        .route("/reports/:report_id/ignore", post(ignore_reported_item))
        .route_layer(axum::middleware::from_extractor::<RequireAdmin>());

    // User routes - for reporting items
    let user_routes = Router::new()
        .route("/reports", post(report_item))
        .route_layer(axum::middleware::from_extractor::<ExtractUserId>());

    // Combine routes
    admin_routes.merge(user_routes)
}
