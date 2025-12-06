use std::{array::TryFromSliceError, sync::Arc};

use sea_orm::{
    DbErr, TryGetError, TryGetable, Value,
    sea_query::{Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

super::macros::new_type!{
    Self = AuthorSecretKey,
    Inner = iroh_docs::Author
}

super::macros::impl_iroh_secret_key!{
    Self = AuthorSecretKey,
    Inner = iroh_docs::Author,
    TryIntoError = TryIntoAuthorSecretKeyError,
    new = iroh_docs::Author::new
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to convert to AuthorSecret: {0}")]
pub struct TryIntoAuthorSecretKeyError(#[from] TryFromSliceError);
