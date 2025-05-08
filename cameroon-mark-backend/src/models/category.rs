use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,
    
    pub description: Option<String>,
    pub image_url: Option<String>,
    
    #[validate(length(min = 1, max = 100, message = "Slug must be between 1 and 100 characters"))]
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateCategoryRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,
    
    pub description: Option<String>,
    pub image_url: Option<String>,
    
    #[validate(length(min = 1, max = 100, message = "Slug must be between 1 and 100 characters"))]
    pub slug: String,
}
