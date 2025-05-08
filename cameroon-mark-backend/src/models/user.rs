use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use uuid::Uuid;
use validator::{Validate, ValidationError};
use sqlx::types::BigDecimal;
use std::str::FromStr;
use serde::de::{self, Deserializer, Visitor};
use sea_orm::{TryGetable, TryGetError};
use sea_orm::sea_query::{ValueType, ValueTypeErr};
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
    pub location: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Admin,
    Seller,
    Buyer,
}

impl FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(UserRole::Admin),
            "seller" => Ok(UserRole::Seller),
            "buyer" => Ok(UserRole::Buyer),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

impl ValueType for UserRole {
    fn try_from(v: sea_orm::Value) -> Result<Self, ValueTypeErr> {
        match v {
            sea_orm::Value::String(Some(s)) => match s.as_str() {
                "admin" => Ok(UserRole::Admin),
                "seller" => Ok(UserRole::Seller),
                "buyer" => Ok(UserRole::Buyer),
                _ => Err(ValueTypeErr),
            },
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "user_role".to_string()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::String
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        sea_orm::sea_query::ColumnType::String(sea_orm::sea_query::StringLen::None)
    }
}

impl From<UserRole> for sea_orm::Value {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::Admin => sea_orm::Value::String(Some(Box::new("admin".to_string()))),
            UserRole::Seller => sea_orm::Value::String(Some(Box::new("seller".to_string()))),
            UserRole::Buyer => sea_orm::Value::String(Some(Box::new("buyer".to_string()))),
        }
    }
}

impl TryGetable for UserRole {
    fn try_get_by<I: sea_orm::ColIdx>(res: &sea_orm::QueryResult, idx: I) -> Result<Self, sea_orm::TryGetError> {
        let value: String = res.try_get_by(idx)?;
        match value.as_str() {
            "admin" => Ok(UserRole::Admin),
            "seller" => Ok(UserRole::Seller),
            "buyer" => Ok(UserRole::Buyer),
            _ => Err(sea_orm::TryGetError::DbErr(sea_orm::DbErr::Custom(format!("Invalid user role: {}", value)))),
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
    pub location: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            location: user.location,
            phone: user.phone,
            created_at: user.created_at,
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
    pub location: Option<String>,
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
    
    pub location: Option<String>,
    pub phone: Option<String>,
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
            location: model.location,
            phone: model.phone,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
