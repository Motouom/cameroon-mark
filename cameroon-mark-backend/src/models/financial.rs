use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;
use validator::Validate;
use std::collections::HashMap;

// Payment processor types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PaymentProcessor {
    MtnMobileMoney,
    OrangeMoney,
    CreditCard,
    BankTransfer,
    CashOnDelivery,
}

// Payment status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Refunded,
    PartiallyRefunded,
    Cancelled,
}

// Payment details
#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub processor: PaymentProcessor,
    pub processor_payment_id: Option<String>,
    pub status: PaymentStatus,
    pub payment_method: String,
    pub metadata: Option<HashMap<String, String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

// Process payment request
#[derive(Debug, Deserialize, Validate)]
pub struct ProcessPaymentRequest {
    pub order_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub processor: PaymentProcessor,
    pub payment_method: String,
    pub phone_number: Option<String>, // For mobile money
    pub return_url: Option<String>, // For redirect-based payments
    pub metadata: Option<HashMap<String, String>>,
}

// Payment response
#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub payment_id: Uuid,
    pub status: PaymentStatus,
    pub redirect_url: Option<String>,
    pub processor_reference: Option<String>,
    pub message: String,
}

// Refund request
#[derive(Debug, Deserialize, Validate)]
pub struct RefundRequest {
    pub payment_id: Uuid,
    pub amount: Option<f64>, // If not provided, full refund
    pub reason: String,
}

// Revenue report
#[derive(Debug, Serialize, Deserialize)]
pub struct RevenueReport {
    pub total_revenue: f64,
    pub net_revenue: f64,
    pub processing_fees: f64,
    pub refunds: f64,
    pub taxes_collected: f64,
    pub revenue_by_day: HashMap<String, f64>,
    pub revenue_by_payment_method: HashMap<String, f64>,
    pub revenue_by_category: Vec<CategoryRevenue>,
    pub top_products: Vec<ProductRevenue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryRevenue {
    pub category_id: Uuid,
    pub category_name: String,
    pub revenue: f64,
    pub percentage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductRevenue {
    pub product_id: Uuid,
    pub product_name: String,
    pub revenue: f64,
    pub units_sold: i32,
}

// Tax calculation
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxCalculation {
    pub order_id: Option<Uuid>,
    pub subtotal: f64,
    pub shipping_cost: f64,
    pub taxes: Vec<TaxItem>,
    pub total_tax: f64,
    pub grand_total: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxItem {
    pub name: String,
    pub rate: f32,
    pub amount: f64,
}

// Tax calculation request
#[derive(Debug, Deserialize, Validate)]
pub struct CalculateTaxRequest {
    pub order_id: Option<Uuid>,
    pub subtotal: f64,
    pub shipping_cost: f64,
    pub shipping_address: Option<TaxAddress>,
    pub items: Vec<TaxableItem>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TaxAddress {
    pub country: String,
    pub state_province: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TaxableItem {
    pub product_id: Uuid,
    pub quantity: i32,
    pub unit_price: f64,
    pub product_type: Option<String>,
    pub tax_code: Option<String>,
}

// Expense tracking
#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub category: String,
    pub description: String,
    pub date: NaiveDate,
    pub receipt_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Create expense request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateExpenseRequest {
    pub amount: f64,
    pub currency: String,
    
    #[validate(length(min = 1, max = 100, message = "Category must be between 1 and 100 characters"))]
    pub category: String,
    
    #[validate(length(min = 1, max = 500, message = "Description must be between 1 and 500 characters"))]
    pub description: String,
    
    pub date: NaiveDate,
    pub receipt_url: Option<String>,
}

// Financial statement
#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialStatement {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub income_statement: IncomeStatement,
    pub balance_sheet: BalanceSheet,
    pub cash_flow: CashFlow,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeStatement {
    pub revenue: f64,
    pub cost_of_goods_sold: f64,
    pub gross_profit: f64,
    pub gross_margin: f32,
    pub operating_expenses: HashMap<String, f64>,
    pub total_operating_expenses: f64,
    pub operating_income: f64,
    pub other_income: f64,
    pub other_expenses: f64,
    pub net_income: f64,
    pub net_margin: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceSheet {
    pub assets: HashMap<String, f64>,
    pub total_assets: f64,
    pub liabilities: HashMap<String, f64>,
    pub total_liabilities: f64,
    pub equity: HashMap<String, f64>,
    pub total_equity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CashFlow {
    pub operating_activities: HashMap<String, f64>,
    pub net_cash_from_operating: f64,
    pub investing_activities: HashMap<String, f64>,
    pub net_cash_from_investing: f64,
    pub financing_activities: HashMap<String, f64>,
    pub net_cash_from_financing: f64,
    pub net_change_in_cash: f64,
    pub beginning_cash_balance: f64,
    pub ending_cash_balance: f64,
}

// Financial report request
#[derive(Debug, Deserialize)]
pub struct FinancialReportRequest {
    pub report_type: String, // "revenue", "expenses", "taxes", "financial_statement"
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub compare_to_previous: Option<bool>,
}
