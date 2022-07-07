use clap::Parser;
use std::path::PathBuf;

/// Recollection CLI Manager
#[derive(Parser, Debug)]
#[clap(about, author, version, arg_required_else_help(true))]
pub(crate) struct Args {
    /// Path to the data file
    #[clap(short, long, value_parser)]
    pub data: Option<PathBuf>,
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Parser, Debug)]
pub(crate) enum Subcommand {
    /// Initialize Recollection
    Init,
    /// List all events
    List,
    /// Clear all events
    Clear,
    /// Add an event
    Add,
    /// Remove an event
    Remove,
    /// Edit events
    Edit,
    /// Disable events
    Disable,
    /// Show upcoming times
    Upcoming(Upcoming),
}

#[derive(Parser, Debug)]
pub(crate) struct Upcoming {
    /// Number of upcoming times to show
    pub count: usize,
}
