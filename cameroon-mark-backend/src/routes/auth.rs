use axum::{
    routing::post,
    Router,
};
use std::sync::Arc;

use crate::handlers::auth;
use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/reset-password", post(auth::reset_password))
}
