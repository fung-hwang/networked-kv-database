use thiserror::Error;

/// This type represents all possible errors in kvs lib.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Key not found")]
    KeyNotFound,
    #[error("Unexpected Command")]
    UnexpectedCommand,
    #[error("{}",.0)]
    ErrorMessage(String),
    #[error("IO Error: {}",.0)]
    IO(#[from] std::io::Error),
    #[error("Serde_json Error: {}", .0)]
    SerdeJson(#[from] serde_json::Error),
    #[error("redb Error: {}", .0)]
    Redb(#[from] redb::Error),
}
