use axum::{
    extract::{Path, State},
    routing::{get, delete, post},
    Router,
    Json,
};
use aws_sdk_s3::types::ObjectIdentifier;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::handlers::upload::{get_presigned_url, upload_image_handler};
use crate::middlewares::auth::ExtractUserId;
use crate::AppState;
use crate::errors::AppError;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/presigned-url", post(get_presigned_url))
        .route("/:image_name", delete(delete_image))
        .route_layer(axum::middleware::from_extractor::<ExtractUserId>())
}

async fn delete_image(
    State(state): State<Arc<AppState>>,
    Path(image_name): Path<String>,
) -> Result<Json<()>, AppError> {
    let bucket = &state.config.minio.bucket;
    
    // Create delete object request
    let delete_obj = ObjectIdentifier::builder()
        .set_key(Some(image_name.clone()))
        .build()
        .map_err(|e| AppError::internal(format!("Failed to build delete request: {}", e)))?;

    // Delete the object from MinIO
    state.s3_client
        .delete_object()
        .bucket(bucket)
        .key(&image_name)
        .send()
        .await
        .map_err(|e| AppError::internal(format!("Failed to delete image: {}", e)))?;

    Ok(Json(()))
}
