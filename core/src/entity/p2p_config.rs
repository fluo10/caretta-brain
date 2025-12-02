use iroh::SecretKey;
use sea_orm::{ActiveValue::Set, entity::prelude::*};

use crate::entity::types::SecretKeyBlob;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "p2p_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub enabled: bool,
    pub secret_key: SecretKeyBlob,
    pub enable_n0: bool,
    pub enable_mdns: bool,
}

impl Model {
    const ID: u32 = 0;

    pub async fn get_or_try_init(db: &DatabaseConnection) -> Result<Self, DbErr> {
        if let Some(x) = Entity::find_by_id(Self::ID).one(db).await? {
            Ok(x)
        } else {
            Ok(ActiveModel {
                id: Set(Self::ID),
                enabled: Set(true),
                secret_key: Set(SecretKey::generate(&mut rand::rng()).into()),
                enable_n0: Set(true),
                enable_mdns: Set(true),
            }
            .insert(db)
            .await?)
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation{}

impl ActiveModelBehavior for ActiveModel {}

