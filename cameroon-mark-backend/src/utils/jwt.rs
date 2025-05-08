use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config,
    errors::{AppError, Result},
    models::user::{User, UserRole},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,        // Subject (user ID)
    pub role: UserRole,   // User role
    pub exp: i64,         // Expiration time
    pub iat: i64,         // Issued at
}

// Generate a JWT token for a user
pub fn generate_token(user: &User) -> Result<String> {
    let now = Utc::now();
    let exp = (now + Duration::hours(24)).timestamp();

    let claims = Claims {
        sub: user.id,
        role: user.role.clone(),
        exp,
        iat: now.timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config::get_config().jwt.secret.as_bytes()),
    )
    .map_err(|_| AppError::internal("Failed to generate token"))?;

    Ok(token)
}

pub fn verify_token(token: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config::get_config().jwt.secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::auth("Invalid token"))?;

    Ok(token_data.claims)
}
