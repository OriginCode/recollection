use cron::Schedule;
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

use crate::RecollectError as Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    schedule: String,
    pub summary: String,
    pub body: String,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{schedule}\nSummary: {summary}\nBody: {body}",
            schedule = self.schedule,
            summary = self.summary,
            body = self.body
        )
    }
}

/// Test if the schedule string is valid in cron format.
pub fn validate_schedule<Sched: AsRef<str>>(schedule: Sched) -> Result<(), Error> {
    let schedule = schedule.as_ref();

    Schedule::from_str(schedule).map_err(|_| Error::ParseSchedError(schedule.to_owned()))?;

    Ok(())
}

impl Event {
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
        })
    }

    pub fn schedule(&self) -> Schedule {
        // Should not panic as we've already validated the schedule string.
        Schedule::from_str(&self.schedule).unwrap()
    }

    pub fn notification(&self) -> Notification {
        Notification::new()
            .summary(&self.summary)
            .body(&self.body)
            .finalize()
    }

    /// Validates the schedule string and update the schedule if it is valid.
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
        let event = Event::new("* * * * * * *", "summary", "body");

        assert_eq!(
            event.schedule().unwrap(),
            Schedule::from_str("* * * * * * *").unwrap()
        );
    }
}
