use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ConnectionTrait,
};
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};
use crate::{
    errors::{AppError, Result},
    models::user::{User, UserProfile, UpdateProfileRequest, ChangePasswordRequest, RegisterRequest, LoginRequest, UserRole, UserAddressRequest, AddressDetails},
    entities::user,
};

// Get a user by ID
pub async fn get_user_by_id(db: &DatabaseConnection, user_id: Uuid) -> Result<User> {
    let query = sea_orm::Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        SELECT 
            id, 
            email, 
            password_hash, 
            name, 
            phone, 
            role::text as role_text, 
            created_at, 
            updated_at,
            address_street,
            address_city,
            address_postal_code,
            address_country
        FROM users 
        WHERE id = $1
        "#,
        vec![user_id.into()],
    );
    
    let row = db.query_one(query)
        .await
        .map_err(|e| AppError::internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::not_found("User not found"))?;
    
    // Construct the user from the query result
    let user = User {
        id: row.try_get::<Uuid>("", "id")?,
        email: row.try_get::<String>("", "email")?,
        password_hash: row.try_get::<String>("", "password_hash")?,
        name: row.try_get::<String>("", "name")?,
        phone: row.try_get::<Option<String>>("", "phone").ok().flatten(),
        role: match row.try_get::<String>("", "role_text")?.as_str() {
            "seller" => UserRole::Seller,
            _ => UserRole::Customer,
        },
        created_at: row.try_get::<DateTime<Utc>>("", "created_at")?,
        updated_at: row.try_get::<DateTime<Utc>>("", "updated_at")?,
        address_street: row.try_get("", "address_street").ok().flatten(),
        address_city: row.try_get("", "address_city").ok().flatten(),
        address_postal_code: row.try_get("", "address_postal_code").ok().flatten(),
        address_country: row.try_get("", "address_country").ok().flatten(),
    };

    Ok(user)
}

// Get a user by email
pub async fn get_user_by_email(db: &DatabaseConnection, email: &str) -> Result<User> {
    let query = sea_orm::Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        SELECT 
            id, 
            email, 
            password_hash, 
            name, 
            phone, 
            role::text as role_text, 
            created_at, 
            updated_at,
            address_street,
            address_city,
            address_postal_code,
            address_country
        FROM users 
        WHERE email = $1
        "#,
        vec![email.into()],
    );
    
    let row = db.query_one(query)
        .await
        .map_err(|e| AppError::internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::not_found("User not found"))?;
    
    // Construct the user from the query result
    let user = User {
        id: row.try_get::<Uuid>("", "id")?,
        email: row.try_get::<String>("", "email")?,
        password_hash: row.try_get::<String>("", "password_hash")?,
        name: row.try_get::<String>("", "name")?,
        phone: row.try_get::<Option<String>>("", "phone").ok().flatten(),
        role: match row.try_get::<String>("", "role_text")?.as_str() {
            "seller" => UserRole::Seller,
            _ => UserRole::Customer,
        },
        created_at: row.try_get::<DateTime<Utc>>("", "created_at")?,
        updated_at: row.try_get::<DateTime<Utc>>("", "updated_at")?,
        address_street: row.try_get("", "address_street").ok().flatten(),
        address_city: row.try_get("", "address_city").ok().flatten(),
        address_postal_code: row.try_get("", "address_postal_code").ok().flatten(),
        address_country: row.try_get("", "address_country").ok().flatten(),
    };

    Ok(user)
}

// Create a new user
pub async fn create_user(db: &DatabaseConnection, payload: RegisterRequest) -> Result<User> {
    let password_hash = hash(payload.password.as_bytes(), DEFAULT_COST)?;
    
    let user_active_model = user::ActiveModel {
        email: Set(payload.email),
        password_hash: Set(password_hash),
        name: Set(payload.name),
        phone: Set(payload.phone),
        role: Set(payload.role),
        address_street: Set(None),
        address_city: Set(None),
        address_postal_code: Set(None),
        address_country: Set(None),
        ..Default::default()
    };

    let user_model = user_active_model.insert(db).await?;
    Ok(User::from(user_model))
}

// Update a user's profile (name, phone - address is separate)
pub async fn update_user(db: &DatabaseConnection, user_id: Uuid, payload: UpdateProfileRequest) -> Result<User> {
    let user_model_db = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("User not found"))?;

    let mut user_active_model: user::ActiveModel = user_model_db.into();
    user_active_model.name = Set(payload.name);
    
    if let Some(phone) = payload.phone {
        user_active_model.phone = Set(Some(phone));
    } else {
        user_active_model.phone = Set(None);
    }

    let updated_user_model = user_active_model.update(db).await?;
    Ok(User::from(updated_user_model))
}

// Get user profile (includes address if available)
pub async fn get_profile(db: &DatabaseConnection, user_id: Uuid) -> Result<UserProfile> {
    let user_model_db = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("User not found"))?;

    Ok(UserProfile::from(User::from(user_model_db)))
}

// Update user's address
pub async fn update_user_address(db: &DatabaseConnection, user_id: Uuid, payload: UserAddressRequest) -> Result<AddressDetails> {
    let user_model_db = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("User not found"))?;

    let mut user_active_model: user::ActiveModel = user_model_db.into();

    user_active_model.address_street = Set(payload.street);
    user_active_model.address_city = Set(payload.city);
    user_active_model.address_postal_code = Set(payload.postal_code);
    user_active_model.address_country = Set(payload.country);
    
    let updated_user_model = user_active_model.update(db).await?;

    Ok(AddressDetails {
        street: updated_user_model.address_street,
        city: updated_user_model.address_city,
        postal_code: updated_user_model.address_postal_code,
        country: updated_user_model.address_country,
    })
}

pub async fn change_password(db: &DatabaseConnection, user_id: Uuid, payload: ChangePasswordRequest) -> Result<()> {
    let user_model_db = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("User not found"))?;

    if !verify(&payload.current_password, &user_model_db.password_hash)? {
        return Err(AppError::bad_request("Current password is incorrect"));
    }

    let password_hash = hash(payload.new_password.as_bytes(), DEFAULT_COST)?;

    let mut user_active_model: user::ActiveModel = user_model_db.into();
    user_active_model.password_hash = Set(password_hash);
    user_active_model.update(db).await?;

    Ok(())
}
