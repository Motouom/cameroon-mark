use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ConnectionTrait,
    QuerySelect, Statement, TransactionTrait,
};
use uuid::Uuid;
use chrono::Utc;

use crate::{
    entities::{user, product, reported_item},
    errors::{AppError, Result},
    models::admin::{DashboardStats, PendingSeller, ReportedItem, ReportItemRequest, SellerActionRequest, ReportActionRequest},
    models::user::{User, UserRole},
};

// Get admin dashboard statistics
pub async fn get_dashboard_stats(db: &DatabaseConnection) -> Result<DashboardStats> {
    let stats_query = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        WITH stats AS (
            SELECT 
                COUNT(*) FILTER (WHERE role = 'buyer') AS total_buyers,
                COUNT(*) FILTER (WHERE role = 'seller') AS total_sellers,
                COUNT(*) FILTER (WHERE role = 'pending_seller') AS pending_sellers,
                COUNT(*) AS total_users
            FROM users
        ),
        product_stats AS (
            SELECT COUNT(*) AS total_products FROM products
        ),
        order_stats AS (
            SELECT 
                COUNT(*) AS total_orders,
                COALESCE(SUM(total_amount), 0) AS total_revenue
            FROM orders
        ),
        report_stats AS (
            SELECT COUNT(*) AS reported_items FROM reported_items WHERE status = 'pending'
        )
        SELECT 
            s.total_users, s.total_sellers, s.total_buyers, s.pending_sellers,
            p.total_products, o.total_orders, o.total_revenue, r.reported_items
        FROM 
            stats s, 
            product_stats p,
            order_stats o,
            report_stats r
        "#,
        vec![],
    );
    
    let row = db.query_one(stats_query)
        .await
        .map_err(|e| AppError::internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::internal("Failed to get dashboard stats"))?;
    
    let stats = DashboardStats {
        total_users: row.try_get::<i64>("", "total_users")?,
        total_sellers: row.try_get::<i64>("", "total_sellers")?,
        total_buyers: row.try_get::<i64>("", "total_buyers")?,
        total_products: row.try_get::<i64>("", "total_products")?,
        total_orders: row.try_get::<i64>("", "total_orders")?,
        total_revenue: row.try_get::<f64>("", "total_revenue")?,
        pending_sellers: row.try_get::<i64>("", "pending_sellers")?,
        reported_items: row.try_get::<i64>("", "reported_items")?,
    };
    
    Ok(stats)
}

// Get all pending sellers
pub async fn get_pending_sellers(db: &DatabaseConnection) -> Result<Vec<PendingSeller>> {
    let pending_sellers_query = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        SELECT id, name, email, phone, created_at
        FROM users
        WHERE role = 'pending_seller'
        ORDER BY created_at DESC
        "#,
        vec![],
    );
    
    let rows = db.query_all(pending_sellers_query)
        .await
        .map_err(|e| AppError::internal(format!("Database error: {}", e)))?;
    
    let mut sellers = Vec::new();
    for row in rows {
        let seller = PendingSeller {
            id: row.try_get::<Uuid>("", "id")?,
            name: row.try_get::<String>("", "name")?,
            email: row.try_get::<String>("", "email")?,
            phone: row.try_get::<Option<String>>("", "phone").ok().flatten(),
            created_at: row.try_get::<chrono::DateTime<Utc>>("", "created_at")?,
        };
        sellers.push(seller);
    }
    
    Ok(sellers)
}

