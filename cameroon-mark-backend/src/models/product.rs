use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::{Validate, ValidationError};
use sqlx::types::BigDecimal;
use std::str::FromStr;
use sea_orm::{TryGetable, TryGetError};
use sea_orm::sea_query::ValueType;
use std::collections::HashSet;

use crate::models::user::UserRole;
use crate::entities::product;

fn validate_price(value: &BigDecimal) -> Result<(), ValidationError> {
    if value <= &BigDecimal::from_str("0").unwrap() {
        return Err(ValidationError::new("price_must_be_positive"));
    }
    Ok(())
}

fn validate_optional_price(value: &BigDecimal) -> Result<(), ValidationError> {
    if value <= &BigDecimal::from_str("0").unwrap() {
        return Err(ValidationError::new("price_must_be_positive"));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageArray(pub Vec<String>);

impl Validate for ImageArray {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.0.is_empty() {
            let mut errors = validator::ValidationErrors::new();
            errors.add("images", ValidationError::new("images_required"));
            return Err(errors);
        }
        Ok(())
    }
}

impl ValueType for ImageArray {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        match v {
            sea_orm::Value::Json(Some(json)) => {
                let array: Vec<String> = serde_json::from_value(*json).map_err(|_| {
                    sea_orm::sea_query::ValueTypeErr
                })?;
                Ok(ImageArray(array))
            }
            _ => Err(sea_orm::sea_query::ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "jsonb".to_string()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::Json
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        sea_orm::sea_query::ColumnType::Json
    }
}

impl From<ImageArray> for sea_orm::Value {
    fn from(images: ImageArray) -> Self {
        sea_orm::Value::Json(Some(serde_json::to_value(images.0).unwrap().into()))
    }
}

impl TryGetable for ImageArray {
    fn try_get_by<I: sea_orm::ColIdx>(res: &sea_orm::QueryResult, idx: I) -> Result<Self, sea_orm::TryGetError> {
        let json: serde_json::Value = res.try_get_by(idx)?;
        let array: Vec<String> = serde_json::from_value(json).map_err(|e| {
            sea_orm::TryGetError::DbErr(sea_orm::DbErr::Custom(format!("Failed to parse image array: {}", e)))
        })?;
        Ok(ImageArray(array))
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Product {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub price: BigDecimal,
    pub images: ImageArray,
    pub category_id: Uuid,
    pub seller_id: Uuid,
    pub stock: i32,
    pub location: String,
    pub featured: bool,
    pub rating: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<product::Model> for Product {
    fn from(model: product::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            description: model.description,
            price: model.price,
            images: model.images,
            category_id: model.category_id,
            seller_id: model.seller_id,
            stock: model.stock,
            location: model.location,
            featured: model.featured,
            rating: model.rating.map(|r| r as f64),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 3, max = 100))]
    pub title: String,
    
    #[validate(length(min = 10, max = 1000))]
    pub description: String,
    
    #[validate(custom = "validate_price")]
    pub price: BigDecimal,
    
    pub category_id: Uuid,
    
    #[validate(range(min = 0))]
    pub stock: i32,
    
    #[validate(custom = "validate_images")]
    pub images: ImageArray,
    
    pub location: Option<String>,
    
    pub featured: Option<bool>,
}

fn validate_images(images: &ImageArray) -> Result<(), ValidationError> {
    if images.0.is_empty() {
        return Err(ValidationError::new("images_required"));
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateProductRequest {
    #[validate(length(min = 3, max = 100))]
    pub title: Option<String>,
    
    #[validate(length(min = 10, max = 1000))]
    pub description: Option<String>,
    
    #[validate(custom = "validate_optional_price")]
    pub price: Option<BigDecimal>,
    
    pub category_id: Option<Uuid>,
    
    #[validate(range(min = 0))]
    pub stock: Option<i32>,
    
    pub images: Option<ImageArray>,
    
    pub location: Option<String>,
    
    pub featured: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub price: BigDecimal,
    pub category_id: Uuid,
    pub seller_id: Uuid,
    pub stock: i32,
    pub images: Vec<String>,
    pub location: String,
    pub featured: bool,
    pub rating: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id,
            title: product.title,
            description: product.description,
            price: product.price,
            category_id: product.category_id,
            seller_id: product.seller_id,
            stock: product.stock,
            images: product.images.0,
            location: product.location,
            featured: product.featured,
            rating: product.rating,
            created_at: product.created_at,
            updated_at: product.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProductFilterOptions {
    pub search: Option<String>,
    pub category_id: Option<Uuid>,
    pub seller_id: Option<Uuid>,
    pub location: Option<String>,
    pub featured: Option<bool>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

impl Default for ProductFilterOptions {
    fn default() -> Self {
        Self {
            search: None,
            category_id: None,
            seller_id: None,
            location: None,
            featured: None,
            page: None,
            per_page: None,
        }
    }
}
