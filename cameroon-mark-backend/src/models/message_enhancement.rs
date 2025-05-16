use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use chrono::{DateTime, Utc};

// Message template
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageTemplate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub subject: String,
    pub content: String,
    pub variables: Vec<String>, // Placeholders like {{name}}, {{product}}, etc.
    pub is_public: bool, // If true, can be used by other sellers
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Create message template request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateMessageTemplateRequest {
    #[validate(length(min = 1, max = 100, message = "Template name must be between 1 and 100 characters"))]
    pub name: String,
    
    #[validate(length(min = 1, max = 200, message = "Subject must be between 1 and 200 characters"))]
    pub subject: String,
    
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,
    
    pub is_public: bool,
}

// Automated response rule
#[derive(Debug, Serialize, Deserialize)]
pub struct AutoResponseRule {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub trigger_keywords: Vec<String>,
    pub template_id: Uuid,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Create auto-response rule request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateAutoResponseRuleRequest {
    #[validate(length(min = 1, max = 100, message = "Rule name must be between 1 and 100 characters"))]
    pub name: String,
    
    #[validate(length(min = 1, message = "At least one keyword is required"))]
    pub trigger_keywords: Vec<String>,
    
    pub template_id: Uuid,
    
    pub is_active: bool,
}

// Send message with template request
#[derive(Debug, Deserialize, Validate)]
pub struct SendTemplatedMessageRequest {
    pub recipient_id: Uuid,
    pub template_id: Uuid,
    pub variables: HashMap<String, String>, // Variable name to value mapping
}

// Bulk messaging request
#[derive(Debug, Deserialize, Validate)]
pub struct BulkMessagingRequest {
    pub template_id: Uuid,
    
    #[validate(length(min = 1, message = "At least one recipient is required"))]
    pub recipient_ids: Vec<Uuid>,
    
    pub variables: HashMap<String, String>, // Common variables for all messages
    
    #[validate(length(min = 1, message = "At least one recipient-specific variable set is required if individual_variables is provided"))]
    pub individual_variables: Option<HashMap<String, HashMap<String, String>>>, // Recipient ID to variables mapping
}

// Message statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageStatistics {
    pub total_messages_sent: i64,
    pub total_messages_received: i64,
    pub unread_messages: i64,
    pub average_response_time: Option<i64>, // In minutes
    pub common_keywords: Vec<(String, i64)>, // (keyword, count) pairs
    pub busiest_hour: Option<i32>, // 0-23 hour of day
    pub message_volume_by_day: HashMap<String, i64>, // Date string to count
}

use std::collections::HashMap;

// Message notification preferences
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageNotificationPreferences {
    pub user_id: Uuid,
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub notification_frequency: String, // "immediate", "hourly", "daily", "off"
    pub quiet_hours_start: Option<i32>, // 0-23
    pub quiet_hours_end: Option<i32>, // 0-23
    pub updated_at: DateTime<Utc>,
}

// Update notification preferences request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateNotificationPreferencesRequest {
    pub email_notifications: bool,
    pub push_notifications: bool,
    
    #[validate(custom = "validate_notification_frequency")]
    pub notification_frequency: String,
    
    pub quiet_hours_start: Option<i32>,
    pub quiet_hours_end: Option<i32>,
}

// Validator function for notification frequency
fn validate_notification_frequency(frequency: &str) -> Result<(), validator::ValidationError> {
    match frequency {
        "immediate" | "hourly" | "daily" | "off" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_frequency")),
    }
}

// Chatbot configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatbotConfig {
    pub id: Uuid,
    pub user_id: Uuid,
    pub is_enabled: bool,
    pub greeting_message: String,
    pub unavailable_message: String,
    pub common_responses: Vec<ChatbotResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Chatbot response
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ChatbotResponse {
    pub id: Option<Uuid>,
    
    #[validate(length(min = 1, message = "At least one trigger keyword is required"))]
    pub trigger_keywords: Vec<String>,
    
    #[validate(length(min = 1, max = 500, message = "Response message must be between 1 and 500 characters"))]
    pub response_message: String,
}

// Update chatbot config request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateChatbotConfigRequest {
    pub is_enabled: bool,
    
    #[validate(length(min = 1, max = 500, message = "Greeting message must be between 1 and 500 characters"))]
    pub greeting_message: String,
    
    #[validate(length(min = 1, max = 500, message = "Unavailable message must be between 1 and 500 characters"))]
    pub unavailable_message: String,
    
    #[validate]
    pub common_responses: Vec<ChatbotResponse>,
}
