use caretta_id::CarettaId;
use chrono::Local;
use sea_orm::{entity::prelude::*, };

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "invitation_token")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: CarettaId,
    pub created_at: chrono::DateTime<Local>,
    pub updated_at: chrono::DateTime<Local>,
    pub closed_at: Option<chrono::DateTime<Local>>,
}

impl ActiveModelBehavior for ActiveModel {}