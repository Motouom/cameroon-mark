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
        .route("/profile", get(user::get_profile).put(user::update_profile))
        .route("/me/password", put(user::change_password))
        .route("/me/address", get(user::get_user_address_handler).put(user::update_user_address_handler))
        .route_layer(axum::middleware::from_extractor::<ExtractUserId>())
}
