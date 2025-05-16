use axum::{
    extract::{Json, State, Multipart},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::errors::{AppError, ApiResponse, Result};
use crate::middlewares::auth::ExtractUserId;
use crate::services::upload;

// AppState is defined in main.rs
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct PresignedUrlRequest {
    pub file_name: String,
    pub content_type: String,
}

pub async fn get_presigned_url(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
    Json(payload): Json<PresignedUrlRequest>,
) -> Result<impl IntoResponse> {
    // Get the bucket name from config
    let bucket = &state.config.minio.bucket;
    
    // Generate a presigned URL for S3/MinIO upload
    let presigned_url = upload::generate_presigned_url(
        &state.s3_client,
        &bucket,
        user_id.0,
        &payload.file_name,
        &payload.content_type,
    ).await?;
    
    // Get the endpoint from config
    let endpoint = &state.config.minio.endpoint;
    
    // Return success response with presigned URL and file URL
    Ok(Json(ApiResponse::success(serde_json::json!({
        "presigned_url": presigned_url,
        "file_url": format!("{}/{}/{}", endpoint, bucket, payload.file_name)
    }))))
}

pub async fn upload_image_handler(
    State(state): State<Arc<AppState>>,
    multipart: Multipart
) -> Result<impl IntoResponse> {
    let uploaded_files = upload::upload_image(
        &state.s3_client,
        &state.config.minio.bucket,
        multipart
    ).await?;
    
    Ok(Json(ApiResponse::success_with_message(
        uploaded_files,
        "Files uploaded successfully",
    )))
}
