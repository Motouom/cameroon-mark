use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Canceled,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "pending"),
            OrderStatus::Processing => write!(f, "processing"),
            OrderStatus::Shipped => write!(f, "shipped"),
            OrderStatus::Delivered => write!(f, "delivered"),
            OrderStatus::Canceled => write!(f, "canceled"),
        }
    }
}

impl FromRow<'_, PgRow> for OrderStatus {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let status: String = row.try_get(0)?;
        match status.as_str() {
            "pending" => Ok(OrderStatus::Pending),
            "processing" => Ok(OrderStatus::Processing),
            "shipped" => Ok(OrderStatus::Shipped),
            "delivered" => Ok(OrderStatus::Delivered),
            "canceled" => Ok(OrderStatus::Canceled),
            _ => Err(sqlx::Error::ColumnDecode {
                index: "status".to_string(),
                source: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid order status: {}", status),
                )),
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "payment_status", rename_all = "lowercase")]
pub enum PaymentStatus {
    Pending,
    Paid,
    Failed,
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentStatus::Pending => write!(f, "pending"),
            PaymentStatus::Paid => write!(f, "paid"),
            PaymentStatus::Failed => write!(f, "failed"),
        }
    }
}

impl FromRow<'_, PgRow> for PaymentStatus {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let status: String = row.try_get(0)?;
        match status.as_str() {
            "pending" => Ok(PaymentStatus::Pending),
            "paid" => Ok(PaymentStatus::Paid),
            "failed" => Ok(PaymentStatus::Failed),
            _ => Err(sqlx::Error::ColumnDecode {
                index: "payment_status".to_string(),
                source: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid payment status: {}", status),
                )),
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "payment_method", rename_all = "lowercase")]
pub enum PaymentMethod {
    Mtn,
    Orange,
    Other,
}

impl std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentMethod::Mtn => write!(f, "mtn"),
            PaymentMethod::Orange => write!(f, "orange"),
            PaymentMethod::Other => write!(f, "other"),
        }
    }
}

impl FromRow<'_, PgRow> for PaymentMethod {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let method: String = row.try_get(0)?;
        match method.as_str() {
            "mtn" => Ok(PaymentMethod::Mtn),
            "orange" => Ok(PaymentMethod::Orange),
            "other" => Ok(PaymentMethod::Other),
            _ => Err(sqlx::Error::ColumnDecode {
                index: "payment_method".to_string(),
                source: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid payment method: {}", method),
                )),
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub buyer_id: Uuid,
    pub total_amount: f64,
    pub status: OrderStatus,
    pub payment_status: PaymentStatus,
    pub shipping_name: String,
    pub shipping_address_1: String,
    pub shipping_address_2: Option<String>,
    pub shipping_city: String,
    pub shipping_postal_code: Option<String>,
    pub shipping_country: String,
    pub shipping_phone: Option<String>,
    pub payment_method: PaymentMethod,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub unit_price: f64,
    pub seller_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateOrderRequest {
    #[validate(length(min = 2, max = 100, message = "Shipping name must be between 2 and 100 characters"))]
    pub shipping_name: String,
    
    #[validate(length(min = 5, max = 200, message = "Shipping address must be between 5 and 200 characters"))]
    pub shipping_address_1: String,
    
    pub shipping_address_2: Option<String>,
    
    #[validate(length(min = 2, max = 100, message = "Shipping city must be between 2 and 100 characters"))]
    pub shipping_city: String,
    
    pub shipping_postal_code: Option<String>,
    
    #[validate(length(min = 2, max = 100, message = "Shipping country must be between 2 and 100 characters"))]
    pub shipping_country: String,
    
    pub shipping_phone: Option<String>,
    
    pub payment_method: PaymentMethod,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateOrderStatusRequest {
    pub status: OrderStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub buyer_id: Uuid,
    pub buyer_name: String,
    pub total_amount: f64,
    pub status: OrderStatus,
    pub payment_status: PaymentStatus,
    pub shipping_name: String,
    pub shipping_address_1: String,
    pub shipping_address_2: Option<String>,
    pub shipping_city: String,
    pub shipping_postal_code: Option<String>,
    pub shipping_country: String,
    pub shipping_phone: Option<String>,
    pub payment_method: PaymentMethod,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub items: Vec<OrderItemResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_title: String,
    pub product_image: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub seller_id: Uuid,
    pub seller_name: String,
}
