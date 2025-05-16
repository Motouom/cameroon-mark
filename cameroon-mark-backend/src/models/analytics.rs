use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDateTime};
use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct SalesSummary {
    pub total_sales: BigDecimal,
    pub total_orders: i64,
    pub average_order_value: BigDecimal,
    pub top_selling_product: Option<TopSellingProduct>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopSellingProduct {
    pub id: Uuid,
    pub title: String,
    pub total_quantity: i64,
    pub total_revenue: BigDecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthlySales {
    pub month: String,  // Format: "YYYY-MM"
    pub sales: BigDecimal,
    pub orders: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPerformance {
    pub id: Uuid,
    pub title: String,
    pub total_quantity: i64,
    pub total_revenue: BigDecimal,
    pub average_rating: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerAnalytics {
    pub summary: SalesSummary,
    pub monthly_sales: Vec<MonthlySales>,
    pub top_products: Vec<ProductPerformance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsTimeRange {
    #[serde(with = "date_format")]
    pub start_date: DateTime<Utc>,
    #[serde(with = "date_format")]
    pub end_date: DateTime<Utc>,
}

impl Default for AnalyticsTimeRange {
    fn default() -> Self {
        use chrono::Duration;
        let end_date = Utc::now();
        let start_date = end_date - Duration::days(30);
        Self {
            start_date,
            end_date,
        }
    }
}

mod date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let naive = chrono::NaiveDate::parse_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)?
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| serde::de::Error::custom("Invalid time"))?;
        Ok(Utc.from_utc_datetime(&naive))
    }
} 