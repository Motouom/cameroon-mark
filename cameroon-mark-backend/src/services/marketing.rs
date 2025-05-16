use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait, QuerySelect, QueryOrder, Order, Condition, IntoActiveModel, Statement, Value,
};
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use sea_orm::prelude::Decimal;

use crate::{
    entities::{product, user, order, saved_item, campaign, discount_code, email_campaign},
    errors::{AppError, Result},
    models::marketing::{
        Campaign, CampaignType, CreateCampaignRequest, TargetAudience,
        DiscountCode, DiscountType, CreateDiscountCodeRequest,
        EmailCampaign, EmailRecipientList, CustomerFilters, CreateEmailCampaignRequest,
        ProductRecommendation, GetRecommendationsRequest,
        SocialMediaPost, SocialEngagement, CreateSocialPostRequest
    },
    utils::validation,
};

// Create promotional campaign
pub async fn create_campaign(
    db: &DatabaseConnection,
    seller_id: Uuid,
    payload: CreateCampaignRequest,
) -> Result<Campaign> {
    // Validate request
    validation::validate(&payload)?;

    // Validate dates
    if payload.start_date >= payload.end_date {
        return Err(AppError::bad_request("End date must be after start date"));
    }

    // Begin transaction
    let txn = db.begin().await?;
    
    // Create campaign
    let campaign_id = Uuid::new_v4();
    let now = Utc::now();
    
    let campaign_model = campaign::ActiveModel {
        id: Set(campaign_id),
        seller_id: Set(seller_id),
        name: Set(payload.name.clone()),
        description: Set(payload.description.clone()),
        campaign_type: Set(format!("{:?}", payload.campaign_type)),
        start_date: Set(payload.start_date.into()),
        end_date: Set(payload.end_date.into()),
        budget: Set(Decimal::new(1000, 1)), // 100.0 as Decimal
        status: Set(if payload.is_active { "active" } else { "inactive" }.to_string()),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
    };
    
    let campaign_result = campaign_model.insert(&txn).await?;
    
    // Create discount code if provided
    let mut discount_code_ids = Vec::new();
    
    if let Some(discount_payload) = payload.create_discount_code {
        let discount_request = CreateDiscountCodeRequest {
            code: discount_payload.code,
            discount_type: discount_payload.discount_type,
            discount_value: discount_payload.discount_value,
            min_purchase_amount: discount_payload.min_purchase_amount,
            max_discount_amount: discount_payload.max_discount_amount,
            usage_limit: discount_payload.usage_limit,
            products: discount_payload.products,
            categories: discount_payload.categories,
            start_date: payload.start_date, // Match campaign dates
            end_date: payload.end_date,
            is_active: payload.is_active, // Match campaign status
        };
        
        let discount_code = create_discount_code_internal(&txn, seller_id, Some(campaign_id), discount_request).await?;
        discount_code_ids.push(discount_code.id);
    }
    
    // No need to update campaign with discount code IDs as the field doesn't exist in our model
    // Just keep track of the IDs for the response
    let updated_campaign = campaign_result;
    
    // Commit transaction
    txn.commit().await?;
    
    // Convert to Campaign model
    let campaign = Campaign {
        id: updated_campaign.id,
        seller_id: updated_campaign.seller_id,
        name: updated_campaign.name,
        description: updated_campaign.description,
        campaign_type: match updated_campaign.campaign_type.as_str() {
            "FlashSale" => CampaignType::FlashSale,
            "ProductLaunch" => CampaignType::ProductLaunch,
            "Seasonal" => CampaignType::Seasonal,
            "Clearance" => CampaignType::Clearance,
            "BundleDeal" => CampaignType::BundleDeal,
            "Loyalty" => CampaignType::Loyalty,
            _ => CampaignType::Custom,
        },
        start_date: updated_campaign.start_date.into(),
        end_date: updated_campaign.end_date.into(),
        is_active: updated_campaign.status == "active",
        target_audience: None, // Entity doesn't have this field
        discount_codes: discount_code_ids,
        banner_image: None, // Entity doesn't have this field
        created_at: updated_campaign.created_at.into(),
        updated_at: updated_campaign.updated_at.into(),
    };
    
    Ok(campaign)
}

