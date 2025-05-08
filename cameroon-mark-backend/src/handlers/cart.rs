use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::{AppError, ApiResponse, Result};
use crate::middlewares::auth::ExtractUserId;
use crate::models::cart::{AddToCartRequest, UpdateCartItemRequest};
use crate::services::cart;
use crate::utils::validation;

// AppState is defined in main.rs
use crate::AppState;

pub async fn get_cart(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
) -> Result<impl IntoResponse> {
    // Get user's cart
    let cart_response = cart::get_cart(&state.db, user_id.0).await?;
    
    // Return success response with cart
    Ok(Json(ApiResponse::success(cart_response)))
}

pub async fn add_to_cart(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
    Json(payload): Json<AddToCartRequest>,
) -> Result<impl IntoResponse> {
    // Validate request payload
    validation::validate(&payload)?;
    
    // Add item to cart
    cart::add_to_cart(&state.db, user_id.0, payload).await?;
    
    // Get updated cart
    let cart_response = cart::get_cart(&state.db, user_id.0).await?;
    
    // Return success response with updated cart
    Ok(Json(ApiResponse::success_with_message(
        cart_response,
        "Item added to cart",
    )))
}

pub async fn update_cart_item(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
    Path(product_id): Path<Uuid>,
    Json(payload): Json<UpdateCartItemRequest>,
) -> Result<impl IntoResponse> {
    // Validate request payload
    validation::validate(&payload)?;
    
    // Update cart item
    cart::update_cart_item(&state.db, user_id.0, product_id, payload.quantity).await?;
    
    // Get updated cart
    let cart_response = cart::get_cart(&state.db, user_id.0).await?;
    
    // Return success response with updated cart
    Ok(Json(ApiResponse::success_with_message(
        cart_response,
        "Cart item updated",
    )))
}

pub async fn remove_from_cart(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
    Path(product_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    // Remove item from cart
    cart::remove_from_cart(&state.db, user_id.0, product_id).await?;
    
    // Get updated cart
    let cart_response = cart::get_cart(&state.db, user_id.0).await?;
    
    // Return success response with updated cart
    Ok(Json(ApiResponse::success_with_message(
        cart_response,
        "Item removed from cart",
    )))
}

pub async fn clear_cart(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
) -> Result<impl IntoResponse> {
    // Clear cart
    cart::clear_cart(&state.db, user_id.0).await?;
    
    // Return success response
    Ok(Json(ApiResponse::success_with_message(
        serde_json::json!({ "items": [], "total_items": 0, "total_price": 0 }),
        "Cart cleared",
    )))
}
