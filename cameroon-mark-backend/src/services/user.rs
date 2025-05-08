use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::{
    errors::{AppError, Result},
    models::user::{User, UserProfile, UpdateProfileRequest, ChangePasswordRequest, RegisterRequest, LoginRequest},
    entities::user,
};

// Get a user by ID
pub async fn get_user_by_id(db: &DatabaseConnection, user_id: Uuid) -> Result<User> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("User not found"))?;

    Ok(User::from(user))
}

// Get a user by email
pub async fn get_user_by_email(db: &DatabaseConnection, email: &str) -> Result<User> {
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("User not found"))?;

    Ok(User::from(user))
}

// Create a new user
pub async fn create_user(db: &DatabaseConnection, payload: RegisterRequest) -> Result<User> {
    let password_hash = hash(payload.password.as_bytes(), DEFAULT_COST)?;
    
    let user = user::ActiveModel {
        email: Set(payload.email),
        password_hash: Set(password_hash),
        name: Set(payload.name),
        location: Set(None),
        role: Set(payload.role),
        phone: Set(payload.phone),
        ..Default::default()
    };

    let user = user.insert(db).await?;
    Ok(User::from(user))
}

// Update a user's profile
pub async fn update_user(db: &DatabaseConnection, user_id: Uuid, payload: UpdateProfileRequest) -> Result<User> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("User not found"))?;

    let mut user: user::ActiveModel = user.into();
    user.name = Set(payload.name);
    if let Some(phone) = payload.phone {
        user.phone = Set(Some(phone));
    }

    let user = user.update(db).await?;
    Ok(User::from(user))
}

pub async fn get_profile(db: &DatabaseConnection, user_id: Uuid) -> Result<User> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("User not found"))?;

    Ok(User::from(user))
}

pub async fn update_profile(
    db: &DatabaseConnection,
    user_id: Uuid,
    payload: UpdateProfileRequest,
) -> Result<User> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("User not found"))?;

    let mut user: user::ActiveModel = user.into();
    user.name = Set(payload.name);
    if let Some(phone) = payload.phone {
        user.phone = Set(Some(phone));
    }

    let user = user.update(db).await?;
    Ok(User::from(user))
}

pub async fn change_password(db: &DatabaseConnection, user_id: Uuid, payload: ChangePasswordRequest) -> Result<()> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("User not found"))?;

    if !verify(&payload.current_password, &user.password_hash)? {
        return Err(AppError::bad_request("Current password is incorrect"));
    }

    let password_hash = hash(payload.new_password.as_bytes(), DEFAULT_COST)?;

    let mut user: user::ActiveModel = user.into();
    user.password_hash = Set(password_hash);
    user.update(db).await?;

    Ok(())
}
