use std::{array::TryFromSliceError, sync::Arc};

use iroh::KeyParsingError;
use sea_orm::{
    DbErr, TryGetError, TryGetable, Value,
    sea_query::{Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

/// A wrapper struct of [`iroh::PublicKey`]
///
/// # sea-orm
/// 
/// Saved as blob.
///
/// # Examples
/// ```
/// # use sea_orm::entity::prelude::*;
/// use caretta_brain::types::IrohEndpointId;
/// #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
/// #[sea_orm(table_name = "public_key_example")]
/// pub struct Model {
///     #[sea_orm(primary_key)]
///     pub id: u32,
///     pub public_key: IrohEndpointId
/// }
/// # #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
/// # pub enum Relation {}
/// #
/// # impl ActiveModelBehavior for ActiveModel{}
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IrohEndpointId(iroh::PublicKey);

impl IrohEndpointId {
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self,KeyParsingError> {
        iroh::PublicKey::from_bytes(bytes).map(|x| Self(x))
    }
}

impl From<iroh::PublicKey> for IrohEndpointId {
    fn from(value: iroh::PublicKey) -> Self {
        Self(value)
    }
}

impl From<IrohEndpointId> for iroh::PublicKey {
    fn from(value: IrohEndpointId) -> Self {
        value.0
    }
}

impl From<IrohEndpointId> for sea_orm::Value {
    fn from(value: IrohEndpointId) -> Self {
        Value::Bytes(Some(Vec::from(value.as_bytes())))
    }
}

impl TryFrom<&[u8]> for IrohEndpointId {
    type Error = TryIntoIrohEndpointIdError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let slice: [u8; 32] = value[0..32].try_into()?;
        Ok(Self::from_bytes(&slice)?)
    }
}

impl TryGetable for IrohEndpointId {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        let vec = <Vec<u8> as sea_orm::TryGetable>::try_get_by(res, index)?;
        let slice: [u8; 32] = vec[0..32].try_into().map_err(|x| DbErr::TryIntoErr {
            from: stringify!(Vec<u8>),
            into: stringify!(IrohEndpointId),
            source: Arc::new(x),
        })?;
        Ok(IrohEndpointId::from_bytes(&slice).map_err(|x| DbErr::TryIntoErr { from: stringify!(Vec<u8>), into: stringify!(IrohEndpointId), source: Arc::new(x) })?)
    }
}

impl ValueType for IrohEndpointId {
    fn try_from(v: Value) -> Result<Self, sea_orm_migration::prelude::ValueTypeErr> {
        let vec = <Vec<u8> as ValueType>::try_from(v)?;
        let key =
            <IrohEndpointId as TryFrom<&[u8]>>::try_from(&vec[0..32]).map_err(|_| ValueTypeErr)?;
        Ok(key)
    }
    fn type_name() -> String {
        stringify!(PublicKey).to_owned()
    }
    fn array_type() -> sea_orm_migration::prelude::ArrayType {
        sea_orm::sea_query::ArrayType::Bytes
    }
    fn column_type() -> sea_orm::ColumnType {
        sea_orm::sea_query::ColumnType::Blob
    }
}

impl sea_orm::sea_query::Nullable for IrohEndpointId {
    fn null() -> Value {
        <Vec<u8> as Nullable>::null()
    }
}

impl Serialize for IrohEndpointId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        todo!()
    }
}

impl<'de> Deserialize<'de> for IrohEndpointId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        todo!()
    }
}


#[derive(Debug, thiserror::Error)]
pub enum TryIntoIrohEndpointIdError {
    #[error("invalid length {0}")]
    InvalidLength(#[from] TryFromSliceError),
    #[error("invalid value {0}")]
    InvalidValue(#[from] KeyParsingError),
}