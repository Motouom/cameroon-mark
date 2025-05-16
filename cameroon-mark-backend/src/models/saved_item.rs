use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::entities::saved_item;
use crate::models::product::ProductSummary;

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedItem {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedItemWithProduct {
    pub id: Uuid,
    pub product: ProductSummary,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddSavedItemRequest {
    pub product_id: Uuid,
}

impl From<saved_item::Model> for SavedItem {
    fn from(model: saved_item::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            product_id: model.product_id,
            created_at: model.created_at,
        }
    }
}