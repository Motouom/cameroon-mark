use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::{AppError, ApiResponse, Result};
use crate::middlewares::auth::{ExtractUserId, ExtractUserRole};
use crate::models::order::{CreateOrderRequest, UpdateOrderStatusRequest};
use crate::models::user::UserRole;
use crate::services::order;
use crate::utils::validation;

// AppState is defined in main.rs
use crate::AppState;

pub async fn create_order(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<impl IntoResponse> {
    // Validate request payload
    validation::validate(&payload)?;
    
    // Create order from cart items
    let order = order::create_order(&state.db, user_id.0, payload).await?;
    
    // Return success response with created order
    Ok((StatusCode::CREATED, Json(ApiResponse::success_with_message(
        order,
        "Order created successfully",
    ))))
}

pub async fn get_orders(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
    user_role: ExtractUserRole,
) -> Result<impl IntoResponse> {
    // Get orders based on user role
    let orders = match user_role.0 {
        // For customers, get their orders
        UserRole::Customer => {
            order::get_buyer_orders(&state.db, user_id.0).await?
        },
        // For sellers, get orders for their products
        UserRole::Seller => {
            order::get_seller_orders(&state.db, user_id.0).await?
        },
        // Admin can see all orders
        UserRole::Admin => {
            // Admins can view all orders in the system
            order::get_all_orders(&state.db).await?
        },
        // Pending sellers don't have access to orders yet
        UserRole::PendingSeller => {
            return Err(AppError::forbidden("Pending sellers cannot view orders"));
        },
    };
    
    // Return success response with orders
    Ok(Json(ApiResponse::success(orders)))
}

pub async fn get_order(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
    user_role: ExtractUserRole,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    // Get order by ID
    let order = order::get_order_by_id(&state.db, id).await?;
    
    // Check if user is authorized to view this order
    match user_role.0 {
        UserRole::Customer => {
            // Customers can only view their own orders
            if order.buyer_id != user_id.0 {
                return Err(AppError::forbidden("You are not authorized to view this order"));
            }
        },
        UserRole::Seller => {
            // Sellers can only view orders that contain their products
            let is_seller_order = order.items.iter().any(|item| item.seller_id == user_id.0);
            if !is_seller_order {
                return Err(AppError::forbidden("You are not authorized to view this order"));
            }
        },
        UserRole::Admin => {
            // Admins can view any order
            // No restrictions needed
        },
        UserRole::PendingSeller => {
            // Pending sellers don't have access to orders
            return Err(AppError::forbidden("Pending sellers cannot view orders"));
        },
    }
    
    // Return success response with order
    Ok(Json(ApiResponse::success(order)))
}

pub async fn update_order_status(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateOrderStatusRequest>,
) -> Result<impl IntoResponse> {
    // Get order by ID
    let order = order::get_order_by_id(&state.db, id).await?;
    
    // Check if user is authorized to update this order
    let is_seller_order = order.items.iter().any(|item| item.seller_id == user_id.0);
    if !is_seller_order {
        return Err(AppError::forbidden("You are not authorized to update this order"));
    }
    
    // Update order status
    let updated_order = order::update_order_status(&state.db, id, payload.status).await?;
    
    // Return success response with updated order
    Ok(Json(ApiResponse::success_with_message(
        updated_order,
        "Order status updated successfully",
    )))
}
