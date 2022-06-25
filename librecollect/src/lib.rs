use thiserror::Error;

mod event;
mod storage;

pub use event::{validate_schedule, Event};
pub use storage::{JsonStorage, Storage};

#[derive(Error, Debug)]
pub enum RecollectError {
    #[error("Failed to parse schedule string: {0}")]
    ParseSchedError(String),
    #[error("Failed to load storage: {0}")]
    LoadError(#[from] std::io::Error),
    #[error("Failed to parse storage: {0}")]
    ParseStorageError(#[from] serde_json::Error),
}
