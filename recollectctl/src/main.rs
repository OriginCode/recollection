use anyhow::Result;
use clap::Parser;
use dirs::data_dir;
use librecollect::{JsonStorage, Storage};
use std::fs;

mod cli;
mod command;

use cli::{Args, Subcommand};

fn main() -> Result<()> {
    let args = Args::parse();
    let data_path = args
        .data
        .unwrap_or_else(|| data_dir().unwrap().join("recollect.json"));

    let mut data = if !data_path.exists() {
        fs::File::create(&data_path)?;
        JsonStorage::new(data_path)
    } else {
        JsonStorage::load(data_path)?
    };

    match args.subcommand {
        Some(s) => match s {
            Subcommand::Init => (), // do nothing as we've created an empty data above
            Subcommand::List => data.events().iter().for_each(|e| println!("{}\n", e)),
            Subcommand::Clear => command::clear(&mut data)?,
            Subcommand::Add => command::add(&mut data)?,
            Subcommand::Remove => command::remove(&mut data)?,
            Subcommand::Edit => command::select_edit(&mut data)?,
            Subcommand::Disable => command::disable(&mut data)?,
            Subcommand::Upcoming(args) => command::upcoming(&mut data, args.count)?,
        },
        None => unreachable!(),
    }

    data.write()?;

    Ok(())
}
