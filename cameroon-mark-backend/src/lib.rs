pub mod config;
pub mod errors;
pub mod handlers;
pub mod middlewares;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;
pub mod entities;
// Re-export the config module for use with AppState
pub use crate::config::Config;

// Define a placeholder AppState struct for the handlers
pub mod app_state {
    use sea_orm::DatabaseConnection;
    use std::sync::Arc;
    use aws_sdk_s3::Client as S3Client;
    use crate::config::Config;

    // Application state that will be shared across handlers
    pub struct AppState {
        pub db: Arc<DatabaseConnection>,
        pub s3_client: Arc<S3Client>,
        pub config: Config,
    }
}

// Re-export the AppState struct
pub use app_state::AppState;

// Routes module is already exported above 