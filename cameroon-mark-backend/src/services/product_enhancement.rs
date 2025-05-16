use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ConnectionTrait,
    QuerySelect, Statement, TransactionTrait, ActiveValue,
};
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

use crate::{
    entities::{product, product_variant, variant_option, product_inventory, inventory_log},
    errors::{AppError, Result},
    models::product_enhancement::{
        BulkProductUploadRequest, ProductUploadItem, ProductVariant, VariantOption,
        ProductInventory, ImportExportStatus, InventoryLog, BulkInventoryUpdateRequest, InventoryUpdateItem
    },
    models::product::Product,
    utils::validation,
};

// Bulk product upload
pub async fn bulk_upload_products(
    db: &DatabaseConnection,
    user_id: Uuid,
    payload: BulkProductUploadRequest,
) -> Result<ImportExportStatus> {
    // Start a transaction
    let txn = db.begin().await?;

    // Create import status record
    let import_status = ImportExportStatus {
        id: Uuid::new_v4(),
        user_id,
        operation_type: "import".to_string(),
        status: "processing".to_string(),
        total_items: payload.products.len() as i32,
        processed_items: 0,
        successful_items: 0,
        failed_items: 0,
        file_url: None,
        error_log: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // TODO: Insert import status into database

    let mut successful_items = 0;
    let mut failed_items = 0;
    let mut error_log = String::new();

    // Process each product
    for (index, product_item) in payload.products.iter().enumerate() {
        // Validate product data
        if let Err(validation_error) = validation::validate(product_item) {
            failed_items += 1;
            error_log.push_str(&format!("Product {}: {}\n", index + 1, validation_error));
            continue;
        }

        // Create product
        match create_product(&txn, user_id, product_item).await {
            Ok(_) => successful_items += 1,
            Err(e) => {
                failed_items += 1;
                error_log.push_str(&format!("Product {}: {}\n", index + 1, e));
            }
        }
    }

    // Update import status
    let updated_status = ImportExportStatus {
        id: import_status.id,
        user_id,
        operation_type: "import".to_string(),
        status: if failed_items > 0 {
            "completed_with_errors"
        } else {
            "completed"
        }
        .to_string(),
        total_items: payload.products.len() as i32,
        processed_items: (successful_items + failed_items) as i32,
        successful_items: successful_items as i32,
        failed_items: failed_items as i32,
        file_url: None,
        error_log: if !error_log.is_empty() {
            Some(error_log)
        } else {
            None
        },
        created_at: import_status.created_at,
        updated_at: Utc::now(),
    };

    // TODO: Update import status in database

    // Commit the transaction
    txn.commit().await?;

    Ok(updated_status)
}

// Helper function to create a single product with variants
async fn create_product(
    db: &impl ConnectionTrait,
    user_id: Uuid,
    product_item: &ProductUploadItem,
) -> Result<Product> {
    // Create product active model
    let product_id = Uuid::new_v4();
    let now = Utc::now();

    let product_model = product::ActiveModel {
        id: Set(product_id),
        user_id: Set(user_id),
        category_id: Set(product_item.category_id),
        name: Set(product_item.name.clone()),
        description: Set(product_item.description.clone()),
        price: Set(product_item.price),
        main_image: Set(product_item.main_image.clone()),
        additional_images: Set(Some(product_item.additional_images.clone())),
        quantity_available: Set(product_item.quantity_available),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let product_result = product_model.insert(db).await?;
    
    // Create product variants if provided
    if let Some(variants) = &product_item.variants {
        for variant in variants {
            create_product_variant(db, product_id, variant).await?;
        }
    }

    // Initialize inventory
    initialize_product_inventory(db, product_id, product_item.quantity_available).await?;

    Ok(Product {
        id: product_result.id,
        name: product_result.name,
        description: product_result.description,
        price: product_result.price,
        main_image: product_result.main_image,
        additional_images: product_result.additional_images.unwrap_or_default(),
        category_id: product_result.category_id,
        user_id: product_result.user_id,
        quantity_available: product_result.quantity_available,
        created_at: product_result.created_at,
        updated_at: product_result.updated_at,
    })
}

// Create product variant and options
async fn create_product_variant(
    db: &impl ConnectionTrait,
    product_id: Uuid,
    variant: &ProductVariant,
) -> Result<()> {
    let variant_id = Uuid::new_v4();
    let now = Utc::now();

    // Create variant
    let variant_model = product_variant::ActiveModel {
        id: Set(variant_id),
        product_id: Set(product_id),
        name: Set(variant.name.clone()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    variant_model.insert(db).await?;

    // Create variant options
    for option in &variant.options {
        let option_model = variant_option::ActiveModel {
            id: Set(Uuid::new_v4()),
            variant_id: Set(variant_id),
            value: Set(option.value.clone()),
            price_adjustment: Set(option.price_adjustment),
            quantity: Set(option.quantity),
            sku: Set(option.sku.clone()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        option_model.insert(db).await?;
    }

    Ok(())
}

// Initialize product inventory
async fn initialize_product_inventory(
    db: &impl ConnectionTrait,
    product_id: Uuid,
    initial_quantity: i32,
) -> Result<()> {
    let now = Utc::now();

    // Create inventory record
    let inventory_model = product_inventory::ActiveModel {
        product_id: Set(product_id),
        total_quantity: Set(initial_quantity),
        available_quantity: Set(initial_quantity),
        reserved_quantity: Set(0),
        low_stock_threshold: Set(5), // Default threshold
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    inventory_model.insert(db).await?;

    // Log the inventory initialization
    let log_model = inventory_log::ActiveModel {
        id: Set(Uuid::new_v4()),
        product_id: Set(product_id),
        variant_option_id: Set(None),
        previous_quantity: Set(0),
        new_quantity: Set(initial_quantity),
        change_type: Set("initialize".to_string()),
        reason: Set("Initial product creation".to_string()),
        reference_id: Set(None),
        created_at: Set(now),
        user_id: Set(None),
        ..Default::default()
    };

    log_model.insert(db).await?;

    Ok(())
}

// Get product inventory
pub async fn get_product_inventory(
    db: &DatabaseConnection,
    product_id: Uuid,
) -> Result<ProductInventory> {
    let inventory = product_inventory::Entity::find_by_id(product_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Product inventory not found"))?;

    // Get variant inventory if applicable
    let variant_inventory_query = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        SELECT vo.id, vo.value, vo.quantity
        FROM variant_options vo
        JOIN product_variants pv ON vo.variant_id = pv.id
        WHERE pv.product_id = $1
        "#,
        vec![product_id.into()],
    );

    let rows = db.query_all(variant_inventory_query)
        .await
        .map_err(|e| AppError::internal(format!("Database error: {}", e)))?;

    let mut variant_inventory = HashMap::new();

    for row in rows {
        let option_id: Uuid = row.try_get("", "id")?;
        let option_value: String = row.try_get("", "value")?;
        let quantity: i32 = row.try_get("", "quantity")?;

        variant_inventory.insert(option_value, quantity);
    }

    let result = ProductInventory {
        product_id: inventory.product_id,
        total_quantity: inventory.total_quantity,
        available_quantity: inventory.available_quantity,
        reserved_quantity: inventory.reserved_quantity,
        low_stock_threshold: inventory.low_stock_threshold,
        variant_inventory: if variant_inventory.is_empty() {
            None
        } else {
            Some(variant_inventory)
        },
        updated_at: inventory.updated_at,
    };

    Ok(result)
}

// Update product inventory
pub async fn update_product_inventory(
    db: &DatabaseConnection,
    user_id: Uuid,
    payload: InventoryUpdateItem,
) -> Result<ProductInventory> {
    let txn = db.begin().await?;

    // Get current inventory
    let inventory = product_inventory::Entity::find_by_id(payload.product_id)
        .one(&txn)
        .await?
        .ok_or_else(|| AppError::not_found("Product inventory not found"))?;

    // Calculate new quantity based on action
    let previous_quantity = inventory.available_quantity;
    let new_quantity = match payload.action.to_lowercase().as_str() {
        "set" => payload.quantity,
        "add" => previous_quantity + payload.quantity,
        "subtract" => {
            if previous_quantity < payload.quantity {
                return Err(AppError::bad_request("Insufficient inventory"));
            }
            previous_quantity - payload.quantity
        }
        _ => return Err(AppError::bad_request("Invalid action")),
    };

    // Update inventory
    let mut inventory_model: product_inventory::ActiveModel = inventory.clone().into();
    inventory_model.available_quantity = Set(new_quantity);
    inventory_model.total_quantity = Set(new_quantity + inventory.reserved_quantity);
    inventory_model.updated_at = Set(Utc::now());

    inventory_model.update(&txn).await?;

    // Update variant inventory if specified
    if let Some(variant_option_id) = payload.variant_option_id {
        // Get current variant option
        let variant_option = variant_option::Entity::find_by_id(variant_option_id)
            .one(&txn)
            .await?
            .ok_or_else(|| AppError::not_found("Variant option not found"))?;

        let previous_variant_quantity = variant_option.quantity;
        
        // Calculate new quantity
        let new_variant_quantity = match payload.action.to_lowercase().as_str() {
            "set" => payload.quantity,
            "add" => previous_variant_quantity + payload.quantity,
            "subtract" => {
                if previous_variant_quantity < payload.quantity {
                    return Err(AppError::bad_request("Insufficient variant inventory"));
                }
                previous_variant_quantity - payload.quantity
            }
            _ => return Err(AppError::bad_request("Invalid action")),
        };

        // Update variant option
        let mut variant_option_model: variant_option::ActiveModel = variant_option.into();
        variant_option_model.quantity = Set(new_variant_quantity);
        variant_option_model.updated_at = Set(Utc::now());

        variant_option_model.update(&txn).await?;
    }

    // Log inventory change
    let log_model = inventory_log::ActiveModel {
        id: Set(Uuid::new_v4()),
        product_id: Set(payload.product_id),
        variant_option_id: Set(payload.variant_option_id),
        previous_quantity: Set(previous_quantity),
        new_quantity: Set(new_quantity),
        change_type: Set(payload.action.to_lowercase()),
        reason: Set(payload.reason),
        reference_id: Set(None),
        created_at: Set(Utc::now()),
        user_id: Set(Some(user_id)),
        ..Default::default()
    };

    log_model.insert(&txn).await?;

    // Commit transaction
    txn.commit().await?;

    // Return updated inventory
    get_product_inventory(db, payload.product_id).await
}

// Bulk update inventory
pub async fn bulk_update_inventory(
    db: &DatabaseConnection,
    user_id: Uuid,
    payload: BulkInventoryUpdateRequest,
) -> Result<ImportExportStatus> {
    let txn = db.begin().await?;

    let import_status = ImportExportStatus {
        id: Uuid::new_v4(),
        user_id,
        operation_type: "inventory_update".to_string(),
        status: "processing".to_string(),
        total_items: payload.updates.len() as i32,
        processed_items: 0,
        successful_items: 0,
        failed_items: 0,
        file_url: None,
        error_log: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // TODO: Insert import status into database

    let mut successful_items = 0;
    let mut failed_items = 0;
    let mut error_log = String::new();

    // Process each inventory update
    for (index, update_item) in payload.updates.iter().enumerate() {
        // Update inventory
        match update_product_inventory(&txn, user_id, update_item.clone()).await {
            Ok(_) => successful_items += 1,
            Err(e) => {
                failed_items += 1;
                error_log.push_str(&format!("Update {}: {}\n", index + 1, e));
            }
        }
    }

    // Update status
    let updated_status = ImportExportStatus {
        id: import_status.id,
        user_id,
        operation_type: "inventory_update".to_string(),
        status: if failed_items > 0 {
            "completed_with_errors"
        } else {
            "completed"
        }
        .to_string(),
        total_items: payload.updates.len() as i32,
        processed_items: (successful_items + failed_items) as i32,
        successful_items: successful_items as i32,
        failed_items: failed_items as i32,
        file_url: None,
        error_log: if !error_log.is_empty() {
            Some(error_log)
        } else {
            None
        },
        created_at: import_status.created_at,
        updated_at: Utc::now(),
    };

    // TODO: Update status in database

    // Commit transaction
    txn.commit().await?;

    Ok(updated_status)
}

// Get inventory history for a product
pub async fn get_inventory_history(
    db: &DatabaseConnection,
    product_id: Uuid,
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<Vec<InventoryLog>> {
    let mut query = inventory_log::Entity::find()
        .filter(inventory_log::Column::ProductId.eq(product_id))
        .order_by_desc(inventory_log::Column::CreatedAt);

    if let Some(limit_val) = limit {
        query = query.limit(limit_val);
    }

    if let Some(offset_val) = offset {
        query = query.offset(offset_val);
    }

    let logs = query.all(db).await?;

    let result = logs.into_iter()
        .map(|log| InventoryLog {
            id: log.id,
            product_id: log.product_id,
            variant_option_id: log.variant_option_id,
            previous_quantity: log.previous_quantity,
            new_quantity: log.new_quantity,
            change_type: log.change_type,
            reason: log.reason,
            reference_id: log.reference_id,
            created_at: log.created_at,
            user_id: log.user_id,
        })
        .collect();

    Ok(result)
}
