use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    Json,
    response::IntoResponse,
};
use uuid::Uuid;
use serde::Deserialize;

use crate::{
    errors::{Result, ApiResponse},
    models::marketing::{
        CreateCampaignRequest, 
        CreateDiscountCodeRequest, 
        CreateEmailCampaignRequest,
        GetRecommendationsRequest,
        CreateSocialPostRequest
    },
    services::marketing,
    utils::validation,
    app_state::AppState,
    middlewares::auth::{ExtractUserId, require_seller},
};

#[derive(Deserialize)]
pub struct GenerateDiscountCodeQuery {
    length: Option<usize>,
}

#[derive(Deserialize)]
pub struct ValidateDiscountCodeQuery {
    code: String,
    subtotal: f64,
    #[serde(default)]
    products: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct CampaignQuery {
    active_only: Option<bool>,
}

#[derive(Deserialize)]
pub struct EmailCampaignQuery {
    status: Option<String>,
}

#[derive(Deserialize)]
pub struct SocialMetricsQuery {
    period: Option<String>,
}

// Campaign Handlers
pub async fn create_campaign(
    State(state): State<Arc<AppState>>,
    ExtractUserId(user_id): ExtractUserId,

    Json(payload): Json<CreateCampaignRequest>,
) -> Result<impl IntoResponse> {
    validation::validate(&payload)?;
    let campaign = marketing::create_campaign(&state.db, user_id, payload).await?;
    Ok(Json(ApiResponse::success(campaign)))
}

pub async fn get_campaigns(
    State(state): State<Arc<AppState>>,
    ExtractUserId(user_id): ExtractUserId,

    Query(query): Query<CampaignQuery>,
) -> Result<impl IntoResponse> {
    let active_only = query.active_only.unwrap_or(false);
    let campaigns = marketing::get_campaigns(&state.db, user_id, active_only).await?;
    Ok(Json(ApiResponse::success(campaigns)))
}

// Discount Code Handlers
pub async fn create_discount_code(
    State(state): State<Arc<AppState>>,
    ExtractUserId(user_id): ExtractUserId,

    Json(payload): Json<CreateDiscountCodeRequest>,
) -> Result<impl IntoResponse> {
    validation::validate(&payload)?;
    let code = marketing::create_discount_code(&state.db, user_id, payload).await?;
    Ok(Json(ApiResponse::success(code)))
}

pub async fn generate_discount_code(
    State(state): State<Arc<AppState>>,
    ExtractUserId(user_id): ExtractUserId,

    Query(query): Query<GenerateDiscountCodeQuery>,
) -> Result<impl IntoResponse> {
    let code = marketing::generate_discount_code(&state.db, user_id, query.length).await?;
    Ok(Json(ApiResponse::success(code)))
}

pub async fn validate_discount_code(
    State(state): State<Arc<AppState>>,
    ExtractUserId(user_id): ExtractUserId,
    Query(query): Query<ValidateDiscountCodeQuery>,
) -> Result<impl IntoResponse> {
    let code = marketing::validate_discount_code(
        &state.db, 
        user_id, 
        &query.code, 
        query.subtotal, 
        query.products
    ).await?;
    Ok(Json(ApiResponse::success(code)))
}

// Product Recommendation Handlers
pub async fn get_product_recommendations(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GetRecommendationsRequest>,
) -> Result<impl IntoResponse> {
    let recommendations = marketing::get_product_recommendations(&state.db, payload).await?;
    Ok(Json(ApiResponse::success(recommendations)))
}

// Email Campaign Handlers
pub async fn create_email_campaign(
    State(state): State<Arc<AppState>>,
    ExtractUserId(user_id): ExtractUserId,

    Json(payload): Json<CreateEmailCampaignRequest>,
) -> Result<impl IntoResponse> {
    validation::validate(&payload)?;
    let campaign = marketing::create_email_campaign(&state.db, user_id, payload).await?;
    Ok(Json(ApiResponse::success(campaign)))
}

pub async fn get_email_campaigns(
    State(state): State<Arc<AppState>>,
    ExtractUserId(user_id): ExtractUserId,

    Query(query): Query<EmailCampaignQuery>,
) -> Result<impl IntoResponse> {
    let campaigns = marketing::get_email_campaigns(&state.db, user_id, query.status).await?;
    Ok(Json(ApiResponse::success(campaigns)))
}

// Social Media Handlers
pub async fn create_social_post(
    State(state): State<Arc<AppState>>,
    ExtractUserId(user_id): ExtractUserId,

    Json(payload): Json<CreateSocialPostRequest>,
) -> Result<impl IntoResponse> {
    validation::validate(&payload)?;
    let post = marketing::create_social_post(&state.db, user_id, payload).await?;
    Ok(Json(ApiResponse::success(post)))
}

pub async fn get_social_media_metrics(
    State(state): State<Arc<AppState>>,
    ExtractUserId(user_id): ExtractUserId,

    Query(query): Query<SocialMetricsQuery>,
) -> Result<impl IntoResponse> {
    let metrics = marketing::get_social_media_metrics(&state.db, user_id, query.period).await?;
    Ok(Json(ApiResponse::success(metrics)))
}
