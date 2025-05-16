use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    RelationTrait, QuerySelect, JoinType,
};
use uuid::Uuid;

use crate::{
    entities::{saved_item, product},
    errors::{AppError, Result},
    models::saved_item::{SavedItem, SavedItemWithProduct, AddSavedItemRequest},
    models::product::{ProductSummary, Product},
};

// Get all saved items for a user
pub async fn get_saved_items(db: &DatabaseConnection, user_id: Uuid) -> Result<Vec<SavedItemWithProduct>> {
    let saved_items = saved_item::Entity::find()
        .filter(saved_item::Column::UserId.eq(user_id))
        .find_with_related(product::Entity)
        .all(db)
        .await?;

    let result = saved_items
        .into_iter()
        .filter_map(|(saved_item, products)| {
            let product = products.first()?;
            Some(SavedItemWithProduct {
                id: saved_item.id,
                product: ProductSummary::from(Product::from(product.clone())),
                created_at: saved_item.created_at,
            })
        })
        .collect();

    Ok(result)
}

// Add a product to saved items
pub async fn add_saved_item(db: &DatabaseConnection, user_id: Uuid, payload: AddSavedItemRequest) -> Result<SavedItem> {
    // Check if product exists
    let product = product::Entity::find_by_id(payload.product_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Product not found"))?;

    // Check if already saved
    let existing = saved_item::Entity::find()
        .filter(saved_item::Column::UserId.eq(user_id))
        .filter(saved_item::Column::ProductId.eq(payload.product_id))
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(AppError::bad_request("Product already saved"));
    }

    // Create new saved item
    let saved_item = saved_item::ActiveModel {
        user_id: Set(user_id),
        product_id: Set(payload.product_id),
        ..Default::default()
    };

    let saved_item = saved_item.insert(db).await?;
    Ok(SavedItem::from(saved_item))
}

// Remove a product from saved items
pub async fn remove_saved_item(db: &DatabaseConnection, user_id: Uuid, product_id: Uuid) -> Result<()> {
    let result = saved_item::Entity::delete_many()
        .filter(saved_item::Column::UserId.eq(user_id))
        .filter(saved_item::Column::ProductId.eq(product_id))
        .exec(db)
        .await?;

    if result.rows_affected == 0 {
        return Err(AppError::not_found("Saved item not found"));
    }

    Ok(())
}