// Get all campaigns for a seller
pub async fn get_campaigns(
    db: &DatabaseConnection,
    seller_id: Uuid,
    active_only: bool,
) -> Result<Vec<Campaign>> {
    let mut query = campaign::Entity::find()
        .filter(campaign::Column::SellerId.eq(seller_id))
        .order_by(campaign::Column::CreatedAt, Order::Desc);
    
    if active_only {
        query = query.filter(campaign::Column::Status.eq("active"));
    }
    
    let campaigns = query.all(db).await?;
    
    let result: Vec<Campaign> = campaigns
        .into_iter()
        .map(|c| {
            // Since we don't have discount_codes field in our entity, we'll use an empty vector
            let discount_codes: Vec<Uuid> = vec![];
            
            Campaign {
                id: c.id,
                seller_id: c.seller_id,
                name: c.name,
                description: c.description,
                campaign_type: match c.campaign_type.as_str() {
                    "FlashSale" => CampaignType::FlashSale,
                    "ProductLaunch" => CampaignType::ProductLaunch,
                    "Seasonal" => CampaignType::Seasonal,
                    "Clearance" => CampaignType::Clearance,
                    "BundleDeal" => CampaignType::BundleDeal,
                    "Loyalty" => CampaignType::Loyalty,
                    _ => CampaignType::Custom,
                },
                start_date: c.start_date.into(),
                end_date: c.end_date.into(),
                is_active: c.status == "active",
                target_audience: None, // Entity doesn't have this field
                discount_codes,
                banner_image: None, // Entity doesn't have this field
                created_at: c.created_at.into(),
                updated_at: c.updated_at.into(),
            }
        })
        .collect();
    
    Ok(result)
}

// Create discount code with internal helper
async fn create_discount_code_internal(
    db: &impl sea_orm::ConnectionTrait,
    seller_id: Uuid,
    campaign_id: Option<Uuid>,
    payload: CreateDiscountCodeRequest,
) -> Result<DiscountCode> {
    // Validate payload
    validation::validate(&payload)?;

    // Check if code already exists
    let existing_code = discount_code::Entity::find()
        .filter(discount_code::Column::Code.eq(&payload.code))
        .filter(discount_code::Column::SellerId.eq(seller_id))
        .one(db)
        .await?;
    
    if existing_code.is_some() {
        return Err(AppError::bad_request("Discount code already exists"));
    }
    
    // Validate dates
    if payload.start_date >= payload.end_date {
        return Err(AppError::bad_request("End date must be after start date"));
    }
    
    // Create discount code
    let code_id = Uuid::new_v4();
    let now = Utc::now();
    
    let code_model = discount_code::ActiveModel {
        id: Set(code_id),
        seller_id: Set(seller_id),
        campaign_id: Set(campaign_id),
        code: Set(payload.code.clone()),
        discount_type: Set(format!("{:?}", payload.discount_type)),
        value: Set(Decimal::new((payload.discount_value * 100.0) as i64, 2)), // Convert from f64 to Decimal
        min_purchase_amount: Set(payload.min_purchase_amount.map(|val| Decimal::new((val * 100.0) as i64, 2))),
        max_uses: Set(payload.usage_limit),
        times_used: Set(0),
        start_date: Set(payload.start_date.into()),
        end_date: Set(payload.end_date.into()),
        is_active: Set(payload.is_active),
        created_at: Set(now.into()),
        updated_at: Set(now.into())
    };
    
    let code_result = code_model.insert(db).await?;
    
    // Convert to DiscountCode response
    let code = DiscountCode {
        id: code_result.id,
        seller_id: code_result.seller_id,
        campaign_id: code_result.campaign_id,
        code: code_result.code,
        discount_type: match code_result.discount_type.as_str() {
            "Percentage" => DiscountType::Percentage,
            "FixedAmount" => DiscountType::FixedAmount,
            "BuyXGetY" => DiscountType::BuyXGetY,
            "FreeShipping" => DiscountType::FreeShipping,
            _ => DiscountType::Bundled,
        },
        discount_value: code_result.value.to_string().parse::<f64>().unwrap_or(0.0),
        min_purchase_amount: code_result.min_purchase_amount.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
        max_discount_amount: None, // Entity doesn't have this field
        usage_limit: code_result.max_uses,
        usage_count: code_result.times_used,
        products: Some(vec![]), // Not included in the response
        categories: Some(vec![]), // Not included in the response
        start_date: code_result.start_date.into(),
        end_date: code_result.end_date.into(),
        is_active: code_result.is_active,
        created_at: code_result.created_at.into(),
        updated_at: code_result.updated_at.into(),
    };
    
    Ok(code)
}

