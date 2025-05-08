use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;

use crate::errors::{AppError, ApiResponse, Result};
use crate::handlers::message;
use crate::middlewares::auth::ExtractUserId;
use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(message::get_messages))
        .route("/:thread_id", get(message::get_message_thread))
        .route("/", post(message::send_message))
        .route("/:id/read", put(message::mark_as_read))
        .route_layer(axum::middleware::from_extractor::<ExtractUserId>())
}
