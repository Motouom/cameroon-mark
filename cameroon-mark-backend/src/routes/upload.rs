use axum::{
    routing::post,
    Router,
};
use std::sync::Arc;

use crate::handlers::upload::{get_presigned_url, upload_image_handler};
use crate::middlewares::auth::ExtractUserId;
use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/presigned-url", post(get_presigned_url))
        .route("/image", post(upload_image_handler))
        .route_layer(axum::middleware::from_extractor::<ExtractUserId>())
}
