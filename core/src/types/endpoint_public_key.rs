use std::{array::TryFromSliceError, sync::Arc};

use iroh::KeyParsingError;
use sea_orm::{
    DbErr, TryGetError, TryGetable, Value,
    sea_query::{Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

new_type!{
    Self = EndpointPublicKey,
    Inner = iroh::PublicKey
}

impl_iroh_public_key!{
    Self = EndpointPublicKey,
    Inner = iroh::PublicKey,
    TryIntoError = TryIntoEndpointPublicKeyError,
}


#[derive(Debug, thiserror::Error)]
pub enum TryIntoEndpointPublicKeyError {
    #[error("invalid length {0}")]
    InvalidLength(#[from] TryFromSliceError),
    #[error("invalid value {0}")]
    InvalidValue(#[from] KeyParsingError),
}