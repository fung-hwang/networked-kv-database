//! Commands across the network for client and server
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Requests {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response4Get {
    Ok(Option<String>),
    Err(String), // crate::Error can't be serialized, and String is the next best thing
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response4Set {
    Ok(()),
    Err(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response4Remove {
    Ok(()),
    Err(String),
}