// Approve a pending seller
pub async fn approve_seller(db: &DatabaseConnection, seller_id: Uuid) -> Result<()> {
    let seller = user::Entity::find_by_id(seller_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Seller not found"))?;
    
    // Check if user is a pending seller
    if seller.role != UserRole::PendingSeller {
        return Err(AppError::bad_request("User is not a pending seller"));
    }
    
    // Update user role to seller
    let mut seller_model: user::ActiveModel = seller.into();
    seller_model.role = Set(UserRole::Seller);
    
    // Save changes
    seller_model.update(db).await?;
    
    Ok(())
}

// Reject a pending seller
pub async fn reject_seller(db: &DatabaseConnection, seller_id: Uuid, payload: SellerActionRequest) -> Result<()> {
    let seller = user::Entity::find_by_id(seller_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Seller not found"))?;
    
    // Check if user is a pending seller
    if seller.role != UserRole::PendingSeller {
        return Err(AppError::bad_request("User is not a pending seller"));
    }
    
    // Update user role to buyer
    let mut seller_model: user::ActiveModel = seller.into();
    seller_model.role = Set(UserRole::Customer);
    
    // Save changes
    seller_model.update(db).await?;
    
    // Here you would ideally send an email or notification to the user with the reason
    // for rejection if provided in the payload
    
    Ok(())
}

// Get all reported items
pub async fn get_reported_items(db: &DatabaseConnection) -> Result<Vec<ReportedItem>> {
    let reported_items_query = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        SELECT 
            r.id, r.product_id, p.name as product_name, 
            p.user_id as seller_id, s.name as seller_name,
            r.user_id as reporter_id, u.name as reporter_name,
            r.reason, r.created_at
        FROM 
            reported_items r
            JOIN products p ON r.product_id = p.id
            JOIN users s ON p.user_id = s.id
            JOIN users u ON r.user_id = u.id
        WHERE 
            r.status = 'pending'
        ORDER BY 
            r.created_at DESC
        "#,
        vec![],
    );
    
    let rows = db.query_all(reported_items_query)
        .await
        .map_err(|e| AppError::internal(format!("Database error: {}", e)))?;
    
    let mut items = Vec::new();
    for row in rows {
        let item = ReportedItem {
            id: row.try_get::<Uuid>("", "id")?,
            product_id: row.try_get::<Uuid>("", "product_id")?,
            product_name: row.try_get::<String>("", "product_name")?,
            seller_id: row.try_get::<Uuid>("", "seller_id")?,
            seller_name: row.try_get::<String>("", "seller_name")?,
            reporter_id: row.try_get::<Uuid>("", "reporter_id")?,
            reporter_name: row.try_get::<String>("", "reporter_name")?,
            reason: row.try_get::<String>("", "reason")?,
            created_at: row.try_get::<chrono::DateTime<Utc>>("", "created_at")?,
        };
        items.push(item);
    }
    
    Ok(items)
}

// Report a product
pub async fn report_item(db: &DatabaseConnection, user_id: Uuid, payload: ReportItemRequest) -> Result<()> {
    // Check if product exists
    let product = product::Entity::find_by_id(payload.product_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Product not found"))?;
    
    // Prevent reporting own products
    if product.seller_id == user_id {
        return Err(AppError::bad_request("Cannot report your own product"));
    }
    
    // Create reported item
    let reported_item = reported_item::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        product_id: Set(payload.product_id),
        reason: Set(payload.reason),
        status: Set("pending".to_string()),
        created_at: Set(Utc::now().into()),
        updated_at: Set(Utc::now().into()),
    };
    
    reported_item.insert(db).await?;
    
    Ok(())
}

// Delete a reported item (and the product)
pub async fn delete_reported_item(db: &DatabaseConnection, report_id: Uuid, payload: ReportActionRequest) -> Result<()> {
    let txn = db.begin().await?;
    
    // Get the report
    let report = reported_item::Entity::find_by_id(report_id)
        .one(&txn)
        .await?
        .ok_or_else(|| AppError::not_found("Report not found"))?;
    
    // Update report status
    let mut report_model: reported_item::ActiveModel = report.clone().into();
    report_model.status = Set("deleted".to_string());
    report_model.update(&txn).await?;
    
    // Delete the product
    product::Entity::delete_by_id(report.product_id)
        .exec(&txn)
        .await?;
    
    // Here you would ideally notify the seller about their product being removed
    
    txn.commit().await?;
    
    Ok(())
}

// Ignore a reported item
pub async fn ignore_reported_item(db: &DatabaseConnection, report_id: Uuid) -> Result<()> {
    // Get the report
    let report = reported_item::Entity::find_by_id(report_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Report not found"))?;
    
    // Update report status
    let mut report_model: reported_item::ActiveModel = report.into();
    report_model.status = Set("ignored".to_string());
    report_model.update(db).await?;
    
    Ok(())
}
