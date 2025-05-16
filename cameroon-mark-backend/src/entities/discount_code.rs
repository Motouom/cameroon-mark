use sea_orm::entity::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "discount_codes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub seller_id: Uuid,
    pub campaign_id: Option<Uuid>,
    pub code: String,
    pub discount_type: String,
    pub value: Decimal,
    pub min_purchase_amount: Option<Decimal>,
    pub max_uses: Option<i32>,
    pub times_used: i32,
    pub start_date: DateTimeWithTimeZone,
    pub end_date: DateTimeWithTimeZone,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::SellerId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::campaign::Entity",
        from = "Column::CampaignId",
        to = "super::campaign::Column::Id"
    )]
    Campaign,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::campaign::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Campaign.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
