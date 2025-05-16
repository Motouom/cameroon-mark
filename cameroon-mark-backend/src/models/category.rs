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

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, max = 100, message = "Category name must be between 1 and 100 characters"))]
    pub name: String,
    #[validate(length(max = 500, message = "Description must be less than 500 characters"))]
    pub description: Option<String>,
    pub image_url: Option<String>,
    #[validate(length(min = 1, max = 100, message = "Slug must be between 1 and 100 characters"))]
    pub slug: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCategoryRequest {
    #[validate(length(min = 1, max = 100, message = "Category name must be between 1 and 100 characters"))]
    pub name: Option<String>,
    #[validate(length(max = 500, message = "Description must be less than 500 characters"))]
    pub description: Option<String>,
    pub image_url: Option<String>,
    #[validate(length(min = 1, max = 100, message = "Slug must be between 1 and 100 characters"))]
    pub slug: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub slug: String,
    pub created_at: String,
    pub updated_at: String,
}
