use chrono::{DateTime, Utc};
use cron::Schedule;
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

use crate::RecollectError as Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    schedule: String,
    pub summary: String,
    pub body: String,
    pub next: Option<DateTime<Utc>>,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Schedule: {schedule}\nSummary: {summary}\nBody: {body}",
            schedule = self.schedule,
            summary = self.summary,
            body = self.body
        )
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.schedule == other.schedule && self.summary == other.summary && self.body == other.body
    }
}

/// Test if the schedule string is valid in cron format.
pub fn validate_schedule<Sched: AsRef<str>>(schedule: Sched) -> Result<(), Error> {
    let schedule = schedule.as_ref();

    Schedule::from_str(schedule).map_err(|_| Error::ParseSchedError(schedule.to_owned()))?;

    Ok(())
}

impl Event {
    /// Creates a new event.
    ///
    /// # Errors
    ///
    /// When the schedule string is invalid in cron format, `RecollectError::ParseSchedError` will
    /// be returned.
    pub fn new<Sched, Sum, Body>(schedule: Sched, summary: Sum, body: Body) -> Result<Self, Error>
    where
        Sched: Into<String>,
        Sum: Into<String>,
        Body: Into<String>,
    {
        let schedule = schedule.into();
        validate_schedule(&schedule)?;

        Ok(Self {
            schedule,
            summary: summary.into(),
            body: body.into(),
            next: None,
        })
    }

    /// Returns the cron formatted `Schedule` object.
    pub fn schedule(&self) -> Schedule {
        // Should not panic as we've already validated the schedule string.
        Schedule::from_str(&self.schedule).unwrap()
    }

    /// Returns the next time the notification should be sent.
    ///
    /// If `self.next` is None, this will return the next approaching time and update the `next`
    /// field, otherwise it will return the time stored in `self.next` to show the missed time.
    pub fn next_time(&mut self) -> DateTime<Utc> {
        self.next.unwrap_or_else(|| self.update_next_time())
    }

    /// Updates the next time the notification should be sent and returns it.
    pub fn update_next_time(&mut self) -> DateTime<Utc> {
        let next = self.schedule().upcoming(Utc).take(1).next().unwrap();
        self.next = Some(next);
        next
    }

    /// Returns the notification to be sent.
    pub fn notification(&self) -> Notification {
        Notification::new()
            .summary(&self.summary)
            .body(&self.body)
            .finalize()
    }

    /// Validates the schedule string and update the schedule if it is valid.
    ///
    /// # Errors
    ///
    /// When the schedule string is invalid in cron format, `RecollectError::ParseSchedError` will
    /// be returned.
    pub fn update_schedule<S: Into<String>>(&mut self, schedule: S) -> Result<(), Error> {
        let sched = schedule.into();

        Schedule::from_str(&sched)
            .map(|_| self.schedule = sched.clone())
            .map_err(|_| Error::ParseSchedError(sched))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule() {
        let event = Event::new("* * * * * * *", "summary", "body").unwrap();

        assert_eq!(
            event.schedule(),
            Schedule::from_str("* * * * * * *").unwrap()
        );
    }
}
