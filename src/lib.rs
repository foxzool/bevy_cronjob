use std::str::FromStr;

use bevy_ecs::prelude::Local;
use chrono::DateTime;
use cron::Schedule;

/// Creates a closure that checks if the cron expression has passed
/// # expression format:
/// Note that the year may be omitted.
/// | sec  | min  | hour | day of month | month | day of week | year      |
/// |------|------|------|--------------|-------|-------------|-----------|
/// | *    | *    | *    | *            | *     | *           | *         |
/// | 0-59 | 0-59 | 0-23 | 1-23         | 1-12  | 1-7         | 1970-2100 |
///
/// # example:
/// | expression | description|
/// |------|------|
/// |0/5 * * * * * | every 5 sec|
/// |0 * * * * * | every minute |
/// |0 5,10 * * * * | every hour on 5 and 10 min|
/// |0 0 1 * * * | every day on 1:00:00|
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