use anyhow::{bail, Result};
use chrono::Utc;
use clap::Parser;
use dirs::data_dir;
use librecollect::{JsonStorage, Storage};
use log::{info, warn};
use notify::{watcher, RecursiveMode, Watcher, DebouncedEvent};
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    flag::register,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::channel,
        Arc,
    },
    thread,
    time::Duration,
};

mod cli;

use cli::Args;

fn update_data<S: Storage + PartialEq>(data: &mut S) -> Result<()> {
    *data = S::load(data.path())?;
    info!("Updating data from {}", data.path().display());
    info!(
        "Data file loaded, found {events} event(s)",
        events = data.events().len()
    );

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

    let mut data = JsonStorage::load(data_path)?;

    // Monitor the data file
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher.watch(data.path(), RecursiveMode::NonRecursive)?;

    // Process the signals
    let term = Arc::new(AtomicBool::new(false));
    register(SIGINT, term.clone())?;
    register(SIGTERM, term.clone())?;

    while !term.load(Ordering::Relaxed) {
        if let Ok(DebouncedEvent::NoticeWrite(_)) = rx.try_recv() {
            update_data(&mut data)?;
        }

        for event in data.events() {
            if event.disabled {
                continue;
            }

            // In case if the notification should be sent a while ago, we can still pick it up.
            if Utc::now() >= event.upcoming() {
                event.notification().appname("Recollection").show()?;
                info!(
                    "Notification sent, event: {event}, next time: {next}",
                    event = serde_json::to_string(event)?,
                    // Update the next notification time.
                    next = event.update_upcoming().to_rfc3339(),
                );
            }
        }

        // Examine the events every second.
        thread::sleep(Duration::from_secs(1));
    }

    // Write data back to file, in case the program is terminated, we can still get the missing
    // notification sent, and also keep the file up-to-date.
    data.write()?;

    Ok(())
}
