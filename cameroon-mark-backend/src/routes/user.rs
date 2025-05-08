use axum::{
    routing::{get, put},
    Router,
};
use std::sync::Arc;

use crate::handlers::user;
use crate::middlewares::auth::ExtractUserId;
use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/profile", get(user::get_profile))
        .route("/profile", put(user::update_profile))
        .route("/me/password", put(user::change_password))
        .route_layer(axum::middleware::from_extractor::<ExtractUserId>())
}
