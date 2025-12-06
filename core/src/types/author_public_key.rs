use std::{array::TryFromSliceError, sync::Arc};

use iroh::{KeyParsingError,};
use sea_orm::{
    DbErr, TryGetError, TryGetable, Value,
    sea_query::{Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

super::macros::new_type!{
    Self = AuthorPublicKey,
    Inner = iroh_docs::AuthorPublicKey
}

super::macros::impl_iroh_public_key!{
    Self = AuthorPublicKey,
    Inner = iroh_docs::AuthorPublicKey,
    TryIntoError = TryIntoAuthorIdError,
}


#[derive(Debug, thiserror::Error)]
pub enum TryIntoAuthorIdError{
    #[error("invalid length {0}")]
    InvalidLength(#[from] TryFromSliceError),
    #[error("invalid value {0}")]
    InvalidValue(#[from] ed25519_dalek::SignatureError),
}