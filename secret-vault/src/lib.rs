#![allow(unused_parens, clippy::new_without_default)]

mod allocator;
pub use allocator::*;

mod encryption;
pub use encryption::*;

pub mod errors;
mod secrets_source;
pub use secrets_source::*;

mod simple_sources;
pub use simple_sources::*;

mod vault_store;

mod common_types;
pub use common_types::*;

#[cfg(feature = "memory-protect")]
pub mod locked_allocator;

#[cfg(feature = "encrypted-ring")]
pub mod ring_encryption;

#[cfg(feature = "gcloud")]
pub mod gcp;

pub type SecretVaultResult<T> = std::result::Result<T, errors::SecretVaultError>;

mod vault;
pub use vault::*;

mod vault_builder;
pub use vault_builder::SecretVaultBuilder;

mod vault_viewer;
pub use vault_viewer::*;
