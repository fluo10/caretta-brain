use std::{array::TryFromSliceError, sync::Arc};

use sea_orm::{
    DbErr, TryGetError, TryGetable, Value,
    sea_query::{Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

/// A wrapper struct of [`iroh::SecretKey`]`
///
/// Saved as blob.
///
/// # Examples
/// ```
/// # use sea_orm::entity::prelude::*;
/// use caretta_brain_backend::types::SecretKey;
/// #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
/// #[sea_orm(table_name = "secret_key_example")]
/// pub struct Model {
///     #[sea_orm(primary_key)]
///     pub id: u32,
///     pub secret_key: IrohEndpointSecret
/// }
/// # #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
/// # pub enum Relation {}
/// #
/// # impl ActiveModelBehavior for ActiveModel{}
/// ```
#[derive(Clone, Debug)]
pub struct IrohEndpointSecret(iroh::SecretKey);

impl IrohEndpointSecret {
    pub fn generate() -> Self {
        Self(iroh::SecretKey::generate(&mut rand::rng()))
    }
}

impl IrohEndpointSecret {
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        Self(iroh::SecretKey::from_bytes(bytes))
    }
}

impl PartialEq for IrohEndpointSecret {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes().eq(&other.to_bytes())
    }
}

impl From<iroh::SecretKey> for IrohEndpointSecret {
    fn from(value: iroh::SecretKey) -> Self {
        Self(value)
    }
}

impl From<IrohEndpointSecret> for iroh::SecretKey {
    fn from(value: IrohEndpointSecret) -> Self {
        value.0
    }
}

impl From<IrohEndpointSecret> for sea_orm::Value {
    fn from(value: IrohEndpointSecret) -> Self {
        Value::Bytes(Some(Vec::from(&value.to_bytes())))
    }
}

impl From<&[u8; 32]> for IrohEndpointSecret {
    fn from(value: &[u8; 32]) -> Self {
        Self::from_bytes(value)
    }
}

impl TryFrom<&[u8]> for IrohEndpointSecret {
    type Error = TryIntoIrohEndpointSecretError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let slice: [u8; 32] = value[0..32].try_into()?;
        Ok(Self::from_bytes(&slice))
    }
}

impl TryGetable for IrohEndpointSecret {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        let vec = <Vec<u8> as sea_orm::TryGetable>::try_get_by(res, index)?;
        let slice: [u8; 32] = vec[0..32].try_into().map_err(|x| DbErr::TryIntoErr {
            from: stringify!(Vec<u8>),
            into: stringify!(SecretKey),
            source: Arc::new(x),
        })?;
        Ok(IrohEndpointSecret::from_bytes(&slice))
    }
}

impl ValueType for IrohEndpointSecret {
    fn try_from(v: Value) -> Result<Self, sea_orm_migration::prelude::ValueTypeErr> {
        let vec = <Vec<u8> as ValueType>::try_from(v)?;
        let key =
            <IrohEndpointSecret as TryFrom<&[u8]>>::try_from(&vec[0..32]).map_err(|_| ValueTypeErr)?;
        Ok(key)
    }
    fn type_name() -> String {
        stringify!(SecretKey).to_owned()
    }
    fn array_type() -> sea_orm_migration::prelude::ArrayType {
        sea_orm::sea_query::ArrayType::Bytes
    }
    fn column_type() -> sea_orm::ColumnType {
        sea_orm::sea_query::ColumnType::Blob
    }
}

impl sea_orm::sea_query::Nullable for IrohEndpointSecret {
    fn null() -> Value {
        <Vec<u8> as Nullable>::null()
    }
}

impl Serialize for IrohEndpointSecret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        todo!()
    }
}

impl<'de> Deserialize<'de> for IrohEndpointSecret {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        todo!()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to convert to IrohEndpointSecret: {0}")]
pub struct TryIntoIrohEndpointSecretError(#[from] TryFromSliceError);
