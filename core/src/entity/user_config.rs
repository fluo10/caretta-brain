use sea_orm::{ActiveValue::Set, entity::prelude::*};

use crate::{context::ServiceContextExt, types::{NamespacePublicKey, NamespaceSecretKey}};

const ID: u32= 0;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_config")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: u32,
    pub iroh_namespace_id: NamespacePublicKey,
}

impl Model {
    const ID: u32 = 0;
    pub async fn from_secret<C>(ctx: &C, namespace: NamespaceSecretKey) -> Result<Self, DbErr> 
    where 
        C: ServiceContextExt
    {
        todo!()
    }

    pub async fn new<C>(ctx: &C) -> Result<Self, DbErr>
    where 
        C: ServiceContextExt
    {
        todo!()
    }

    pub async fn get<C>(ctx: &C) -> Result<Option<Self>, DbErr>
    where 
        C: ServiceContextExt
    {
        Entity::find_by_id(Self::ID).one(ctx.as_database_connection()).await
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation{}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {
    use crate::context::ServiceContextExt;

    use super::*;

    #[tokio::test]
    async fn insert_and_get_record() {
        let ctx  = crate::tests::service_conext().await;
        let model = Model::new(ctx).await.unwrap();
        assert_eq!(model, Model::get(ctx).await.unwrap().unwrap());
    }
}

