use sea_orm::{ActiveValue::Set, entity::prelude::*};

use crate::types::{IrohEndpointSecret};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "device_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub p2p_enabled: bool,
    pub p2p_secret_key: IrohEndpointSecret,
    pub p2p_enable_n0: bool,
    pub p2p_enable_mdns: bool,
}

impl Model {
    const ID: u32 = 0;

    pub async fn get_or_try_init(db: &DatabaseConnection) -> Result<Self, DbErr> {
        if let Some(x) = Entity::find_by_id(Self::ID).one(db).await? {
            Ok(x)
        } else {
            Ok(ActiveModel {
                id: Set(Self::ID),
                p2p_enabled: Set(true),
                p2p_secret_key: Set(IrohEndpointSecret::generate()),
                p2p_enable_n0: Set(true),
                p2p_enable_mdns: Set(true),
            }
            .insert(db)
            .await?)
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation{}

impl ActiveModelBehavior for ActiveModel {}

