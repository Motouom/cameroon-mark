pub mod auth;
pub mod user;
pub mod product;
pub mod category;
pub mod upload;
pub mod cart;
pub mod order;
pub mod message;
pub mod saved_item;
pub mod analytics;
pub mod admin;
pub mod marketing;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;

use crate::handlers;
use crate::AppState;

pub fn configure_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/api", api_routes())
}

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        // Auth routes
        .nest("/auth", auth_routes())
        // Product routes
        .nest("/products", product_routes())
        // Order routes
        .nest("/orders", order_routes())
        // Message routes
        .nest("/messages", message_routes())
        // Analytics routes
        .nest("/analytics", analytics::routes())
        // Saved items routes
        .nest("/saved-items", saved_item::routes())
        // User routes (profile, address, etc.)
        .nest("/users", user::routes())
        // Admin routes
        .nest("/admin", admin::routes())
}

fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login))
        .route("/reset-password", post(handlers::auth::reset_password))
}

fn product_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(handlers::product::get_products))
        .route("/", post(handlers::product::create_product))
        .route("/:id", get(handlers::product::get_product))
        .route("/:id", put(handlers::product::update_product))
        .route("/:id", delete(handlers::product::delete_product))
}

fn order_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(handlers::order::get_orders))
        .route("/", post(handlers::order::create_order))
        .route("/:id", get(handlers::order::get_order))
        .route("/:id/status", put(handlers::order::update_order_status))
}

fn message_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(handlers::message::get_messages))
        .route("/", post(handlers::message::send_message))
        .route("/:id/thread", get(handlers::message::get_message_thread))
        .route("/:id/read", put(handlers::message::mark_as_read))
}
