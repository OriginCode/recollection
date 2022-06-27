use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{event::Event, RecollectError as Error};

pub trait Storage: PartialEq {
    fn new<P: Into<PathBuf>>(path: P) -> Self;
    fn load<P: Into<PathBuf>>(path: P) -> Result<Self, Error>
    where
        Self: Sized;
    fn write(&self) -> Result<(), Error>;
    fn events(&mut self) -> &mut Vec<Event>;
    fn path(&self) -> &Path;
    fn into_inner(self) -> Vec<Event>;
}

pub struct JsonStorage {
    events: Vec<Event>,
    path: PathBuf,
}

impl PartialEq for JsonStorage {
    fn eq(&self, other: &Self) -> bool {
        self.events == other.events
    }
}

impl Storage for JsonStorage {
    fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            events: Vec::new(),
            path: path.into(),
        }
    }

    fn load<P: Into<PathBuf>>(path: P) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let path = path.into();

        Ok(Self {
            events: serde_json::from_slice(&fs::read(&path)?)?,
            path,
        })
    }

    fn write(&self) -> Result<(), Error> {
        fs::write(&self.path, serde_json::to_string_pretty(&self.events)?)?;

        Ok(())
    }

    fn events(&mut self) -> &mut Vec<Event> {
        &mut self.events
    }

    fn path(&self) -> &Path {
        &self.path
    }

    /// Consumes the storage and returns the owned vector of events.
    fn into_inner(self) -> Vec<Event> {
        self.events
    }
}
