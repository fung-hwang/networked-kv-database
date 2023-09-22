/*! Just kvstore(temp) */

mod engines;
mod error;

pub use engines::kvstore::*;
pub use engines::KvsEngine;
pub use error::Error;

/// Alias for a Result with the error type kvs::Error
pub type Result<T> = std::result::Result<T, crate::Error>;