// Public method to create discount code
pub async fn create_discount_code(
    db: &DatabaseConnection,
    seller_id: Uuid,
    payload: CreateDiscountCodeRequest,
) -> Result<DiscountCode> {
    create_discount_code_internal(db, seller_id, None, payload).await
}

// Generate a random discount code
pub async fn generate_discount_code(
    db: &DatabaseConnection,
    seller_id: Uuid,
    length: Option<usize>,
) -> Result<String> {
    let code_length = length.unwrap_or(8);
    let mut rng = thread_rng();
    
    // Try up to 5 times to generate a unique code
    for _ in 0..5 {
        let code: String = (0..code_length)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect::<String>()
            .to_uppercase();
        
        // Check if code exists
        let existing_code = discount_code::Entity::find()
            .filter(discount_code::Column::Code.eq(&code))
            .filter(discount_code::Column::SellerId.eq(seller_id))
            .one(db)
            .await?;
        
        if existing_code.is_none() {
            return Ok(code);
        }
    }
    
    Err(AppError::internal("Failed to generate unique discount code"))
}

// Validate a discount code
pub async fn validate_discount_code(
    db: &DatabaseConnection,
    user_id: Uuid,
    code: &str,
    subtotal: f64,
    products: Vec<Uuid>,
) -> Result<DiscountCode> {
    // Find code
    let discount = discount_code::Entity::find()
        .filter(discount_code::Column::Code.eq(code))
        .filter(discount_code::Column::IsActive.eq(true))
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Discount code not found or inactive"))?;
    
    // Check dates
    let now = Utc::now();
    if now < discount.start_date || now > discount.end_date {
        return Err(AppError::bad_request("Discount code is not valid at this time"));
    }
    
    // Check usage limit
    if let Some(limit) = discount.max_uses {
        if discount.times_used >= limit {
            return Err(AppError::bad_request("Discount code has reached its usage limit"));
        }
    }
    
    // Check minimum purchase amount
    if let Some(min_amount) = discount.min_purchase_amount {
        // Convert Decimal to f64 for comparison
        let min_amount_f64 = min_amount.to_string().parse::<f64>().unwrap_or(0.0);
        if subtotal < min_amount_f64 {
            return Err(AppError::bad_request(format!("Order subtotal must be at least {}", min_amount_f64)));
        }
    }
    
    // Our entity model doesn't have product or category restrictions fields,
    // so we'll skip those validations for now
    // In a future update, we could add these fields to the entity model if needed
    
    // Convert to DiscountCode model
    let discount_code = DiscountCode {
        id: discount.id,
        seller_id: discount.seller_id,
        campaign_id: discount.campaign_id,
        code: discount.code,
        discount_type: match discount.discount_type.as_str() {
            "Percentage" => DiscountType::Percentage,
            "FixedAmount" => DiscountType::FixedAmount,
            "BuyXGetY" => DiscountType::BuyXGetY,
            "FreeShipping" => DiscountType::FreeShipping,
            _ => DiscountType::Bundled,
        },
        discount_value: discount.value.to_string().parse::<f64>().unwrap_or(0.0),
        min_purchase_amount: discount.min_purchase_amount.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
        max_discount_amount: None, // Entity doesn't have this field
        usage_limit: discount.max_uses,
        usage_count: discount.times_used,
        products: Some(vec![]), // Entity doesn't have this field
        categories: Some(vec![]), // Entity doesn't have this field
        start_date: discount.start_date.into(),
        end_date: discount.end_date.into(),
        is_active: discount.is_active,
        created_at: discount.created_at.into(),
        updated_at: discount.updated_at.into(),
    };
    
    Ok(discount_code)
}

