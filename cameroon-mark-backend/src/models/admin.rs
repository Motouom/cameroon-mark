use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;
use crate::models::user::UserRole;

// Admin dashboard statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_users: i64,
    pub total_sellers: i64,
    pub total_buyers: i64,
    pub total_products: i64,
    pub total_orders: i64,
    pub total_revenue: f64,
    pub pending_sellers: i64,
    pub reported_items: i64,
}

// Seller approval/rejection
#[derive(Debug, Serialize, Deserialize)]
pub struct PendingSeller {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SellerActionRequest {
    pub reason: Option<String>,
}

// Reported items
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedItem {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub seller_id: Uuid,
    pub seller_name: String,
    pub reporter_id: Uuid,
    pub reporter_name: String,
    pub reason: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ReportItemRequest {
    pub product_id: Uuid,
    #[validate(length(min = 5, max = 500, message = "Reason must be between 5 and 500 characters"))]
    pub reason: String,
}

// This struct is used for ignoring a report or providing a reason for deletion
#[derive(Debug, Deserialize, Validate)]
pub struct ReportActionRequest {
    pub reason: Option<String>,
}
