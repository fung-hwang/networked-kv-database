/*! A simlpe key-value store
 *
 * Including:
 * 1. Key-value store with different engines
 * 2. Persistent key/value store server and client with synchronous networking over a custom protocol
 *
 * Anyway, these two things are placed in the same library o(╥﹏╥)o
 */

mod client;
mod common;
mod engines;
mod error;
mod server;

pub use client::KvsClient;
pub use engines::kvstore::KvStore;
pub use engines::redb::Redb;
pub use engines::KvsEngine;
pub use error::Error;
pub use server::KvsServer;

/// Alias for a Result with the error type kvs::Error
pub type Result<T> = std::result::Result<T, crate::Error>;
