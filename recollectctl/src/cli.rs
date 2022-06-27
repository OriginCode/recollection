use clap::Parser;
use std::path::PathBuf;

/// Recollection CLI Manager
#[derive(Parser, Debug)]
#[clap(about, author, version)]
pub(crate) struct Args {
    /// Path to the data file
    #[clap(short, long, value_parser)]
    pub data: Option<PathBuf>,
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Parser, Debug)]
pub(crate) enum Subcommand {
    /// List all events
    List,
    /// Clear all events
    Clear,
    /// Add an event
    Add,
    /// Remove an event
    Remove,
}