// Apply discount to cart
pub async fn apply_discount(
    discount: &DiscountCode,
    subtotal: f64,
    products: Vec<(Uuid, f64, i32)>, // (product_id, price, quantity)
) -> Result<f64> {
    let mut discount_amount = 0.0;
    
    match discount.discount_type {
        DiscountType::Percentage => {
            // Use the discount_value from the DiscountCode model
            discount_amount = subtotal * (discount.discount_value / 100.0);
        },
        DiscountType::FixedAmount => {
            // Use the discount_value from the DiscountCode model
            discount_amount = discount.discount_value;
            if discount_amount > subtotal {
                discount_amount = subtotal;
            }
        },
        DiscountType::FreeShipping => {
            // Free shipping is handled separately at checkout
            discount_amount = 0.0;
        },
        DiscountType::BuyXGetY => {
            // Complex logic for BOGO discounts would go here
            // For simplicity, treating as a percentage discount
            discount_amount = subtotal * 0.1; // 10% off as placeholder
        },
        DiscountType::Bundled => {
            // Complex logic for bundle discounts would go here
            discount_amount = subtotal * 0.15; // 15% off as placeholder
        },
    }
    
    // Our entity doesn't have a max_discount_amount field, so we'll skip this check
    // In a production system, we might want to add this field to the entity
    
    Ok(discount_amount)
}

