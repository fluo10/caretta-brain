use sea_orm::{ActiveValue::Set, entity::prelude::*};

use crate::types::{EndpointSecretKey};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "device_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub iroh_endpoint_secret: EndpointSecretKey,
    pub iroh_enable_n0: bool,
    pub iroh_enable_mdns: bool,
}

impl Model {
    const ID: u32 = 0;

    pub async fn get_or_try_init(db: &DatabaseConnection) -> Result<Self, DbErr> {
        if let Some(x) = Entity::find_by_id(Self::ID).one(db).await? {
            Ok(x)
        } else {
            Ok(ActiveModel {
                id: Set(Self::ID),
                iroh_endpoint_secret: Set(EndpointSecretKey::new()),
                iroh_enable_n0: Set(true),
                iroh_enable_mdns: Set(true),
            }
            .insert(db)
            .await?)
        }
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
        let db = crate::tests::service_conext().await.as_database_connection();
        let model = Model::get_or_try_init(db).await.unwrap();
        assert_eq!(model, Model::get_or_try_init(db).await.unwrap());
    }
}