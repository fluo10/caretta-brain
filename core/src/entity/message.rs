use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "message")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub sender: Uuid,
    pub body: String,
    #[sea_orm(has_many, via = "note_tag")]
    pub tags: HasMany<super::tag::Entity>,
    pub workspace_id: Uuid,
    #[sea_orm(belongs_to, from = "workspace_id", to = "id")]
    pub workspace: HasOne<super::workspace::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}