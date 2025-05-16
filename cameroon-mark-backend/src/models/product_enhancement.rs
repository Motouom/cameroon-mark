use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// Bulk product upload request model
#[derive(Debug, Deserialize, Validate)]
pub struct BulkProductUploadRequest {
    #[validate]
    pub products: Vec<ProductUploadItem>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ProductUploadItem {
    #[validate(length(min = 3, max = 100, message = "Product name must be between 3 and 100 characters"))]
    pub name: String,

    #[validate(length(min = 10, message = "Description must be at least 10 characters"))]
    pub description: String,

    pub price: f64,

    pub category_id: Uuid,

    #[validate(url(message = "Main image must be a valid URL"))]
    pub main_image: String,

    #[validate(url(message = "Each additional image must be a valid URL"))]
    pub additional_images: Vec<String>,

    pub quantity_available: i32,

    #[validate]
    pub variants: Option<Vec<ProductVariant>>,
}

// Product Variants model
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ProductVariant {
    pub id: Option<Uuid>,
    
    #[validate(length(min = 1, max = 100, message = "Variant name must be between 1 and 100 characters"))]
    pub name: String,
    
    pub product_id: Option<Uuid>,
    
    #[validate]
    pub options: Vec<VariantOption>,
    
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VariantOption {
    pub id: Option<Uuid>,
    
    #[validate(length(min = 1, max = 100, message = "Option value must be between 1 and 100 characters"))]
    pub value: String,
    
    pub variant_id: Option<Uuid>,
    
    pub price_adjustment: f64,
    pub quantity: i32,
    pub sku: Option<String>,
    
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Inventory Model
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductInventory {
    pub product_id: Uuid,
    pub total_quantity: i32,
    pub available_quantity: i32,
    pub reserved_quantity: i32,
    pub low_stock_threshold: i32,
    pub variant_inventory: Option<HashMap<String, i32>>,
    pub updated_at: DateTime<Utc>,
}

// Product Import/Export Status
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportExportStatus {
    pub id: Uuid,
    pub user_id: Uuid,
    pub operation_type: String,  // "import" or "export"
    pub status: String,  // "pending", "processing", "completed", "failed"
    pub total_items: i32,
    pub processed_items: i32,
    pub successful_items: i32,
    pub failed_items: i32,
    pub file_url: Option<String>,
    pub error_log: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Inventory Log
#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryLog {
    pub id: Uuid,
    pub product_id: Uuid,
    pub variant_option_id: Option<Uuid>,
    pub previous_quantity: i32,
    pub new_quantity: i32,
    pub change_type: String,  // "add", "remove", "adjust", "reserve"
    pub reason: String,
    pub reference_id: Option<Uuid>,  // e.g., order ID
    pub created_at: DateTime<Utc>,
    pub user_id: Option<Uuid>,  // Who made the change
}

// Bulk Update Inventory Request
#[derive(Debug, Deserialize, Validate)]
pub struct BulkInventoryUpdateRequest {
    #[validate]
    pub updates: Vec<InventoryUpdateItem>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct InventoryUpdateItem {
    pub product_id: Uuid,
    pub variant_option_id: Option<Uuid>,
    pub quantity: i32,
    pub action: String,  // "set", "add", "subtract"
    pub reason: String,
}
