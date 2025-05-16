use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Router,
};
use std::sync::Arc;

use crate::errors::{AppError, ApiResponse, Result};
use crate::middlewares::auth::ExtractUserId;
use crate::models::user::{UpdateProfileRequest, ChangePasswordRequest, UserProfile, UserAddressRequest, AddressDetails};
use crate::services::user;
use crate::utils::validation;

// AppState is defined in main.rs
use crate::AppState;

pub async fn get_profile(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
) -> Result<impl IntoResponse> {
    // Get user profile (now includes address details if available)
    let user_profile = user::get_profile(&state.db, user_id.0).await?;
    
    // Return success response with user profile
    Ok(Json(ApiResponse::success(user_profile)))
}

pub async fn update_profile(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse> {
    // Validate request payload
    validation::validate(&payload)?;
    
    // Update user profile (name, phone only)
    let updated_user = user::update_user(&state.db, user_id.0, payload).await?;
    
    // Return success response with updated user profile (main fields)
    // The UserProfile from User will contain the address, but this specific update
    // only modified name/phone. Client should re-fetch full profile or address if needed after this.
    Ok(Json(ApiResponse::success_with_message(
        UserProfile::from(updated_user),
        "Profile updated successfully",
    )))
}

// New handler to get user address
pub async fn get_user_address_handler(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
) -> Result<impl IntoResponse> {
    let user_profile = user::get_profile(&state.db, user_id.0).await?;
    match user_profile.address {
        Some(address_details) => Ok(Json(ApiResponse::success(address_details))),
        None => Ok(Json(ApiResponse::success(AddressDetails {
            street: None,
            city: None,
            postal_code: None,
            country: None,
        }))),
    }
}

// New handler to update user address
pub async fn update_user_address_handler(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
    Json(payload): Json<UserAddressRequest>,
) -> Result<impl IntoResponse> {
    validation::validate(&payload)?;
    let updated_address = user::update_user_address(&state.db, user_id.0, payload).await?;
    Ok(Json(ApiResponse::success_with_message(
        updated_address,
        "Address updated successfully",
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
