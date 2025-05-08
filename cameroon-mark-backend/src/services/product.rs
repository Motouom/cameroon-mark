use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionTrait,
};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use crate::{
    errors::{AppError, Result},
    models::product::{Product, ProductResponse, CreateProductRequest, UpdateProductRequest, ProductFilterOptions, ImageArray},
    entities::product,
};

// Get products with filtering, sorting, and pagination
pub async fn get_products(db: &DatabaseConnection, filter: ProductFilterOptions) -> Result<Vec<Product>> {
    let page = filter.page.unwrap_or(1);
    let per_page = filter.per_page.unwrap_or(10);
    let offset = ((page - 1) * per_page) as u64;
    let limit = per_page as u64;

    let mut query = product::Entity::find();

    // Apply filters
    if let Some(category_id) = filter.category_id {
        query = query.filter(product::Column::CategoryId.eq(category_id));
    }
    if let Some(search) = filter.search {
        query = query.filter(product::Column::Title.contains(&search));
    }
    if let Some(location) = filter.location {
        query = query.filter(product::Column::Location.contains(&location));
    }
    if let Some(featured) = filter.featured {
        query = query.filter(product::Column::Featured.eq(featured));
    }

    // Apply pagination and ordering
    let products = query
        .order_by_desc(product::Column::CreatedAt)
        .offset(offset)
        .limit(limit)
        .all(db)
        .await?;

    Ok(products.into_iter().map(Product::from).collect())
}

// Get a product by ID
pub async fn get_product_by_id(db: &DatabaseConnection, product_id: Uuid) -> Result<Product> {
    let product = product::Entity::find_by_id(product_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Product not found"))?;

    Ok(Product::from(product))
}

// Create a new product
pub async fn create_product(db: &DatabaseConnection, seller_id: Uuid, payload: CreateProductRequest) -> Result<Product> {
    let product = product::ActiveModel {
        title: Set(payload.title),
        description: Set(payload.description),
        price: Set(payload.price),
        category_id: Set(payload.category_id),
        seller_id: Set(seller_id),
        stock: Set(payload.stock),
        images: Set(payload.images),
        location: Set(payload.location.unwrap_or_else(|| "Unknown".to_string())),
        featured: Set(payload.featured.unwrap_or(false)),
        ..Default::default()
    };

    let product = product.insert(db).await?;
    Ok(Product::from(product))
}

// Update a product
pub async fn update_product(db: &DatabaseConnection, product_id: Uuid, seller_id: Uuid, payload: UpdateProductRequest) -> Result<Product> {
    let product = product::Entity::find_by_id(product_id)
        .filter(product::Column::SellerId.eq(seller_id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Product not found"))?;

    let mut product: product::ActiveModel = product.into();

    if let Some(title) = payload.title {
        product.title = Set(title);
    }
    if let Some(description) = payload.description {
        product.description = Set(description);
    }
    if let Some(price) = payload.price {
        product.price = Set(price);
    }
    if let Some(category_id) = payload.category_id {
        product.category_id = Set(category_id);
    }
    if let Some(stock) = payload.stock {
        product.stock = Set(stock);
    }
    if let Some(images) = payload.images {
        product.images = Set(images);
    }
    if let Some(location) = payload.location {
        product.location = Set(location);
    }
    if let Some(featured) = payload.featured {
        product.featured = Set(featured);
    }

    let product = product.update(db).await?;
    Ok(Product::from(product))
}

// Delete a product
pub async fn delete_product(db: &DatabaseConnection, product_id: Uuid, seller_id: Uuid) -> Result<()> {
    let result = product::Entity::delete_many()
        .filter(product::Column::Id.eq(product_id))
        .filter(product::Column::SellerId.eq(seller_id))
        .exec(db)
        .await?;

    if result.rows_affected == 0 {
        return Err(AppError::not_found("Product not found"));
    }

    Ok(())
}
