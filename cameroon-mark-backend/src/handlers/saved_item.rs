use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::{AppError, ApiResponse, Result};
use crate::middlewares::auth::ExtractUserId;
use crate::models::saved_item::AddSavedItemRequest;
use crate::services::saved_item;
use crate::utils::validation;
use crate::AppState;

// Get all saved items for the current user
pub async fn get_saved_items(
    State(state): State<Arc<AppState>>,
    ExtractUserId(user_id): ExtractUserId,
) -> Result<impl IntoResponse> {
    let saved_items = saved_item::get_saved_items(&state.db, user_id).await?;
    Ok(Json(ApiResponse::success(saved_items)))
}

// Add a product to saved items
pub async fn add_saved_item(
    State(state): State<Arc<AppState>>,
    ExtractUserId(user_id): ExtractUserId,
    Json(payload): Json<AddSavedItemRequest>,
) -> Result<impl IntoResponse> {
    validation::validate(&payload)?;
    
    let saved_item = saved_item::add_saved_item(&state.db, user_id, payload).await?;
    Ok(Json(ApiResponse::success_with_message(
        saved_item,
        "Product added to saved items",
    )))
}

// Remove a product from saved items
pub async fn remove_saved_item(
    State(state): State<Arc<AppState>>,
    ExtractUserId(user_id): ExtractUserId,
    Path(product_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    saved_item::remove_saved_item(&state.db, user_id, product_id).await?;
    Ok(Json(ApiResponse::success_with_message((), "Product removed from saved items")))
}