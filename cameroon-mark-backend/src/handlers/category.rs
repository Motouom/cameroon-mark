use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::errors::{AppError, ApiResponse, Result};
use crate::services::category;

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
