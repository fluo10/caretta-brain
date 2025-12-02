use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "tag_thread")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub thread_id: Uuid,
    #[sea_orm(belongs_to, from = "thread_id", to = "id")]
    pub thread: Option<super::note::Entity>,
    #[sea_orm(belongs_to, from = "tab_id", to = "id")]
    pub tag: Option<super::tag::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}