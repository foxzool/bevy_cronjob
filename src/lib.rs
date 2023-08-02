use std::str::FromStr;

use bevy_ecs::prelude::Local;
use chrono::DateTime;
use cron::Schedule;

/// Creates a closure that checks if the cron expression has passed
pub fn schedule_passed(expression: &str) -> impl FnMut(Local<Option<DateTime<chrono::Utc>>>) -> bool {
    let schedule = Schedule::from_str(expression).expect("Failed to parse cron expression");
    move |mut local_schedule: Local<Option<DateTime<chrono::Utc>>>| {
        if let Some(datetime) = schedule.upcoming(chrono::Utc).next() {
            let now = chrono::Utc::now();
            match *local_schedule {
                Some(local) => {
                    if now > local {
                        *local_schedule = Some(datetime);
                        return true;
                    }
                }

                None => *local_schedule = Some(datetime),
            }
        }

        false
    }
}