use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::AppState;
use crate::handlers::marketing::{
    create_campaign,
    get_campaigns,
    create_discount_code,
    generate_discount_code,
    validate_discount_code,
    get_product_recommendations,
    create_email_campaign,
    get_email_campaigns,
    create_social_post,
    get_social_media_metrics,
};

pub fn routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let router = Router::new()
        // Campaign routes
        .route("/campaigns", get(|| async { "Campaigns endpoint" }))
        .route("/campaigns", post(|| async { "Create campaign endpoint" }))
        
        // Discount code routes
        .route("/discount-codes", post(|| async { "Create discount code endpoint" }))
        .route("/discount-codes/generate", get(|| async { "Generate discount code endpoint" }))
        .route("/discount-codes/validate", get(|| async { "Validate discount code endpoint" }))
        
        // Product recommendation routes
        .route("/recommendations", post(|| async { "Product recommendations endpoint" }))
        
        // Email campaign routes
        .route("/email-campaigns", get(|| async { "Email campaigns endpoint" }))
        .route("/email-campaigns", post(|| async { "Create email campaign endpoint" }))
        
        // Social media routes
        .route("/social-posts", post(|| async { "Create social post endpoint" }))
        .route("/social-metrics", get(|| async { "Social media metrics endpoint" }))
        
        .with_state(state);
        
    router
}
