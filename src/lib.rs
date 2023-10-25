/*! A simlpe key-value store
 *
 * Including:
 * 1. Key-value store with different engines
 * 2. Persistent key/value store server and client with synchronous networking over a custom protocol
 *
 * Anyway, these two things are placed in the same library o(╥﹏╥)o
 */

mod cs; // client-server
mod engines; // storage engines
mod error;
mod thread_pool;

pub use cs::client::KvsClient;
pub use cs::server::KvsServer;
pub use engines::jammdb::Jammdb;
pub use engines::kvstore::KvStore;
pub use engines::KvsEngine;
// pub use engines::redb::Redb;
pub use error::Error;
pub use thread_pool::naive::NaiveThreadPool;
pub use thread_pool::rayon::RayonThreadPool;
pub use thread_pool::shared_queue::SharedQueueThreadPool;
pub use thread_pool::ThreadPool;

/// Alias for a Result with the error type kvs::Error
pub type Result<T> = std::result::Result<T, crate::Error>;
