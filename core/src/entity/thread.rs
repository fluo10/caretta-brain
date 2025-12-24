use caretta_id::CarettaId;
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "thread")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    //pub public_id: CarettaId,
    pub title: String,
    #[sea_orm(has_many)]
    pub messages: HasMany<super::message::Entity>,
    pub workspace_id: Uuid,
}

impl ActiveModelBehavior for ActiveModel {}