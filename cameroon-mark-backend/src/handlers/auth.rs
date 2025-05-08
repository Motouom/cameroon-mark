use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::errors::{AppError, ApiResponse, Result};
use crate::models::user::{LoginRequest, RegisterRequest, PasswordResetRequest, UserProfile};
use crate::services::auth;
use crate::utils::validation;
use crate::AppState;

#[derive(serde::Serialize)]
struct AuthResponse {
    user: UserProfile,
    token: String,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Response> {
    // Validate request payload
    validation::validate(&payload)?;
    
    // Register the user
    let (user, token) = auth::register(&state.db, payload).await?;
    
    Ok((
        StatusCode::CREATED,
        Json(AuthResponse { user: UserProfile::from(user), token }),
    )
        .into_response())
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Response> {
    // Validate request payload
    validation::validate(&payload)?;
    
    // Authenticate the user
    let (user, token) = auth::login(&state.db, payload).await?;
    
    Ok((
        StatusCode::OK,
        Json(AuthResponse { user: UserProfile::from(user), token }),
    )
        .into_response())
}

pub async fn reset_password(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PasswordResetRequest>,
) -> Result<impl IntoResponse> {
    // Validate request payload
    validation::validate(&payload)?;
    
    // Request password reset
    auth::request_password_reset(&state.db, &payload.email).await?;
    
    // Return success response
    Ok(Json(ApiResponse::success_with_message(
        serde_json::json!({}),
        "Password reset instructions sent to your email",
    )))
}
