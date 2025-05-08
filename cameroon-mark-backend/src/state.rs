use sea_orm::DatabaseConnection;
use std::sync::Arc;
use aws_sdk_s3::Client as S3Client;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub s3_client: Arc<S3Client>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, s3_client: S3Client) -> Self {
        Self {
            db: Arc::new(db),
            s3_client: Arc::new(s3_client),
        }
    }
} 