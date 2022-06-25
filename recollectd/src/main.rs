use anyhow::{bail, Result};
use chrono::Utc;
use clap::Parser;
use dirs::data_dir;
use librecollect::{JsonStorage, Storage};
use log::{info, warn};
use std::thread;

mod cli;

use cli::Args;

fn update_data<S: Storage + PartialEq>(data: &mut S) -> Result<()> {
    let file_data = S::load(data.path())?;

    if *data != file_data {
        *data = file_data;
        info!("Updating data from {}", data.path().display());
        info!(
            "Data file loaded, found {events} event(s)",
            events = data.events().len()
        );
    }

    Ok(())
}

fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let args = Args::parse();
    let data_path = args
        .data
        .unwrap_or_else(|| data_dir().unwrap().join("recollect.json"));

    if !data_path.exists() {
        warn!("Data file not found, consider using `recollectctl` to add events first");
        bail!("Data file not found");
    }

    let mut data = JsonStorage::new(data_path);

    loop {
        update_data(&mut data)?;

        for event in data.events() {
            // In case if the notification should be sent a while ago, we can still pick it up.
            if Utc::now() >= event.next_time() {
                event.notification().appname("Recollection").show()?;
                info!(
                    "Notification sent, event: {event}",
                    event = serde_json::to_string(event)?
                );

                // Update the next notification time.
                info!(
                    "Next time: {next}",
                    next = event.update_next_time().to_rfc3339()
                );
            }
        }

        // Write data back to file, in case the program is closed, we can still get the missing
        // notification sent, and also keep the file up-to-date.
        data.write()?;

        // Examine the events every second.
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
