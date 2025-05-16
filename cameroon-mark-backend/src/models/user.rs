use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use uuid::Uuid;
use validator::{Validate, ValidationError};
use sqlx::types::BigDecimal;
use std::str::FromStr;
use serde::de::{self, Deserializer, Visitor};
use sea_orm::entity::prelude::*;
use sea_orm::DeriveActiveEnum;
use std::collections::HashSet;
use crate::entities::user;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub address_street: Option<String>,
    pub address_city: Option<String>,
    pub address_postal_code: Option<String>,
    pub address_country: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_role")]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    #[sea_orm(string_value = "seller")]
    Seller,
    #[sea_orm(string_value = "customer")]
    Customer,
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "pending_seller")]
    PendingSeller,
}

impl FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "seller" => Ok(UserRole::Seller),
            "customer" => Ok(UserRole::Customer),
            "buyer" => Ok(UserRole::Customer), // For backward compatibility
            "admin" => Ok(UserRole::Admin),
            "pending_seller" => Ok(UserRole::PendingSeller),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

// User profile data that can be returned to clients
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub address: Option<AddressDetails>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddressDetails {
    pub street: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            phone: user.phone,
            created_at: user.created_at,
            address: if user.address_street.is_some() || user.address_city.is_some() || user.address_postal_code.is_some() || user.address_country.is_some() {
                Some(AddressDetails {
                    street: user.address_street,
                    city: user.address_city,
                    postal_code: user.address_postal_code,
                    country: user.address_country,
                })
            } else {
                None
            },
        }
    }
}

// Registration request validation
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 100, message = "Name must be between 3 and 100 characters"))]
    pub name: String,

    #[validate(email(message = "Email must be valid"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "Passwords must match"))]
    pub password_confirmation: String,

    #[serde(deserialize_with = "deserialize_role")]
    pub role: UserRole,
    pub phone: Option<String>,
}

// Login request validation
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Email must be valid"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

// Update user profile request validation
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 3, max = 100, message = "Name must be between 3 and 100 characters"))]
    pub name: String,

    pub phone: Option<String>,
}

// New struct for handling address updates
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct UserAddressRequest {
    pub street: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

// Custom deserializer for UserRole
fn deserialize_role<'de, D>(deserializer: D) -> Result<UserRole, D::Error>
where
    D: Deserializer<'de>,
{
    struct RoleVisitor;

    impl<'de> Visitor<'de> for RoleVisitor {
        type Value = UserRole;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string representing a user role")
        }

        fn visit_str<E>(self, value: &str) -> Result<UserRole, E>
        where
            E: de::Error,
        {
            UserRole::from_str(value).map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_str(RoleVisitor)
}

// Change password request validation
#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, message = "Current password is required"))]
    pub current_password: String,

    #[validate(length(min = 8, message = "New password must be at least 8 characters"))]
    pub new_password: String,

    #[validate(must_match(other = "new_password", message = "Passwords must match"))]
    pub new_password_confirmation: String,
}

// Password reset request validation
#[derive(Debug, Deserialize, Validate)]
pub struct PasswordResetRequest {
    #[validate(email(message = "Email must be valid"))]
    pub email: String,
}

// Password reset confirmation validation
#[derive(Debug, Deserialize, Validate)]
pub struct PasswordResetConfirmRequest {
    pub token: String,

    #[validate(length(min = 8, message = "New password must be at least 8 characters"))]
    pub new_password: String,

    #[validate(must_match(other = "new_password", message = "Passwords must match"))]
    pub new_password_confirmation: String,
}

impl From<user::Model> for User {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            email: model.email,
            password_hash: model.password_hash,
            role: model.role,
            phone: model.phone,
            created_at: model.created_at,
            updated_at: model.updated_at,
            address_street: model.address_street,
            address_city: model.address_city,
            address_postal_code: model.address_postal_code,
            address_country: model.address_country,
        }
    }
}
