use std::str::FromStr;

use bevy_ecs::prelude::Local;
use chrono::DateTime;
use cron::Schedule;
pub use english_to_cron::str_cron_syntax;

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
///         .add_systems(Update, print_per_5_sec.run_if(schedule_passed("0/5 * * * ? *")))
///         .add_systems(Update, print_per_min.run_if(schedule_passed("0 * * * ? *")))
///         .add_systems(Update, print_per_hour.run_if(schedule_passed("0 0 * * ? *")))
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
pub const EVERY_5_SEC: &str = "0/5 * * * * ? *";
/// run every 10 sec
pub const EVERY_10_SEC: &str = "0/10 * * * * ? *";
/// run every 30 sec
pub const EVERY_30_SEC: &str = "0/30 * * * * ? *";
/// run every minute
pub const EVERY_MIN: &str = "0 * * * * ? *";
/// run every 5 minutes
pub const EVERY_5_MIN: &str = "0 0/5 * * * ? *";
/// run every 10 minutes
pub const EVERY_10_MIN: &str = "0 0/10 * * * ? *";
/// run every 30 minutes
pub const EVERY_30_MIN: &str = "0 0/30 * * * ? *";
/// run every hour
pub const EVERY_HOUR: &str = "0 0 * * * ? *";
/// run every day
pub const EVERY_DAY: &str = "0 0 0 */1 * ? *";

/// run every day at 1 am
pub const EVERY_1_AM: &str = "0 0 1 */1 * ? *";

/// run every day at 2 am
pub const EVERY_2_AM: &str = "0 0 2 */1 * ? *";

/// run every day at 3 am
pub const EVERY_3_AM: &str = "0 0 3 */1 * ? *";

/// run every day at 4 am
pub const EVERY_4_AM: &str = "0 0 4 */1 * ? *";

/// run every day at 5 am
pub const EVERY_5_AM: &str = "0 0 5 */1 * ? *";

/// run every day at 6 am
pub const EVERY_6_AM: &str = "0 0 6 */1 * ? *";

/// run every day at 7 am
pub const EVERY_7_AM: &str = "0 0 7 */1 * ? *";

/// run every day at 8 am
pub const EVERY_8_AM: &str = "0 0 8 */1 * ? *";

/// run every day at 9 am
pub const EVERY_9_AM: &str = "0 0 9 */1 * ? *";

/// run every day at 10 am
pub const EVERY_10_AM: &str = "0 0 10 */1 * ? *";

/// run every day at 11 am
pub const EVERY_11_AM: &str = "0 0 11 */1 * ? *";

/// run every day at 12 pm
pub const EVERY_12_PM: &str = "0 0 12 */1 * ? *";

/// run every day at 1 pm
pub const EVERY_1_PM: &str = "0 0 13 */1 * ? *";

/// run every day at 2 pm
pub const EVERY_2_PM: &str = "0 0 14 */1 * ? *";

/// run every day at 3 pm
pub const EVERY_3_PM: &str = "0 0 15 */1 * ? *";

/// run every day at 4 pm
pub const EVERY_4_PM: &str = "0 0 16 */1 * ? *";

/// run every day at 5 pm
pub const EVERY_5_PM: &str = "0 0 17 */1 * ? *";

/// run every day at 6 pm
pub const EVERY_6_PM: &str = "0 0 18 */1 * ? *";

/// run every day at 7 pm
pub const EVERY_7_PM: &str = "0 0 19 */1 * ? *";

/// run every day at 8 pm
pub const EVERY_8_PM: &str = "0 0 20 */1 * ? *";

/// run every day at 9 pm
pub const EVERY_9_PM: &str = "0 0 21 */1 * ? *";

/// run every day at 10 pm
pub const EVERY_10_PM: &str = "0 0 22 */1 * ? *";

/// run every day at 11 pm
pub const EVERY_11_PM: &str = "0 0 23 */1 * ? *";

