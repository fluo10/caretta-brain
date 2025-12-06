use std::{array::TryFromSliceError, sync::Arc};

use sea_orm::{
    DbErr, TryGetError, TryGetable, Value,
    sea_query::{Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

new_type!{
    Self = EndpointSecretKey,
    Inner = iroh::SecretKey
}

impl_iroh_secret_key!{
    Self = EndpointSecretKey,
    Inner = iroh::SecretKey,
    TryIntoError = TryIntoEndpointSecretKeyError,
    new = iroh::SecretKey::generate
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to convert to EndpointSecret: {0}")]
pub struct TryIntoEndpointSecretKeyError(#[from] TryFromSliceError);
