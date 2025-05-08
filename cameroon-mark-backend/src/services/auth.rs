use chrono::{DateTime, Utc};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveValue, ConnectionTrait};
use sea_orm::ActiveModelTrait;
use uuid::Uuid;

use crate::errors::{AppError, Result};
use crate::models::user::{User, UserRole, RegisterRequest, LoginRequest};
use crate::utils::{jwt, password};
use crate::entities::{user, user::Entity as UserEntity};

// Register a new user
pub async fn register(db: &DatabaseConnection, payload: RegisterRequest) -> Result<(User, String)> {
    // Check if the passwords match
    if payload.password != payload.password_confirmation {
        return Err(AppError::validation("Passwords do not match"));
    }
    
    // Check if the email is already taken using a direct SQL query
    let email_exists = sea_orm::Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as exists",
        vec![payload.email.clone().into()],
    );
    
    let row = db.query_one(email_exists)
        .await
        .map_err(|e| AppError::internal(format!("Database error checking email: {}", e)))?
        .ok_or_else(|| AppError::internal("No result from email check query"))?;
    
    let exists: bool = row.try_get::<bool>("exists", "")
        .map_err(|e| AppError::internal(format!("Error parsing email check result: {}", e)))?;
    
    if exists {
        return Err(AppError::validation("Email is already taken"));
    }
    
    // Hash the password
    let password_hash = password::hash_password(&payload.password)?;
    
    // Create a new user active model
    let user_id = Uuid::new_v4();
    let now = Utc::now();
    
    // Use a prepared statement with parameter binding for the role
    let insert_stmt = sea_orm::Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        WITH role_value AS (
            SELECT $7::text::user_role as role
        )
        INSERT INTO users (id, email, password_hash, name, location, phone, role, created_at, updated_at) 
        SELECT $1, $2, $3, $4, $5, $6, role, $8, $9 FROM role_value
        RETURNING id, email, password_hash, name, location, phone, role::text, created_at, updated_at
        "#,
        vec![
            user_id.into(),
            payload.email.clone().into(),
            password_hash.into(),
            payload.name.clone().into(),
            sea_orm::Value::String(None),  // location is NULL
            match &payload.phone {
                Some(phone) => sea_orm::Value::String(Some(Box::new(phone.clone()))),
                None => sea_orm::Value::String(None),
            },
            match payload.role {
                UserRole::Admin => "admin".into(),
                UserRole::Seller => "seller".into(),
                UserRole::Buyer => "buyer".into(),
            },
            now.into(),
            now.into(),
        ],
    );
    
    let row = db.query_one(insert_stmt)
        .await
        .map_err(|e| AppError::internal(format!("Database error inserting user: {}", e)))?
        .ok_or_else(|| AppError::internal("No result from user insert query"))?;
    
    // Manually construct the User from the row
    let user = User {
        id: row.try_get::<Uuid>("id", "")?,
        email: row.try_get::<String>("email", "")?,
        password_hash: row.try_get::<String>("password_hash", "")?,
        name: row.try_get::<String>("name", "")?,
        location: row.try_get::<Option<String>>("location", "").ok().flatten(),
        phone: row.try_get::<Option<String>>("phone", "").ok().flatten(),
        role: match row.try_get::<String>("role", "")?.as_str() {
            "admin" => UserRole::Admin,
            "seller" => UserRole::Seller,
            _ => UserRole::Buyer,
        },
        created_at: row.try_get::<DateTime<Utc>>("created_at", "")?,
        updated_at: row.try_get::<DateTime<Utc>>("updated_at", "")?,
    };
    
    // Generate JWT token
    let token = jwt::generate_token(&user)?;

    Ok((user, token))
}

// Login a user
pub async fn login(db: &DatabaseConnection, payload: LoginRequest) -> Result<(User, String)> {
    // Use a raw SQL query to find the user by email and avoid role type issues
    let login_query = sea_orm::Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        SELECT 
            id, 
            email, 
            password_hash, 
            name, 
            location, 
            phone, 
            role::text as role_text, 
            created_at, 
            updated_at 
        FROM users 
        WHERE email = $1
        "#,
        vec![payload.email.clone().into()],
    );
    
    let row = db.query_one(login_query)
        .await
        .map_err(|e| AppError::internal(format!("Database error during login: {}", e)))?
        .ok_or_else(|| AppError::auth("Invalid email or password"))?;
    
    // Get the password hash to verify
    let password_hash = row.try_get::<String>("password_hash", "")?;
    
    // Verify password
    if !password::verify_password(&payload.password, &password_hash)? {
        return Err(AppError::auth("Invalid email or password"));
    }
    
    // Construct the user from the query result
    let user = User {
        id: row.try_get::<Uuid>("id", "")?,
        email: row.try_get::<String>("email", "")?,
        password_hash,
        name: row.try_get::<String>("name", "")?,
        location: row.try_get::<Option<String>>("location", "").ok().flatten(),
        phone: row.try_get::<Option<String>>("phone", "").ok().flatten(),
        role: match row.try_get::<String>("role_text", "")?.as_str() {
            "admin" => UserRole::Admin,
            "seller" => UserRole::Seller,
            _ => UserRole::Buyer,
        },
        created_at: row.try_get::<DateTime<Utc>>("created_at", "")?,
        updated_at: row.try_get::<DateTime<Utc>>("updated_at", "")?,
    };

    // Generate JWT token
    let token = jwt::generate_token(&user)?;

    Ok((user, token))
}

// Request a password reset
pub async fn request_password_reset(db: &DatabaseConnection, email: &str) -> Result<()> {
    // Use a simple query to check if the email exists without fetching the entire user record
    let email_exists = sea_orm::Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as exists",
        vec![email.into()],
    );
    
    let row = db.query_one(email_exists)
        .await
        .map_err(|e| AppError::internal(format!("Database error checking email: {}", e)))?
        .ok_or_else(|| AppError::internal("No result from email check query"))?;
    
    let exists: bool = row.try_get::<bool>("exists", "")
        .map_err(|e| AppError::internal(format!("Error parsing email check result: {}", e)))?;
    
    // Even if the user doesn't exist, we return success to prevent email enumeration
    if !exists {
        return Ok(());
    }
    
    // In a real application, we would generate a reset token, store it in the database,
    // and send an email to the user with a link to reset their password.
    // For this example, we'll just return success.
    
    Ok(())
}
