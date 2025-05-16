use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate, NaiveDateTime};
use uuid::Uuid;
use validator::Validate;
use std::collections::HashMap;

// Campaign types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CampaignType {
    Sale,           // General discount sale
    FlashSale,      // Short time-limited sale  
    ProductLaunch,  // New product introduction
    Seasonal,       // Holiday or season-specific
    Clearance,      // End of season or inventory clearance
    BundleDeal,     // Bundle products together
    Loyalty,        // Reward for loyal customers
    Custom,         // Custom campaign type
}

// Discount types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DiscountType {
    Percentage,     // % off
    FixedAmount,    // Fixed amount off
    BuyXGetY,       // Buy X get Y free/discounted
    FreeShipping,   // Free shipping
    Bundled,        // Bundle discount
}

// Promotional campaign
#[derive(Debug, Serialize, Deserialize)]
pub struct Campaign {
    pub id: Uuid,
    pub seller_id: Uuid,
    pub name: String,
    pub description: String,
    pub campaign_type: CampaignType,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub is_active: bool,
    pub target_audience: Option<TargetAudience>,
    pub discount_codes: Vec<Uuid>, // IDs of associated discount codes
    pub banner_image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Create campaign request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCampaignRequest {
    #[validate(length(min = 3, max = 100, message = "Campaign name must be between 3 and 100 characters"))]
    pub name: String,
    
    #[validate(length(min = 10, message = "Description must be at least 10 characters"))]
    pub description: String,
    
    pub campaign_type: CampaignType,
    
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    
    pub is_active: bool,
    
    pub target_audience: Option<TargetAudience>,
    
    pub create_discount_code: Option<CreateDiscountCodeRequest>,
    
    pub banner_image: Option<String>,
}

// Target audience for campaign
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TargetAudience {
    pub customer_type: Option<String>, // "new", "returning", "all"
    pub min_orders: Option<i32>,
    pub min_total_spent: Option<f64>,
    pub specific_user_ids: Option<Vec<Uuid>>,
}

// Discount code
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountCode {
    pub id: Uuid,
    pub seller_id: Uuid,
    pub campaign_id: Option<Uuid>,
    pub code: String,
    pub discount_type: DiscountType,
    pub discount_value: f64,
    pub min_purchase_amount: Option<f64>,
    pub max_discount_amount: Option<f64>,
    pub usage_limit: Option<i32>,
    pub usage_count: i32,
    pub products: Option<Vec<Uuid>>,
    pub categories: Option<Vec<Uuid>>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Create discount code request
#[derive(Debug, Deserialize, Validate, Clone)]
pub struct CreateDiscountCodeRequest {
    #[validate(length(min = 3, max = 20, message = "Discount code must be between 3 and 20 characters"))]
    pub code: String,
    
    pub discount_type: DiscountType,
    
    pub discount_value: f64,
    
    pub min_purchase_amount: Option<f64>,
    
    pub max_discount_amount: Option<f64>,
    
    pub usage_limit: Option<i32>,
    
    pub products: Option<Vec<Uuid>>,
    
    pub categories: Option<Vec<Uuid>>,
    
    pub start_date: DateTime<Utc>,
    
    pub end_date: DateTime<Utc>,
    
    pub is_active: bool,
}

// Email marketing campaign
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailCampaign {
    pub id: Uuid,
    pub seller_id: Uuid,
    pub name: String,
    pub subject: String,
    pub content: String,
    pub recipient_list: EmailRecipientList,
    pub scheduled_time: Option<DateTime<Utc>>,
    pub sent_time: Option<DateTime<Utc>>,
    pub status: String, // "draft", "scheduled", "sending", "sent", "failed"
    pub open_count: i32,
    pub click_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Email recipient list
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailRecipientList {
    pub all_customers: bool,
    pub specific_customers: Option<Vec<Uuid>>,
    pub customer_filters: Option<CustomerFilters>,
}

// Customer filters for email targeting
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerFilters {
    pub purchased_products: Option<Vec<Uuid>>,
    pub purchased_categories: Option<Vec<Uuid>>,
    pub min_orders: Option<i32>,
    pub last_order_after: Option<NaiveDate>,
    pub last_order_before: Option<NaiveDate>,
    pub min_spent: Option<f64>,
}

// Create email campaign request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateEmailCampaignRequest {
    #[validate(length(min = 3, max = 100, message = "Campaign name must be between 3 and 100 characters"))]
    pub name: String,
    
    #[validate(length(min = 5, max = 100, message = "Subject must be between 5 and 100 characters"))]
    pub subject: String,
    
    #[validate(length(min = 10, message = "Content must be at least 10 characters"))]
    pub content: String,
    
    pub recipient_list: EmailRecipientList,
    
    pub scheduled_time: Option<DateTime<Utc>>,
}

// Product recommendation
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductRecommendation {
    pub product_id: Uuid,
    pub name: String,
    pub image: String,
    pub price: f64,
    pub relevance_score: f32,
    pub recommendation_type: String, // "similar", "frequently_bought_together", "customers_also_viewed", "trending"
}

// Get recommendations request
#[derive(Debug, Deserialize)]
pub struct GetRecommendationsRequest {
    pub product_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub recommendation_type: Option<String>,
    pub limit: Option<i32>,
}

// Social media post
#[derive(Debug, Serialize, Deserialize)]
pub struct SocialMediaPost {
    pub id: Uuid,
    pub seller_id: Uuid,
    pub content: String,
    pub image_urls: Vec<String>,
    pub platform: String, // "facebook", "twitter", "instagram", etc.
    pub product_ids: Vec<Uuid>,
    pub scheduled_time: Option<DateTime<Utc>>,
    pub posted_time: Option<DateTime<Utc>>,
    pub status: String, // "draft", "scheduled", "posted", "failed"
    pub post_url: Option<String>,
    pub engagement: Option<SocialEngagement>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Social media engagement metrics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SocialEngagement {
    pub likes: i32,
    pub shares: i32,
    pub comments: i32,
    pub clicks: i32,
    pub reach: i32,
}

// Create social media post request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateSocialPostRequest {
    #[validate(length(min = 1, max = 280, message = "Content must be between 1 and 280 characters"))]
    pub content: String,
    
    pub image_urls: Vec<String>,
    
    pub platform: String,
    
    pub product_ids: Vec<Uuid>,
    
    pub scheduled_time: Option<DateTime<Utc>>,
}
