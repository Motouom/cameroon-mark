use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use bigdecimal::BigDecimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub total_amount: BigDecimal,
    #[sea_orm(column_type = "String(StringLen::N(20))", nullable)]
    pub status: String,
    #[sea_orm(column_type = "String(StringLen::N(20))", nullable)]
    pub payment_status: String,
    #[sea_orm(column_type = "String(StringLen::N(20))", nullable)]
    pub payment_method: String,
    pub shipping_address: Json,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::order_item::Entity",
        from = "Column::Id",
        to = "super::order_item::Column::OrderId"
    )]
    OrderItems,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::order_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrderItems.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 