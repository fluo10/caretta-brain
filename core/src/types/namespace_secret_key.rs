use std::{array::TryFromSliceError, sync::Arc};

use sea_orm::{
    DbErr, TryGetError, TryGetable, Value,
    sea_query::{Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

new_type!{
    Self = NamespaceSecretKey,
    Inner = iroh_docs::NamespaceSecret
}

impl_iroh_secret_key!{
    Self = NamespaceSecretKey,
    Inner = iroh_docs::NamespaceSecret,
    TryIntoError = TryIntoNamespaceSecretKeyError,
    new = iroh_docs::NamespaceSecret::new
}
#[derive(Debug, thiserror::Error)]
#[error("Failed to convert to NamespaceSecret: {0}")]
pub struct TryIntoNamespaceSecretKeyError(#[from] TryFromSliceError);
