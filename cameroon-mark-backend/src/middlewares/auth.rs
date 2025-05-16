use std::future::Future;
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode, Request},
    response::{IntoResponse, Response},
    middleware::Next,
    body::Body,
};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::user::UserRole,
    utils::jwt,
    AppState,
};
use std::sync::Arc;

#[derive(Debug)]
pub struct ExtractUserId(pub Uuid);

#[derive(Debug)]
pub struct ExtractUserRole(pub UserRole);

#[async_trait]
impl<'a ,S> FromRequestParts<S> for ExtractUserId
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or_else(|| AppError::auth("Missing authorization header"))?
            .to_str()
            .map_err(|_| AppError::auth("Invalid authorization header"))?;
    
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::auth("Invalid authorization header format"))?;
    
        let claims = jwt::verify_token(token)?;
        Ok(ExtractUserId(claims.sub))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ExtractUserRole
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or_else(|| AppError::auth("Missing authorization header"))?
            .to_str()
            .map_err(|_| AppError::auth("Invalid authorization header"))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::auth("Invalid authorization header format"))?;

        let claims = jwt::verify_token(token)?;
        Ok(ExtractUserRole(claims.role))
    }
}

pub async fn require_auth(ExtractUserId(user_id): ExtractUserId) -> Result<Uuid, Response> {
    Ok(user_id)
}

pub async fn require_seller(
    request: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .ok_or_else(|| AppError::auth("Missing authorization header").into_response())?
        .to_str()
        .map_err(|_| AppError::auth("Invalid authorization header").into_response())?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::auth("Invalid authorization header format").into_response())?;

    let claims = jwt::verify_token(token)
        .map_err(|e| e.into_response())?;

    if claims.role != UserRole::Seller {
        return Err(AppError::forbidden("Seller access required").into_response());
    }

    Ok(next.run(request).await)
}

// Admin middleware
#[derive(Debug)]
pub struct RequireAdmin;

#[async_trait]
impl<S> FromRequestParts<S> for RequireAdmin
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or_else(|| AppError::auth("Missing authorization header"))?
            .to_str()
            .map_err(|_| AppError::auth("Invalid authorization header"))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::auth("Invalid authorization header format"))?;

        let claims = jwt::verify_token(token)?;
        
        // Check if the user is an admin - update this based on your user roles implementation
        // You might need to modify this condition based on how admin role is defined in your system
        if claims.role != UserRole::Admin {
            return Err(AppError::forbidden("Admin access required"));
        }

        Ok(RequireAdmin)
    }
}