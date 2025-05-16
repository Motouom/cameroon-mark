use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// Enhanced order status with more detailed workflow steps
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OrderFulfillmentStatus {
    Pending,
    Confirmed,
    Processing,
    PackagingInProgress,
    ReadyForShipment,
    Shipped,
    OutForDelivery,
    Delivered,
    PartiallyDelivered,
    ReturnRequested,
    ReturnInProgress,
    ReturnReceived,
    Refunded,
    PartiallyRefunded,
    Cancelled,
    OnHold,
}

// Order batch model
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBatch {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub status: String, // "pending", "processing", "completed"
    pub order_ids: Vec<Uuid>,
    pub total_orders: i32,
    pub processed_orders: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Batch order processing request
#[derive(Debug, Deserialize, Validate)]
pub struct BatchOrderProcessingRequest {
    #[validate(length(min = 1, max = 100, message = "Batch name must be between 1 and 100 characters"))]
    pub name: String,
    
    #[validate(length(min = 1, message = "At least one order must be selected"))]
    pub order_ids: Vec<Uuid>,
    
    pub action: String, // "confirm", "process", "ship", etc.
    pub notes: Option<String>,
}

// Order fulfillment details
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderFulfillment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub status: OrderFulfillmentStatus,
    pub tracking_number: Option<String>,
    pub shipping_provider: Option<String>,
    pub shipping_label_url: Option<String>,
    pub package_weight: Option<f64>,
    pub package_dimensions: Option<String>,
    pub notes: Option<String>,
    pub handler_id: Option<Uuid>, // User handling the fulfillment
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Create or update order fulfillment request
#[derive(Debug, Deserialize, Validate)]
pub struct OrderFulfillmentRequest {
    pub status: OrderFulfillmentStatus,
    pub tracking_number: Option<String>,
    pub shipping_provider: Option<String>,
    pub package_weight: Option<f64>,
    pub package_dimensions: Option<String>,
    pub notes: Option<String>,
}

// Shipping label generation request
#[derive(Debug, Deserialize, Validate)]
pub struct ShippingLabelRequest {
    pub order_id: Uuid,
    pub shipping_provider: String,
    pub service_type: String, // e.g., "standard", "express"
    
    #[validate]
    pub recipient_address: ShippingAddress,
    
    #[validate]
    pub sender_address: ShippingAddress,
    
    pub package_weight: f64,
    pub package_dimensions: Option<String>,
    pub insurance_amount: Option<f64>,
    pub signature_required: bool,
}

// Shipping address for label generation
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ShippingAddress {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,
    
    #[validate(length(min = 1, max = 200, message = "Street address must be between 1 and 200 characters"))]
    pub street: String,
    
    #[validate(length(min = 1, max = 100, message = "City must be between 1 and 100 characters"))]
    pub city: String,
    
    #[validate(length(min = 1, max = 100, message = "State/Province must be between 1 and 100 characters"))]
    pub state_province: String,
    
    #[validate(length(min = 1, max = 20, message = "Postal code must be between 1 and 20 characters"))]
    pub postal_code: String,
    
    #[validate(length(min = 1, max = 100, message = "Country must be between 1 and 100 characters"))]
    pub country: String,
    
    pub phone: Option<String>,
}

// Order return/refund request
#[derive(Debug, Deserialize, Validate)]
pub struct OrderReturnRequest {
    pub order_id: Uuid,
    
    #[validate(length(min = 1, message = "At least one item must be selected for return"))]
    pub items: Vec<OrderReturnItem>,
    
    #[validate(length(min = 10, max = 500, message = "Return reason must be between 10 and 500 characters"))]
    pub reason: String,
    
    pub return_method: String, // "pickup", "dropoff", "ship"
    pub refund_method: String, // "original_payment", "store_credit", "bank_transfer"
    
    pub additional_notes: Option<String>,
}

// Order return item
#[derive(Debug, Deserialize, Validate)]
pub struct OrderReturnItem {
    pub order_item_id: Uuid,
    pub quantity: i32,
    pub condition: String, // "unopened", "opened", "damaged", "defective"
}

// Order return response
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderReturn {
    pub id: Uuid,
    pub order_id: Uuid,
    pub status: String, // "requested", "approved", "in_transit", "received", "completed", "rejected"
    pub return_items: Vec<OrderReturnItem>,
    pub reason: String,
    pub return_method: String,
    pub refund_method: String,
    pub return_tracking_number: Option<String>,
    pub refund_amount: Option<f64>,
    pub refund_processed_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
