use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect};
use librecollect::{validate_schedule, Event, Storage};

pub(crate) fn clear<S: Storage>(storage: &mut S) -> Result<()> {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Are you sure you want to clear all events?")
        .interact()?
    {
        storage.events().clear();
    } else {
        eprintln!("Operation cancelled.");
    }

    Ok(())
}

pub(crate) fn add<S: Storage>(storage: &mut S) -> Result<()> {
    let schedule: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Event schedule in crontab format")
        .interact()?;

    if validate_schedule(&schedule).is_err() {
        eprintln!("Invalid schedule format, please use cron format.");
        add(storage)?;

        return Ok(());
    }

    let summary: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Event summary")
        .interact()?;
    let body: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Event body")
        .interact()?;
    let disabled: bool = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Disable the event by default?")
        .default(false)
        .interact()?;

    storage
        .events()
        .push(Event::new(schedule, summary, body, disabled)?);

    Ok(())
}

pub(crate) fn remove<S: Storage>(storage: &mut S) -> Result<()> {
    let to_be_removed = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select events to be removed")
        .items(storage.events())
        .interact()?;

    for (offset, index) in to_be_removed.into_iter().enumerate() {
        storage.events().remove(index - offset);
    }

    Ok(())
}

fn edit(event: &mut Event) -> Result<()> {
    let schedule: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Event schedule in crontab format")
        .with_initial_text(event.schedule())
        .interact()?;

    if validate_schedule(&schedule).is_err() {
        eprintln!("Invalid schedule format, please use cron format.");
        edit(event)?;

        return Ok(());
    }

    event.update_schedule(schedule)?;
    event.summary = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Event summary")
        .with_initial_text(&event.summary)
        .interact()?;
    event.body = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Event body")
        .with_initial_text(&event.body)
        .interact()?;
    event.disabled = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Disable event?")
        .default(false)
        .interact()?;

    Ok(())
}

pub(crate) fn select_edit<S: Storage>(storage: &mut S) -> Result<()> {
    let to_be_edited = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select events to be edited")
        .items(storage.events())
        .interact()?;

    for index in to_be_edited {
        edit(&mut storage.events()[index])?;
    }

    Ok(())
}

pub(crate) fn disable<S: Storage>(storage: &mut S) -> Result<()> {
    let disabled = storage.events().iter().fold(Vec::new(), |mut acc, event| {
        acc.push(event.disabled);
        acc
    });
    let to_be_disabled = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select events to be edited")
        .items(storage.events())
        .defaults(&disabled)
        .interact()?;

    for index in to_be_disabled {
        storage.events()[index].disabled = true;
    }

    Ok(())
}

pub(crate) fn upcoming<S: Storage>(storage: &mut S, n: usize) -> Result<()> {
    let events = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select events")
        .items(storage.events())
        .interact()?;

    events.iter().for_each(|index| {
        let event = &storage.events()[*index];

        println!("{}:", event.summary);
        event.upcoming_timeline(n).iter().for_each(|time| {
            println!("{}", time);
        });
        println!();
    });

    Ok(())
}