/// run every day at 12 am
pub const EVERY_12_AM: &str = "0 0 0 */1 * ? *";

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
///
/// # english expression:
/// | expression | equal crontab expression|
/// |------|------|
/// |every 5 seconds | 0/5 * * * * *|
/// |every minute | 0 * * * * *|
/// |every hour | 0 0 * * * *|
/// |every day | 0 0 0 */1 * *|
/// |every day at 1 am | 0 0 1 */1 * *|
///
pub fn schedule_passed(
    expression: &str,
) -> impl FnMut(Local<Option<DateTime<chrono::Local>>>) -> bool {
    let new_expression = if expression.chars().any(|c| c.is_ascii_alphabetic()) {
        str_cron_syntax(expression).expect("Failed to parse cron expression")
    } else {
        expression.to_string()
    };

    let schedule = Schedule::from_str(&new_expression).expect("Failed to parse cron expression");
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

#[test]
fn test_expression() {
    assert_eq!(EVERY_5_SEC, str_cron_syntax("every 5 seconds").unwrap());
    assert_eq!(EVERY_10_SEC, str_cron_syntax("every 10 seconds").unwrap());
    assert_eq!(EVERY_30_SEC, str_cron_syntax("every 30 seconds").unwrap());
    assert_eq!(EVERY_MIN, str_cron_syntax("every minute").unwrap());
    assert_eq!(EVERY_5_MIN, str_cron_syntax("every 5 minutes").unwrap());
    assert_eq!(EVERY_10_MIN, str_cron_syntax("every 10 minutes").unwrap());
    assert_eq!(EVERY_30_MIN, str_cron_syntax("every 30 minutes").unwrap());
    assert_eq!(EVERY_HOUR, str_cron_syntax("every hour").unwrap());
    assert_eq!(EVERY_DAY, str_cron_syntax("every day").unwrap());
    assert_eq!(EVERY_1_AM, str_cron_syntax("every day at 1 am").unwrap());
    assert_eq!(EVERY_2_AM, str_cron_syntax("every day at 2 am").unwrap());
    assert_eq!(EVERY_3_AM, str_cron_syntax("every day at 3 am").unwrap());
    assert_eq!(EVERY_4_AM, str_cron_syntax("every day at 4 am").unwrap());
    assert_eq!(EVERY_5_AM, str_cron_syntax("every day at 5 am").unwrap());
    assert_eq!(EVERY_6_AM, str_cron_syntax("every day at 6 am").unwrap());
    assert_eq!(EVERY_7_AM, str_cron_syntax("every day at 7 am").unwrap());
    assert_eq!(EVERY_8_AM, str_cron_syntax("every day at 8 am").unwrap());
    assert_eq!(EVERY_9_AM, str_cron_syntax("every day at 9 am").unwrap());
    assert_eq!(EVERY_10_AM, str_cron_syntax("every day at 10 am").unwrap());
    assert_eq!(EVERY_11_AM, str_cron_syntax("every day at 11 am").unwrap());
    assert_eq!(EVERY_12_PM, str_cron_syntax("every day at 12 pm").unwrap());
    assert_eq!(EVERY_1_PM, str_cron_syntax("every day at 1 pm").unwrap());
    assert_eq!(EVERY_2_PM, str_cron_syntax("every day at 2 pm").unwrap());
    assert_eq!(EVERY_3_PM, str_cron_syntax("every day at 3 pm").unwrap());
    assert_eq!(EVERY_4_PM, str_cron_syntax("every day at 4 pm").unwrap());
    assert_eq!(EVERY_5_PM, str_cron_syntax("every day at 5 pm").unwrap());
    assert_eq!(EVERY_6_PM, str_cron_syntax("every day at 6 pm").unwrap());
    assert_eq!(EVERY_7_PM, str_cron_syntax("every day at 7 pm").unwrap());
    assert_eq!(EVERY_8_PM, str_cron_syntax("every day at 8 pm").unwrap());
    assert_eq!(EVERY_9_PM, str_cron_syntax("every day at 9 pm").unwrap());
    assert_eq!(EVERY_10_PM, str_cron_syntax("every day at 10 pm").unwrap());
    assert_eq!(EVERY_11_PM, str_cron_syntax("every day at 11 pm").unwrap());
    assert_eq!(EVERY_12_AM, str_cron_syntax("every day at 12 am").unwrap());
}
