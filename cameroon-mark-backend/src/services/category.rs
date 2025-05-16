use sea_orm::{
    DatabaseConnection, EntityTrait, QueryFilter,
    Condition, Set, ActiveValue, IntoActiveValue,
    ActiveModelTrait, ColumnTrait, ModelTrait, PaginatorTrait,
};
use chrono::Utc;
use uuid::Uuid;

use crate::errors::{AppError, Result};
use crate::entities::category::{self, Entity as Category, Model as CategoryModel};
use crate::models::category::{CreateCategoryRequest, UpdateCategoryRequest};

// Get all categories
pub async fn get_categories(db: &DatabaseConnection) -> Result<Vec<CategoryModel>> {
    let categories = Category::find()
        .all(db)
        .await
        .map_err(|e| AppError::SeaOrmDatabase(e))?;
    
    Ok(categories)
}

pub async fn get_category(db: &DatabaseConnection, id: Uuid) -> Result<CategoryModel> {
    let category = Category::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| AppError::SeaOrmDatabase(e))?
        .ok_or_else(|| AppError::not_found("Category not found"))?;
    
    Ok(category)
}

pub async fn create_category(
    db: &DatabaseConnection,
    payload: CreateCategoryRequest,
) -> Result<CategoryModel> {
    // Check if category with same slug exists
    let existing = Category::find()
        .filter(category::Column::Slug.eq(&payload.slug))
        .one(db)
        .await
        .map_err(|e| AppError::SeaOrmDatabase(e))?;
    
    if existing.is_some() {
        return Err(AppError::bad_request("Category with this slug already exists"));
    }
    
    let now = Utc::now();
    let category = category::ActiveModel {
        id: Uuid::new_v4().into_active_value(),
        name: payload.name.into_active_value(),
        description: payload.description.into_active_value(),
        image_url: payload.image_url.into_active_value(),
        slug: payload.slug.into_active_value(),
        created_at: now.into_active_value(),
        updated_at: now.into_active_value(),
    };
    
    let category = category
        .insert(db)
        .await
        .map_err(|e| AppError::SeaOrmDatabase(e))?;
    
    Ok(category)
}

pub async fn update_category(
    db: &DatabaseConnection,
    id: Uuid,
    payload: UpdateCategoryRequest,
) -> Result<CategoryModel> {
    let category = get_category(db, id).await?;
    
    // If updating slug, check it doesn't conflict
    if let Some(slug) = &payload.slug {
        let existing = Category::find()
            .filter(category::Column::Slug.eq(slug))
            .filter(category::Column::Id.ne(id))
            .one(db)
            .await
            .map_err(|e| AppError::SeaOrmDatabase(e))?;
        
        if existing.is_some() {
            return Err(AppError::bad_request("Category with this slug already exists"));
        }
    }
    
    let mut category: category::ActiveModel = category.into();
    
    if let Some(name) = payload.name {
        category.name = Set(name);
    }
    
    if let Some(description) = payload.description {
        category.description = Set(Some(description));
    }
    
    if let Some(image_url) = payload.image_url {
        category.image_url = Set(Some(image_url));
    }
    
    if let Some(slug) = payload.slug {
        category.slug = Set(slug);
    }
    
    category.updated_at = Set(Utc::now());
    
    let category = category
        .update(db)
        .await
        .map_err(|e| AppError::SeaOrmDatabase(e))?;
    
    Ok(category)
}

pub async fn delete_category(db: &DatabaseConnection, id: Uuid) -> Result<()> {
    let category = get_category(db, id).await?;
    
    // Check if category has any products
    let products_count = category
        .find_related(crate::entities::product::Entity)
        .count(db)
        .await
        .map_err(|e| AppError::SeaOrmDatabase(e))?;
    
    if products_count > 0 {
        return Err(AppError::bad_request("Cannot delete category with existing products"));
    }
    
    Category::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| AppError::SeaOrmDatabase(e))?;
    
    Ok(())
}
