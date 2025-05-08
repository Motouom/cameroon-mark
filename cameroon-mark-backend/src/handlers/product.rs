use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    errors::Result,
    models::product::{CreateProductRequest, UpdateProductRequest, ProductFilterOptions, ProductResponse},
    services::product,
    middlewares::auth::ExtractUserId,
    AppState,
};

pub async fn get_products(
    State(state): State<Arc<AppState>>,
    Query(filter): Query<ProductFilterOptions>,
) -> Result<Json<Vec<ProductResponse>>> {
    let products = product::get_products(&state.db, filter).await?;
    Ok(Json(products.into_iter().map(ProductResponse::from).collect()))
}

pub async fn get_product(
    State(state): State<Arc<AppState>>,
    Path(product_id): Path<Uuid>,
) -> Result<Json<ProductResponse>> {
    let product = product::get_product_by_id(&state.db, product_id).await?;
    Ok(Json(ProductResponse::from(product)))
}

pub async fn create_product(
    State(state): State<Arc<AppState>>,
    ExtractUserId(seller_id): ExtractUserId,
    Json(payload): Json<CreateProductRequest>,
) -> Result<Json<ProductResponse>> {
    let product = product::create_product(&state.db, seller_id, payload).await?;
    Ok(Json(ProductResponse::from(product)))
}

pub async fn update_product(
    State(state): State<Arc<AppState>>,
    ExtractUserId(seller_id): ExtractUserId,
    Path(product_id): Path<Uuid>,
    Json(payload): Json<UpdateProductRequest>,
) -> Result<Json<ProductResponse>> {
    let product = product::update_product(&state.db, product_id, seller_id, payload).await?;
    Ok(Json(ProductResponse::from(product)))
}

pub async fn delete_product(
    State(state): State<Arc<AppState>>,
    ExtractUserId(seller_id): ExtractUserId,
    Path(product_id): Path<Uuid>,
) -> Result<StatusCode> {
    product::delete_product(&state.db, product_id, seller_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
