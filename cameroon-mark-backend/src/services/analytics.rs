use sea_orm::{
    DatabaseConnection, EntityTrait, QuerySelect, QueryFilter,
    Condition, Order, QueryOrder, JoinType, DbBackend, Statement,
    ColumnTrait, ConnectionTrait,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use bigdecimal::BigDecimal;

use crate::errors::{AppError, Result};
use crate::models::analytics::{
    AnalyticsTimeRange, SalesSummary, TopSellingProduct,
    MonthlySales, ProductPerformance, SellerAnalytics
};
use crate::entities::{order, order_item, product};

pub struct AnalyticsService;

impl AnalyticsService {
    pub async fn get_seller_analytics(
        db: &DatabaseConnection,
        seller_id: Uuid,
        time_range: Option<AnalyticsTimeRange>
    ) -> Result<SellerAnalytics> {
        let time_range = time_range.unwrap_or_default();

        // Get all orders for the seller within the time range using raw SQL
        let orders = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            SELECT DISTINCT o.* 
            FROM orders o
            JOIN order_items oi ON o.id = oi.order_id
            JOIN products p ON oi.product_id = p.id
            WHERE p.seller_id = $1
            AND o.created_at BETWEEN $2 AND $3
            AND o.status != 'canceled'
            "#,
            vec![
                seller_id.into(),
                time_range.start_date.into(),
                time_range.end_date.into(),
            ]
        );

        let orders = db.query_all(orders)
            .await?
            .into_iter()
            .map(|row| -> Result<order::Model> {
                Ok(order::Model {
                    id: row.try_get("", "id")?,
                    user_id: row.try_get("", "user_id")?,
                    total_amount: row.try_get("", "total_amount")?,
                    status: row.try_get("", "status")?,
                    payment_status: row.try_get("", "payment_status")?,
                    payment_method: row.try_get("", "payment_method")?,
                    shipping_address: row.try_get("", "shipping_address")?,
                    created_at: row.try_get("", "created_at")?,
                    updated_at: row.try_get("", "updated_at")?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        // Calculate summary
        let summary = Self::calculate_summary(db, &orders, seller_id).await?;

        // Calculate monthly sales
        let monthly_sales = Self::calculate_monthly_sales(&orders);

        // Get top products
        let top_products = Self::get_top_products(db, seller_id, &time_range).await?;

        Ok(SellerAnalytics {
            summary,
            monthly_sales,
            top_products,
        })
    }

    async fn calculate_summary(
        db: &DatabaseConnection,
        orders: &[order::Model],
        seller_id: Uuid
    ) -> Result<SalesSummary> {
        let total_sales: BigDecimal = orders.iter()
            .map(|o| &o.total_amount)
            .sum();

        let total_orders = orders.len() as i64;

        let average_order_value = if total_orders > 0 {
            total_sales.clone() / BigDecimal::from(total_orders)
        } else {
            BigDecimal::from(0)
        };

        // Get top selling product using raw SQL
        let top_selling_product = if !orders.is_empty() {
            let result = Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"
                SELECT 
                    p.id,
                    p.title,
                    COALESCE(SUM(oi.quantity), 0) as total_quantity,
                    COALESCE(SUM(oi.unit_price * oi.quantity), 0) as total_revenue
                FROM order_items oi
                JOIN products p ON oi.product_id = p.id
                WHERE p.seller_id = $1
                GROUP BY p.id, p.title
                ORDER BY total_quantity DESC NULLS LAST
                LIMIT 1
                "#,
                vec![seller_id.into()]
            );
            
            let row = db.query_one(result)
                .await?
                .ok_or_else(|| AppError::not_found("No top selling product found"))?;

            Some(TopSellingProduct {
                id: row.try_get("", "id")?,
                title: row.try_get("", "title")?,
                total_quantity: row.try_get("", "total_quantity")?,
                total_revenue: row.try_get("", "total_revenue")?,
            })
        } else {
            None
        };

        Ok(SalesSummary {
            total_sales,
            total_orders,
            average_order_value,
            top_selling_product,
        })
    }

    fn calculate_monthly_sales(orders: &[order::Model]) -> Vec<MonthlySales> {
        use std::collections::HashMap;
        use chrono::Datelike;

        let mut sales_by_month: HashMap<String, (BigDecimal, i64)> = HashMap::new();

        for order in orders {
            let month = format!(
                "{}-{:02}",
                order.created_at.year(),
                order.created_at.month()
            );

            let entry = sales_by_month
                .entry(month)
                .or_insert((BigDecimal::from(0), 0));
            
            entry.0 += &order.total_amount;
            entry.1 += 1;
        }

        let mut monthly_sales: Vec<MonthlySales> = sales_by_month
            .into_iter()
            .map(|(month, (sales, orders))| MonthlySales {
                month,
                sales,
                orders,
            })
            .collect();

        // Sort by month
        monthly_sales.sort_by(|a, b| a.month.cmp(&b.month));
        monthly_sales
    }

    async fn get_top_products(
        db: &DatabaseConnection,
        seller_id: Uuid,
        time_range: &AnalyticsTimeRange
    ) -> Result<Vec<ProductPerformance>> {
        let result = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            SELECT 
                p.id,
                p.title,
                COALESCE(SUM(oi.quantity), 0) as total_quantity,
                COALESCE(SUM(oi.unit_price * oi.quantity), 0) as total_revenue,
                CAST(0 AS FLOAT8) as average_rating
            FROM products p
            LEFT JOIN order_items oi ON p.id = oi.product_id
            LEFT JOIN orders o ON oi.order_id = o.id
            WHERE p.seller_id = $1
            AND (o.created_at IS NULL OR o.created_at BETWEEN $2 AND $3)
            GROUP BY p.id, p.title
            ORDER BY total_revenue DESC NULLS LAST
            LIMIT 10
            "#,
            vec![
                seller_id.into(),
                time_range.start_date.into(),
                time_range.end_date.into(),
            ]
        );

        let rows = db.query_all(result).await?;

        let top_products = rows.into_iter()
            .map(|row| {
                Ok(ProductPerformance {
                    id: row.try_get("", "id")?,
                    title: row.try_get("", "title")?,
                    total_quantity: row.try_get("", "total_quantity")?,
                    total_revenue: row.try_get("", "total_revenue")?,
                    average_rating: Some(row.try_get("", "average_rating")?),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(top_products)
    }
} 