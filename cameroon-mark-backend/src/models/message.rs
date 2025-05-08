use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub thread_id: Option<Uuid>,
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub subject: String,
    pub message: String,
    pub read: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateMessageRequest {
    pub recipient_id: Uuid,
    
    pub thread_id: Option<Uuid>,
    
    #[validate(length(min = 1, max = 100, message = "Subject must be between 1 and 100 characters"))]
    pub subject: String,
    
    #[validate(length(min = 1, message = "Message cannot be empty"))]
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub id: Uuid,
    pub thread_id: Option<Uuid>,
    pub sender_id: Uuid,
    pub sender_name: String,
    pub recipient_id: Uuid,
    pub recipient_name: String,
    pub subject: String,
    pub message: String,
    pub read: bool,
    pub created_at: DateTime<Utc>,
}
