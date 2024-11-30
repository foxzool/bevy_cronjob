use std::str::FromStr;

use bevy_ecs::prelude::Local;
use chrono::DateTime;
use cron::Schedule;

/// bevy_cronjob is a simple helper to run cronjobs (at repeated schedule) in Bevy.
/// # Usage
///
/// ``` rust,no_run
/// use std::time::Duration;
/// use bevy::{ MinimalPlugins};
/// use bevy::app::{App, PluginGroup, ScheduleRunnerPlugin, Update};
/// use bevy::log::{info, LogPlugin};
///
/// use bevy_ecs::prelude::{IntoSystemConfigs};
/// use bevy_cronjob::schedule_passed;
///
/// fn main() {
///     App::new()
///         .add_plugins(
///             MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
///                 1.0 / 60.0,
///             ))),
///         )
///         .add_plugins(LogPlugin::default())
///         .add_systems(Update, print_per_5_sec.run_if(schedule_passed("0/5 * * * * *")))
///         .add_systems(Update, print_per_min.run_if(schedule_passed("0 * * * * *")))
///         .add_systems(Update, print_per_hour.run_if(schedule_passed("0 0 * * * *")))
///         .run();
/// }
///
/// fn print_per_5_sec() {
///     info!("print every 5 sec")
/// }
///
/// fn print_per_min() {
///     info!("print every minute")
/// }
/// fn print_per_hour() {
///     info!("print every hour")
/// }
/// ```
///
/// ## Expression
///
/// the scheduling expression is base on [cron](https://github.com/zslayton/cron)
///
/// | sec  | min  | hour | day of month | month | day of week | year      |
/// |------|------|------|--------------|-------|-------------|-----------|
/// | *    | *    | *    | *            | *     | *           | *         |
/// | 0-59 | 0-59 | 0-23 | 1-23         | 1-12  | 1-7         | 1970-2100 |
///
/// Time is specified in Local Time. Note that the year may be omitted.
///
/// Comma separated values such as `1,2,3` are allowed. For example, a schedule of `0,15,30,45 * * *
/// * *`' would execute on every 15 seconds.
///
/// Ranges can be specified with a dash. For example `1-5 * * * * *`' would execute on every second
/// for the first 5 seconds of a minute.

/// run every 5 sec
pub const EVERY_5_SEC: &str = "0/5 * * * * * *";
/// run every 10 sec
pub const EVERY_10_SEC: &str = "0/10 * * * * * *";
/// run every 30 sec
pub const EVERY_30_SEC: &str = "0/30 * * * * * *";
/// run every minute
pub const EVERY_MIN: &str = "0 * * * * * *";
/// run every 5 minutes
pub const EVERY_5_MIN: &str = "0 */5 * * * * *";
/// run every 10 minutes
pub const EVERY_10_MIN: &str = "0 */10 * * * * *";
/// run every 30 minutes
pub const EVERY_30_MIN: &str = "0 */30 * * * * *";
/// run every hour
pub const EVERY_HOUR: &str = "0 0 * * * * *";
/// run every day
pub const EVERY_DAY: &str = "0 0 0 * * * *";

/// run every day at 1 am UTC
pub const EVERY_1_AM: &str = "0 0 1 * * * *";

/// run every day at 2 am UTC
pub const EVERY_2_AM: &str = "0 0 2 * * * *";

/// run every day at 3 am UTC
pub const EVERY_3_AM: &str = "0 0 3 * * * *";

/// run every day at 4 am UTC
pub const EVERY_4_AM: &str = "0 0 4 * * * *";

/// run every day at 5 am UTC
pub const EVERY_5_AM: &str = "0 0 5 * * * *";

/// run every day at 6 am UTC
pub const EVERY_6_AM: &str = "0 0 6 * * * *";

/// run every day at 7 am UTC
pub const EVERY_7_AM: &str = "0 0 7 * * * *";

/// run every day at 8 am UTC
pub const EVERY_8_AM: &str = "0 0 8 * * * *";

/// run every day at 9 am UTC
pub const EVERY_9_AM: &str = "0 0 9 * * * *";

/// run every day at 10 am UTC
pub const EVERY_10_AM: &str = "0 0 10 * * * *";

/// run every day at 11 am UTC
pub const EVERY_11_AM: &str = "0 0 11 * * * *";

/// run every day at 12 pm UTC
pub const EVERY_12_PM: &str = "0 0 12 * * * *";

/// run every day at 1 pm UTC
pub const EVERY_1_PM: &str = "0 0 13 * * * *";

/// run every day at 2 pm UTC
pub const EVERY_2_PM: &str = "0 0 14 * * * *";

/// run every day at 3 pm UTC
pub const EVERY_3_PM: &str = "0 0 15 * * * *";

/// run every day at 4 pm UTC
pub const EVERY_4_PM: &str = "0 0 16 * * * *";

/// run every day at 5 pm UTC
pub const EVERY_5_PM: &str = "0 0 17 * * * *";

/// run every day at 6 pm UTC
pub const EVERY_6_PM: &str = "0 0 18 * * * *";

/// run every day at 7 pm UTC
pub const EVERY_7_PM: &str = "0 0 19 * * * *";

/// run every day at 8 pm UTC
pub const EVERY_8_PM: &str = "0 0 20 * * * *";

/// run every day at 9 pm UTC
pub const EVERY_9_PM: &str = "0 0 21 * * * *";

/// run every day at 10 pm UTC
pub const EVERY_10_PM: &str = "0 0 22 * * * *";

/// run every day at 11 pm UTC
pub const EVERY_11_PM: &str = "0 0 23 * * * *";

/// run every day at 12 am UTC
pub const EVERY_12_AM: &str = "0 0 0 * * * *";

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
pub fn schedule_passed(
    expression: &str,
) -> impl FnMut(Local<Option<DateTime<chrono::Local>>>) -> bool {
    let schedule = Schedule::from_str(expression).expect("Failed to parse cron expression");
    move |mut local_schedule: Local<Option<DateTime<chrono::Local>>>| {
        if let Some(datetime) = schedule.upcoming(chrono::Local).next() {
            let now = chrono::Local::now();
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
