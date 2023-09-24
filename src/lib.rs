/*! A simlpe key-value store
 *
 * TODO: include different engine and
 */

mod client;
mod common;
mod engines;
mod error;
mod server;

pub use client::KvsClient;
pub use engines::kvstore::KvStore;
pub use engines::KvsEngine;
pub use error::Error;
pub use server::KvsServer;

/// Alias for a Result with the error type kvs::Error
pub type Result<T> = std::result::Result<T, crate::Error>;
