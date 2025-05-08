use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::{AppError, ApiResponse, Result};
use crate::middlewares::auth::ExtractUserId;
use crate::models::message::CreateMessageRequest;
use crate::services::message;
use crate::utils::validation;

// AppState is defined in main.rs
use crate::AppState;

pub async fn get_messages(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
) -> Result<impl IntoResponse> {
    // Get all messages for the current user
    let messages = message::get_user_messages(&state.db, user_id.0).await?;
    
    // Return success response with messages
    Ok(Json(ApiResponse::success(messages)))
}

pub async fn get_message_thread(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
    Path(thread_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    // Get message thread
    let messages = message::get_message_thread(&state.db, user_id.0, thread_id).await?;
    
    // Return success response with message thread
    Ok(Json(ApiResponse::success(messages)))
}

pub async fn send_message(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
    Json(payload): Json<CreateMessageRequest>,
) -> Result<impl IntoResponse> {
    // Validate request payload
    validation::validate(&payload)?;
    
    // Send message
    let new_message = message::send_message(&state.db, user_id.0, payload).await?;
    
    // Return success response with sent message
    Ok((StatusCode::CREATED, Json(ApiResponse::success_with_message(
        new_message,
        "Message sent successfully",
    ))))
}

pub async fn mark_as_read(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    // Mark message as read
    message::mark_message_as_read(&state.db, user_id.0, id).await?;
    
    // Return success response
    Ok(Json(ApiResponse::success_with_message(
        serde_json::json!({}),
        "Message marked as read",
    )))
}
