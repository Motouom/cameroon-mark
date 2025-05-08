use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::errors::{AppError, ApiResponse, Result};
use crate::middlewares::auth::ExtractUserId;
use crate::models::user::{UpdateProfileRequest, ChangePasswordRequest, UserProfile};
use crate::services::user;
use crate::utils::validation;

// AppState is defined in main.rs
use crate::AppState;

pub async fn get_profile(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
) -> Result<impl IntoResponse> {
    // Get user profile
    let user = user::get_user_by_id(&state.db, user_id.0).await?;
    
    // Return success response with user profile
    Ok(Json(ApiResponse::success(UserProfile::from(user))))
}

pub async fn update_profile(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse> {
    // Validate request payload
    validation::validate(&payload)?;
    
    // Update user profile
    let updated_user = user::update_user(&state.db, user_id.0, payload).await?;
    
    // Return success response with updated user profile
    Ok(Json(ApiResponse::success_with_message(
        UserProfile::from(updated_user),
        "Profile updated successfully",
    )))
}

pub async fn change_password(
    State(state): State<Arc<AppState>>,
    ExtractUserId(user_id): ExtractUserId,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<()>> {
    user::change_password(&state.db, user_id, payload).await?;
    Ok(Json(()))
}
