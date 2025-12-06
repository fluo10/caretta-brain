use std::{array::TryFromSliceError, sync::Arc};

use iroh::KeyParsingError;
use sea_orm::{
    DbErr, TryGetError, TryGetable, Value,
    sea_query::{Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

new_type!{
    Self = NamespacePublicKey,
    Inner = iroh_docs::NamespacePublicKey
}

impl_iroh_public_key!{
    Self = NamespacePublicKey,
    Inner = iroh_docs::NamespacePublicKey,
    TryIntoError = TryIntoNamespacePublicKeyError,
}


#[derive(Debug, thiserror::Error)]
pub enum TryIntoNamespacePublicKeyError {
    #[error("invalid length {0}")]
    InvalidLength(#[from] TryFromSliceError),
    #[error("invalid value {0}")]
    InvalidValue(#[from] ed25519_dalek::SignatureError),
}