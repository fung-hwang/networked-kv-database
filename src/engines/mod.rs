use crate::Result;

pub mod kvstore;
pub mod redb;

/// Trait for a key-value storage engine.
pub trait KvsEngine {
    /// New the key-value storage at the specified path
    fn open(path: impl AsRef<std::path::Path>) -> Result<Self>
    where
        Self: Sized;

    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// Removes a given key.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::KeyNotFound` if the given key is not found.
    fn remove(&mut self, key: String) -> Result<()>;
}
