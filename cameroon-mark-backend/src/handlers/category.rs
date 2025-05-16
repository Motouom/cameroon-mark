use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::{AppError, ApiResponse, Result};
use crate::services::category;
use crate::models::category::{CreateCategoryRequest, UpdateCategoryRequest};
use crate::middlewares::auth::ExtractUserId;

// AppState is defined in main.rs
use crate::AppState;

pub async fn get_categories(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    // Get all categories
    let categories = category::get_categories(&state.db).await?;
    
    // Return success response with categories
    Ok(Json(ApiResponse::success(categories)))
}

pub async fn get_category(
    State(state): State<Arc<AppState>>,
    Path(category_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let category = category::get_category(&state.db, category_id).await?;
    Ok(Json(ApiResponse::success(category)))
}

pub async fn create_category(
    State(state): State<Arc<AppState>>,
    _user_id: ExtractUserId, // Ensure user is authenticated
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<impl IntoResponse> {
    let category = category::create_category(&state.db, payload).await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::success(category))))
}

pub async fn update_category(
    State(state): State<Arc<AppState>>,
    _user_id: ExtractUserId, // Ensure user is authenticated
    Path(category_id): Path<Uuid>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<impl IntoResponse> {
    let category = category::update_category(&state.db, category_id, payload).await?;
    Ok(Json(ApiResponse::success(category)))
}

pub async fn delete_category(
    State(state): State<Arc<AppState>>,
    _user_id: ExtractUserId, // Ensure user is authenticated
    Path(category_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    category::delete_category(&state.db, category_id).await?;
    Ok(Json(ApiResponse::success("Category deleted successfully")))
}
