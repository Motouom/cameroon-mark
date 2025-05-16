use sea_orm::entity::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "email_campaigns")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub seller_id: Uuid,
    pub campaign_id: Option<Uuid>,
    pub subject: String,
    pub content: String,
    pub recipient_list: String,  // Serialized JSON of recipients
    pub scheduled_time: Option<DateTimeWithTimeZone>,
    pub sent_time: Option<DateTimeWithTimeZone>,
    pub status: String,           // draft, scheduled, sent, failed
    pub opened_count: i32,
    pub click_count: i32,
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
