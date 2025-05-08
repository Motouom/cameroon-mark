use aws_sdk_s3::{Client as S3Client, presigning::PresigningConfig};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use crate::errors::{AppError, Result};

// Generate a presigned URL for S3/MinIO upload
pub async fn generate_presigned_url(
    s3_client: &Arc<S3Client>,
    bucket: &str,
    user_id: Uuid,
    file_name: &str,
    content_type: &str,
) -> Result<String> {
    // Create a unique file name using the user ID and original file name
    let unique_file_name = format!("{}/{}", user_id, file_name);
    
    // Create a presigned PUT request
    let presigned_req = s3_client
        .put_object()
        .bucket(bucket)
        .key(&unique_file_name)
        .content_type(content_type)
        .presigned(PresigningConfig::expires_in(Duration::from_secs(3600)).unwrap())
        .await
        .map_err(|e| AppError::external_service(format!("Failed to generate presigned URL: {}", e)))?;
    
    // Return the presigned URL
    Ok(presigned_req.uri().to_string())
}

use axum::{
    body::Bytes,
    extract::Multipart,
    http::StatusCode,
};
use std::io;
use tokio::fs;

const MAX_FILE_SIZE: usize = 5 * 1024 * 1024; // 5MB
const ALLOWED_EXTENSIONS: [&str; 4] = ["jpg", "jpeg", "png", "gif"];

pub async fn upload_image(mut multipart: Multipart) -> Result<Vec<String>> {
    let mut uploaded_files = Vec::new();
    let mut has_files = false;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::bad_request(format!("Failed to process multipart form: {}", e))
    })? {
        has_files = true;
        
        let file_name = field.file_name()
            .ok_or_else(|| AppError::bad_request("No file name provided"))?
            .to_string();
            
        let extension = file_name.split('.')
            .last()
            .ok_or_else(|| AppError::bad_request("Invalid file extension"))?
            .to_lowercase();
            
        if !ALLOWED_EXTENSIONS.contains(&extension.as_str()) {
            return Err(AppError::bad_request(format!(
                "Invalid file type. Allowed types: {}",
                ALLOWED_EXTENSIONS.join(", ")
            )));
        }
        
        let data = field.bytes().await.map_err(|e| {
            AppError::bad_request(format!("Failed to read file data: {}", e))
        })?;
        
        if data.len() > MAX_FILE_SIZE {
            return Err(AppError::bad_request(format!(
                "File too large. Maximum size is {}MB",
                MAX_FILE_SIZE / 1024 / 1024
            )));
        }
        
        // Generate unique filename
        let unique_filename = format!("{}.{}", Uuid::new_v4(), extension);
        let file_path = format!("uploads/{}", unique_filename);
        
        // Create uploads directory if it doesn't exist
        fs::create_dir_all("uploads").await.map_err(|e| {
            AppError::internal(format!("Failed to create uploads directory: {}", e))
        })?;
        
        // Save file
        fs::write(&file_path, data).await.map_err(|e| {
            AppError::internal(format!("Failed to save file: {}", e))
        })?;
        
        uploaded_files.push(unique_filename);
    }
    
    if !has_files {
        return Err(AppError::bad_request("No files uploaded"));
    }
    
    Ok(uploaded_files)
}
