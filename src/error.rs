use thiserror::Error;

/// This type represents all possible errors in kvs lib.
#[derive(Error, Debug)]
pub enum Error {
    // TODO: Classified by module
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
    #[error("Jammdb Error: {}", .0)]
    Jammdb(#[from] jammdb::Error),
    #[error("Rayon ThreadPoolBuildError: {}", .0)]
    RayonThreadPoolBuildError(#[from] rayon::ThreadPoolBuildError),
}
