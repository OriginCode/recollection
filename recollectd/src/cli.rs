use clap::Parser;
use std::path::PathBuf;

/// Recollection Daemon
#[derive(Parser, Debug)]
#[clap(about, author, version)]
pub(crate) struct Args {
    /// Path to the data file
    #[clap(short, long, value_parser)]
    pub data: Option<PathBuf>,
}
