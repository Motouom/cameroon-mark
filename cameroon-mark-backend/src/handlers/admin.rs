use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::{AppError, ApiResponse, Result};
use crate::middlewares::auth::{ExtractUserId, RequireAdmin};
use crate::models::admin::{SellerActionRequest, ReportActionRequest, ReportItemRequest};
use crate::services::admin;
use crate::utils::validation;
use crate::AppState;

// Get admin dashboard statistics
pub async fn get_dashboard_stats(
    State(state): State<Arc<AppState>>,
    _: RequireAdmin,
) -> Result<impl IntoResponse> {
    let stats = admin::get_dashboard_stats(&state.db).await?;
    Ok(Json(ApiResponse::success(stats)))
}

// Get all pending sellers
pub async fn get_pending_sellers(
    State(state): State<Arc<AppState>>,
    _: RequireAdmin,
) -> Result<impl IntoResponse> {
    let pending_sellers = admin::get_pending_sellers(&state.db).await?;
    Ok(Json(ApiResponse::success(pending_sellers)))
}

// Approve a pending seller
pub async fn approve_seller(
    State(state): State<Arc<AppState>>,
    _: RequireAdmin,
    Path(seller_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    admin::approve_seller(&state.db, seller_id).await?;
    Ok(Json(ApiResponse::success_with_message((), "Seller approved successfully")))
}

// Reject a pending seller
pub async fn reject_seller(
    State(state): State<Arc<AppState>>,
    _: RequireAdmin,
    Path(seller_id): Path<Uuid>,
    Json(payload): Json<SellerActionRequest>,
) -> Result<impl IntoResponse> {
    validation::validate(&payload)?;
    admin::reject_seller(&state.db, seller_id, payload).await?;
    Ok(Json(ApiResponse::success_with_message((), "Seller rejected")))
}

// Get all reported items
pub async fn get_reported_items(
    State(state): State<Arc<AppState>>,
    _: RequireAdmin,
) -> Result<impl IntoResponse> {
    let reported_items = admin::get_reported_items(&state.db).await?;
    Ok(Json(ApiResponse::success(reported_items)))
}

// Report a product (non-admin endpoint)
pub async fn report_item(
    State(state): State<Arc<AppState>>,
    ExtractUserId(user_id): ExtractUserId,
    Json(payload): Json<ReportItemRequest>,
) -> Result<impl IntoResponse> {
    validation::validate(&payload)?;
    admin::report_item(&state.db, user_id, payload).await?;
    Ok(Json(ApiResponse::success_with_message((), "Product reported successfully")))
}

// Delete a reported item
pub async fn delete_reported_item(
    State(state): State<Arc<AppState>>,
    _: RequireAdmin,
    Path(report_id): Path<Uuid>,
    Json(payload): Json<ReportActionRequest>,
) -> Result<impl IntoResponse> {
    validation::validate(&payload)?;
    admin::delete_reported_item(&state.db, report_id, payload).await?;
    Ok(Json(ApiResponse::success_with_message((), "Reported item deleted")))
}

// Ignore a reported item
pub async fn ignore_reported_item(
    State(state): State<Arc<AppState>>,
    _: RequireAdmin,
    Path(report_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    admin::ignore_reported_item(&state.db, report_id).await?;
    Ok(Json(ApiResponse::success_with_message((), "Report ignored")))
}
