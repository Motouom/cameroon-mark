use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;
use std::collections::HashMap;

// Customer demographics analytics
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerDemographics {
    pub total_customers: i64,
    pub new_customers_30_days: i64,
    pub gender_distribution: Option<HashMap<String, i64>>,
    pub age_distribution: Option<HashMap<String, i64>>, // "<18", "18-24", "25-34", "35-44", "45-54", "55-64", "65+"
    pub country_distribution: HashMap<String, i64>,
    pub city_distribution: HashMap<String, i64>,
}

// Product performance metrics
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPerformance {
    pub top_selling_products: Vec<TopSellingProduct>,
    pub highest_rated_products: Vec<HighestRatedProduct>,
    pub category_performance: Vec<CategoryPerformance>,
    pub most_viewed_products: Vec<MostViewedProduct>,
    pub most_wished_products: Vec<MostWishedProduct>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopSellingProduct {
    pub id: Uuid,
    pub name: String,
    pub units_sold: i64,
    pub revenue: f64,
    pub category_name: String,
    pub seller_id: Uuid,
    pub seller_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HighestRatedProduct {
    pub id: Uuid,
    pub name: String,
    pub average_rating: f32,
    pub review_count: i64,
    pub category_name: String,
    pub seller_id: Uuid,
    pub seller_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryPerformance {
    pub id: Uuid,
    pub name: String,
    pub units_sold: i64,
    pub revenue: f64,
    pub product_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MostViewedProduct {
    pub id: Uuid,
    pub name: String,
    pub view_count: i64,
    pub seller_id: Uuid,
    pub seller_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MostWishedProduct {
    pub id: Uuid,
    pub name: String,
    pub wish_count: i64,
    pub seller_id: Uuid,
    pub seller_name: String,
}

// Inventory turnover analysis
#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryAnalysis {
    pub overall_turnover_rate: f64, // Average number of times inventory sold in time period
    pub turnover_by_category: Vec<CategoryTurnover>,
    pub low_stock_items: Vec<LowStockItem>,
    pub overstock_items: Vec<OverstockItem>,
    pub dead_stock_items: Vec<DeadStockItem>, // Items with no sales in last 90 days
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryTurnover {
    pub id: Uuid,
    pub name: String,
    pub turnover_rate: f64,
    pub average_days_to_sell: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LowStockItem {
    pub id: Uuid,
    pub name: String,
    pub current_stock: i32,
    pub threshold: i32,
    pub expected_stockout_date: Option<NaiveDate>,
    pub reorder_suggestion: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverstockItem {
    pub id: Uuid,
    pub name: String,
    pub current_stock: i32,
    pub optimal_stock: i32,
    pub days_of_stock: i32,
    pub holding_cost: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadStockItem {
    pub id: Uuid,
    pub name: String,
    pub current_stock: i32,
    pub days_without_sale: i32,
    pub holding_cost: f64,
    pub last_sale_date: Option<NaiveDate>,
}

// Sales forecasting
#[derive(Debug, Serialize, Deserialize)]
pub struct SalesForecasts {
    pub next_7_days: f64,
    pub next_30_days: f64,
    pub next_90_days: f64,
    pub category_forecasts: Vec<CategoryForecast>,
    pub product_forecasts: Vec<ProductForecast>,
    pub seasonal_trends: Vec<SeasonalTrend>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryForecast {
    pub id: Uuid,
    pub name: String,
    pub next_7_days: f64,
    pub next_30_days: f64,
    pub next_90_days: f64,
    pub growth_rate: f32, // Percentage growth expected
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductForecast {
    pub id: Uuid,
    pub name: String,
    pub next_7_days: i32, // Forecasted units
    pub next_30_days: i32,
    pub next_90_days: i32,
    pub confidence_score: f32, // 0.0 to 1.0
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeasonalTrend {
    pub season: String, // "spring", "summer", "fall", "winter", or specific months
    pub category_id: Uuid,
    pub category_name: String,
    pub expected_change: f32, // Percentage increase/decrease
}

// Customer retention metrics
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerRetention {
    pub overall_retention_rate: f32, // Percentage of returning customers
    pub churn_rate: f32, // Percentage of customers lost
    pub average_orders_per_customer: f32,
    pub repeat_purchase_rate: f32,
    pub customer_lifetime_value: f64,
    pub retention_by_cohort: Vec<CustomerCohort>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerCohort {
    pub acquisition_month: String, // "YYYY-MM"
    pub initial_customers: i32,
    pub month_1_retention: f32, // Percentage retained after 1 month
    pub month_3_retention: f32,
    pub month_6_retention: f32,
    pub month_12_retention: f32,
    pub average_value: f64, // Average revenue per customer in cohort
}

// Profit margins calculation
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfitMargins {
    pub gross_margin: f32, // Percentage
    pub net_margin: f32, // Percentage
    pub product_margins: Vec<ProductMargin>,
    pub category_margins: Vec<CategoryMargin>,
    pub cost_breakdown: CostBreakdown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductMargin {
    pub id: Uuid,
    pub name: String,
    pub revenue: f64,
    pub cost: f64,
    pub profit: f64,
    pub margin: f32, // Percentage
    pub units_sold: i32,
    pub profit_per_unit: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryMargin {
    pub id: Uuid,
    pub name: String,
    pub revenue: f64,
    pub cost: f64,
    pub profit: f64,
    pub margin: f32, // Percentage
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostBreakdown {
    pub product_costs: f64,
    pub shipping_costs: f64,
    pub payment_processing_fees: f64,
    pub refunds: f64,
    pub other_costs: f64,
}

// Analytics period
#[derive(Debug, Deserialize)]
pub struct AnalyticsPeriod {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub compare_to_previous: Option<bool>,
}
