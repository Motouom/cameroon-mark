use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use uuid::Uuid;

use crate::errors::{AppError, Result};
use crate::models::category::Category;
use crate::entities::{category, category::Entity as CategoryEntity};

// Get all categories
pub async fn get_categories(db: &DatabaseConnection) -> Result<Vec<Category>> {
    // Get all categories from the database using Sea-ORM
    let categories = CategoryEntity::find()
        .order_by_asc(category::Column::Name)
        .all(db)
        .await
        .map_err(AppError::from)?;
    
    // Convert entity models to application models
    let categories = categories
        .into_iter()
        .map(|model| {
            // Generate the slug first to avoid ownership issues
            let slug = model.name.to_lowercase().replace(" ", "-");
            
            Category {
                id: model.id,
                name: model.name,
                description: model.description,
                image_url: None, // Not in entity model
                slug,
                created_at: model.created_at,
                updated_at: model.updated_at,
            }
        })
        .collect();
    
    Ok(categories)
}
