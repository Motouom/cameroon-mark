use axum::{
    extract::{State, Query},
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::models::analytics::AnalyticsTimeRange;
use crate::services::analytics::AnalyticsService;
use crate::services::user;
use crate::middlewares::auth::ExtractUserId;
use crate::errors::{AppError, ApiResponse, Result};
use crate::AppState;
use crate::models::user::UserRole;

pub async fn get_seller_analytics(
    State(state): State<Arc<AppState>>,
    user_id: ExtractUserId,
    Query(time_range): Query<AnalyticsTimeRange>,
) -> Result<impl IntoResponse> {
    // Get the user's role from the database
    let user = user::get_user_by_id(&state.db, user_id.0).await?;
    
    // Ensure the user is a seller
    if user.role != UserRole::Seller {
        return Err(AppError::forbidden("Only sellers can access analytics"));
    }

    // Get analytics data
    let analytics = AnalyticsService::get_seller_analytics(
        &state.db,
        user_id.0,
        Some(time_range)
    ).await?;

    Ok(Json(ApiResponse::success(analytics)))
} 