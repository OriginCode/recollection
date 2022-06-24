use thiserror::Error;

mod event;
mod storage;

pub use event::Event;
pub use storage::{JsonStorage, Storage};

#[derive(Error, Debug)]
pub enum RecollectError {
    #[error("Failed to parse schedule string: {0}")]
    ParseEventError(String),
    #[error("Failed to load storage: {0}")]
    LoadError(#[from] std::io::Error),
    #[error("Failed to parse storage: {0}")]
    ParseStorageError(#[from] serde_json::Error),
}