// Increment discount code usage count
pub async fn increment_discount_usage(
    db: &DatabaseConnection,
    discount_id: Uuid,
) -> Result<()> {
    let discount = discount_code::Entity::find_by_id(discount_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Discount code not found"))?;
    
    let mut discount_model = discount.into_active_model();
    discount_model.times_used = Set(discount_model.times_used.unwrap() + 1);
    discount_model.updated_at = Set(Utc::now().into());
    
    discount_model.update(db).await?;
    
    Ok(())
}

// Get product recommendations
pub async fn get_product_recommendations(
    db: &DatabaseConnection,
    payload: GetRecommendationsRequest,
) -> Result<Vec<ProductRecommendation>> {
    let limit = payload.limit.unwrap_or(10) as u64;
    let mut recommendations = Vec::new();
    
    match payload.recommendation_type.as_deref() {
        Some("similar") => {
            // Get similar products based on category and attributes
            if let Some(product_id) = payload.product_id {
                let product = product::Entity::find_by_id(product_id)
                    .one(db)
                    .await?
                    .ok_or_else(|| AppError::not_found("Product not found"))?;
                
                // Find products in the same category
                let similar_products = product::Entity::find()
                    .filter(product::Column::CategoryId.eq(product.category_id))
                    .filter(product::Column::Id.ne(product_id))
                    .limit(limit)
                    .all(db)
                    .await?;
                
                recommendations = similar_products.into_iter()
                    .map(|p| ProductRecommendation {
                        product_id: p.id,
                        name: p.title,
                        image: p.images.0.first().cloned().unwrap_or_default(),
                        price: p.price.to_string().parse::<f64>().unwrap_or(0.0),
                        relevance_score: 0.95, // Placeholder
                        recommendation_type: "similar".to_string(),
                    })
                    .collect();
            }
        },
        Some("frequently_bought_together") => {
            // Get products frequently bought together
            if let Some(product_id) = payload.product_id {
                // This would typically query order_item data to find co-occurring products
                // Simplified implementation for prototype
                let related_products = product::Entity::find()
                    .limit(limit)
                    .all(db)
                    .await?;
                
                recommendations = related_products.into_iter()
                    .filter(|p| p.id != product_id)
                    .map(|p| ProductRecommendation {
                        product_id: p.id,
                        name: p.title,
                        image: p.images.0.first().cloned().unwrap_or_default(),
                        price: p.price.to_string().parse::<f64>().unwrap_or(0.0),
                        relevance_score: 0.85, // Placeholder
                        recommendation_type: "frequently_bought_together".to_string(),
                    })
                    .collect();
            }
        },
        Some("trending") => {
            // Get trending products based on views and purchases
            // This would query product_view and order_item tables
            // Simplified for prototype
            let trending_products = product::Entity::find()
                .limit(limit)
                .all(db)
                .await?;
            
            recommendations = trending_products.into_iter()
                .map(|p| ProductRecommendation {
                    product_id: p.id,
                    name: p.title,
                    image: p.images.0.first().cloned().unwrap_or_default(),
                    price: p.price.to_string().parse::<f64>().unwrap_or(0.0),
                    relevance_score: 0.9, // Placeholder
                    recommendation_type: "trending".to_string(),
                })
                .collect();
        },
        Some("for_you") | None => {
            // Personalized recommendations based on user's history
            if let Some(user_id) = payload.user_id {
                // Combine viewed products, purchased products, and saved items
                // Simplified for prototype
                let user_products = saved_item::Entity::find()
                    .filter(saved_item::Column::UserId.eq(user_id))
                    .find_also_related(product::Entity)
                    .all(db)
                    .await?;
                
                let personalized = product::Entity::find()
                    .limit(limit)
                    .all(db)
                    .await?;
                
                // Combine results
                let mut product_map = HashMap::new();
                
                for (_, maybe_product) in user_products {
                    if let Some(product) = maybe_product {
                        product_map.insert(product.id, product);
                    }
                }
                
                for p in personalized {
                    if !product_map.contains_key(&p.id) {
                        product_map.insert(p.id, p);
                    }
                }
                
                recommendations = product_map.values()
                    .take(limit as usize)
                    .map(|p| ProductRecommendation {
                        product_id: p.id,
                        name: p.title.clone(),
                        image: p.images.0.first().cloned().unwrap_or_default().clone(),
                        price: p.price.to_string().parse::<f64>().unwrap_or(0.0),
                        relevance_score: 0.9, // Placeholder
                        recommendation_type: "for_you".to_string(),
                    })
                    .collect();
            }
        },
        _ => {
            // Default to trending products if unrecognized type
            let default_products = product::Entity::find()
                .limit(limit)
                .all(db)
                .await?;
            
            recommendations = default_products.into_iter()
                .map(|p| ProductRecommendation {
                    product_id: p.id,
                    name: p.title,
                    image: p.images.0.first().cloned().unwrap_or_default(),
                    price: p.price.to_string().parse::<f64>().unwrap_or(0.0),
                    relevance_score: 0.8, // Placeholder
                    recommendation_type: "recommended".to_string(),
                })
                .collect();
        }
    }
    
    Ok(recommendations)
}

// Create email marketing campaign
pub async fn create_email_campaign(
    db: &DatabaseConnection,
    seller_id: Uuid,
    payload: CreateEmailCampaignRequest,
) -> Result<EmailCampaign> {
    // Validate request
    validation::validate(&payload)?;

    // Create campaign
    let campaign_id = Uuid::new_v4();
    let now = Utc::now();
    
    let campaign_model = email_campaign::ActiveModel {
        id: Set(campaign_id),
        seller_id: Set(seller_id),
        campaign_id: Set(None),
        subject: Set(payload.subject),
        content: Set(payload.content),
        recipient_list: Set(serde_json::to_value(&payload.recipient_list).unwrap_or_default().to_string()),
        scheduled_time: Set(payload.scheduled_time.map(|dt| dt.into())),
        sent_time: Set(None),
        status: Set(if payload.scheduled_time.is_some() { "scheduled" } else { "draft" }.to_string()),
        opened_count: Set(0),
        click_count: Set(0),
        created_at: Set(now.into()),
        updated_at: Set(now.into())
    };
    
    let campaign_result = campaign_model.insert(db).await?;
    
    // Convert to EmailCampaign model
    let campaign = EmailCampaign {
        id: campaign_result.id,
        seller_id: campaign_result.seller_id,
        name: "Email Campaign".to_string(), // Default name since entity doesn't have this field
        subject: campaign_result.subject,
        content: campaign_result.content,
        recipient_list: match serde_json::from_str::<EmailRecipientList>(&campaign_result.recipient_list) {
            Ok(list) => list,
            Err(_) => EmailRecipientList {
                all_customers: true,
                specific_customers: None,
                customer_filters: None,
            }
        },
        scheduled_time: campaign_result.scheduled_time.map(|t| t.into()),
        sent_time: campaign_result.sent_time.map(|t| t.into()),
        status: campaign_result.status,
        open_count: campaign_result.opened_count,
        click_count: campaign_result.click_count,
        created_at: campaign_result.created_at.into(),
        updated_at: campaign_result.updated_at.into(),
    };
    
    Ok(campaign)
}

// Get email campaigns for a seller
pub async fn get_email_campaigns(
    db: &DatabaseConnection,
    seller_id: Uuid,
    status: Option<String>,
) -> Result<Vec<EmailCampaign>> {
    let mut query = email_campaign::Entity::find()
        .filter(email_campaign::Column::SellerId.eq(seller_id))
        .order_by(email_campaign::Column::CreatedAt, Order::Desc);
    
    if let Some(status_filter) = status {
        query = query.filter(email_campaign::Column::Status.eq(status_filter));
    }
    
    let campaigns = query.all(db).await?;
    
    let result = campaigns.into_iter()
        .map(|c| {
            EmailCampaign {
                id: c.id,
                seller_id: c.seller_id,
                name: "Email Campaign".to_string(), // Default name since our entity doesn't have this field
                subject: c.subject,
                content: c.content,
                recipient_list: match serde_json::from_str::<EmailRecipientList>(&c.recipient_list) {
                    Ok(list) => list,
                    Err(_) => EmailRecipientList {
                        all_customers: true,
                        specific_customers: None,
                        customer_filters: None,
                    }
                },
                scheduled_time: c.scheduled_time.map(|t| t.into()),
                sent_time: c.sent_time.map(|t| t.into()),
                status: c.status,
                open_count: c.opened_count,
                click_count: c.click_count,
                created_at: c.created_at.into(),
                updated_at: c.updated_at.into(),
            }
        })
        .collect();
    
    Ok(result)
}

// Create social media post
pub async fn create_social_post(
    db: &DatabaseConnection,
    seller_id: Uuid,
    payload: CreateSocialPostRequest,
) -> Result<SocialMediaPost> {
    // Validate request
    validation::validate(&payload)?;

    // Create post
    let post_id = Uuid::new_v4();
    let now = Utc::now();
    
    // In a real implementation, this would integrate with social media platforms
    // For now, just store the post data
    
    // Save to database (using raw SQL as example since we don't have a social_post entity defined)
    let post = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"INSERT INTO social_posts 
            (id, seller_id, content, image_url, platforms, scheduled_time, status, created_at, updated_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
           RETURNING id, seller_id, content, image_url, platforms, scheduled_time, status, created_at, updated_at"#,
        vec![
            post_id.into(),
            seller_id.into(),
            payload.content.clone().into(),
            payload.image_urls.clone().into(),
            serde_json::to_value(&payload.platform).unwrap_or_default().into(),
            if let Some(time) = payload.scheduled_time { time.into() } else { sea_orm::Value::String(Some(Box::new(String::new()))) },
            (if payload.scheduled_time.is_some() { "scheduled" } else { "draft" }).into(),
            now.into(),
            now.into(),
        ],
    );
    
    // Execute statement and construct return object
    // Since we're mocking this functionality, we'll just create a return object directly
    let social_post = SocialMediaPost {
        id: post_id,
        seller_id,
        content: payload.content,
        image_urls: payload.image_urls.clone(),
        platform: payload.platform.clone(),
        product_ids: payload.product_ids.clone(),
        scheduled_time: payload.scheduled_time,
        posted_time: None,
        status: if payload.scheduled_time.is_some() { "scheduled" } else { "draft" }.to_string(),
        post_url: None,
        engagement: Some(SocialEngagement {
            likes: 0,
            shares: 0,
            comments: 0,
            clicks: 0,
            reach: 0,
        }),
        created_at: now,
        updated_at: now,
    };
    
    Ok(social_post)
}

// Get social media post metrics
pub async fn get_social_media_metrics(
    db: &DatabaseConnection,
    seller_id: Uuid,
    period: Option<String>,
) -> Result<HashMap<String, SocialEngagement>> {
    // In a real implementation, this would query actual metrics from social media platforms
    // or from a database that stores this information
    
    // Mock implementation for demonstration purposes
    let mut metrics = HashMap::new();
    
    metrics.insert("facebook".to_string(), SocialEngagement {
        likes: 150,
        shares: 45,
        comments: 30,
        reach: 1200,
        clicks: 120,
    });
    
    metrics.insert("instagram".to_string(), SocialEngagement {
        likes: 320,
        shares: 0, // Instagram doesn't have traditional shares
        comments: 75,
        reach: 2500,
        clicks: 180, // Adding required field
    });
    
    metrics.insert("twitter".to_string(), SocialEngagement {
        likes: 95,
        shares: 65, // Retweets
        comments: 25,
        reach: 800,
        clicks: 110,
    });
    
    Ok(metrics)
}
