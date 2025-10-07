//! `bevy_cronjob` is a simple helper to run cronjobs (at repeated schedule) in Bevy.
//!
//! # Usage
//!
//! ```rust,no_run
//! use bevy::log::LogPlugin;
//! use bevy::prelude::*;
//! use bevy_app::ScheduleRunnerPlugin;
//! use bevy_cronjob::prelude::*;
//! use std::time::Duration;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(
//!             MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
//!                 1.0 / 60.0,
//!             ))),
//!         )
//!         .add_plugins(LogPlugin::default())
//!         .add_plugins(CronJobPlugin)
//!         .add_systems(Update, print_per_5_sec.run_if(schedule_passed("0/5 * * * ? *")))
//!         .add_systems(Update, print_per_min.run_if(schedule_passed("0 * * * ? *")))
//!         .add_systems(Update, print_per_hour.run_if(schedule_passed("0 0 * * ? *")))
//!         .run();
//! }
//!
//! fn print_per_5_sec() {
//!     info!("print every 5 sec")
//! }
//!
//! fn print_per_min() {
//!     info!("print every minute")
//! }
//!
//! fn print_per_hour() {
//!     info!("print every hour")
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands
//!         .spawn(ScheduleTimer::new("every 3 seconds"))
//!         .observe(|_: On<ScheduleArrived>| {
//!             info!("3 seconds passed");
//!         });
//! }
//! ```
//!
//! ## Expression Format
//!
//! The scheduling expression is based on the [cron](https://github.com/zslayton/cron) crate.
//!
//! | sec  | min  | hour | day of month | month | day of week | year      |
//! |------|------|------|--------------|-------|-------------|-----------|
//! | *    | *    | *    | *            | *     | *           | *         |
//! | 0-59 | 0-59 | 0-23 | 1-31         | 1-12  | 1-7         | 1970-2100 |
//!
//! Time is specified in Local Time. Note that the year may be omitted.
//!
//! Comma separated values such as `1,2,3` are allowed. For example, a schedule of `0,15,30,45 * * *
//! * *` would execute every 15 seconds.
//!
//! Ranges can be specified with a dash. For example, `1-5 * * * * *` would execute every second
//! for the first 5 seconds of a minute.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use chrono::{DateTime, Local as ChronoLocal};
use cron::Schedule;
pub use english_to_cron::str_cron_syntax;
use std::str::FromStr;

// Common cron expression constants for convenience
/// Execute every 5 seconds: "0/5 * * * * ? *"
pub const EVERY_5_SEC: &str = "0/5 * * * * ? *";
/// Execute every 10 seconds: "0/10 * * * * ? *"
pub const EVERY_10_SEC: &str = "0/10 * * * * ? *";
/// Execute every 30 seconds: "0/30 * * * * ? *"
pub const EVERY_30_SEC: &str = "0/30 * * * * ? *";
/// Execute every minute: "0 * * * * ? *"
pub const EVERY_MIN: &str = "0 * * * * ? *";
/// Execute every 5 minutes: "0 0/5 * * * ? *"
pub const EVERY_5_MIN: &str = "0 0/5 * * * ? *";
/// Execute every 10 minutes: "0 0/10 * * * ? *"
pub const EVERY_10_MIN: &str = "0 0/10 * * * ? *";
/// Execute every 30 minutes: "0 0/30 * * * ? *"
pub const EVERY_30_MIN: &str = "0 0/30 * * * ? *";
/// Execute every hour: "0 0 * * * ? *"
pub const EVERY_HOUR: &str = "0 0 * * * ? *";
/// Execute every day at midnight: "0 0 0 */1 * ? *"
pub const EVERY_DAY: &str = "0 0 0 */1 * ? *";

/// Execute every day at 1 AM: "0 0 1 */1 * ? *"
pub const EVERY_1_AM: &str = "0 0 1 */1 * ? *";
/// Execute every day at 2 AM: "0 0 2 */1 * ? *"
pub const EVERY_2_AM: &str = "0 0 2 */1 * ? *";
/// Execute every day at 3 AM: "0 0 3 */1 * ? *"
pub const EVERY_3_AM: &str = "0 0 3 */1 * ? *";
/// Execute every day at 4 AM: "0 0 4 */1 * ? *"
pub const EVERY_4_AM: &str = "0 0 4 */1 * ? *";
/// Execute every day at 5 AM: "0 0 5 */1 * ? *"
pub const EVERY_5_AM: &str = "0 0 5 */1 * ? *";
/// Execute every day at 6 AM: "0 0 6 */1 * ? *"
pub const EVERY_6_AM: &str = "0 0 6 */1 * ? *";
/// Execute every day at 7 AM: "0 0 7 */1 * ? *"
pub const EVERY_7_AM: &str = "0 0 7 */1 * ? *";
/// Execute every day at 8 AM: "0 0 8 */1 * ? *"
pub const EVERY_8_AM: &str = "0 0 8 */1 * ? *";
/// Execute every day at 9 AM: "0 0 9 */1 * ? *"
pub const EVERY_9_AM: &str = "0 0 9 */1 * ? *";
/// Execute every day at 10 AM: "0 0 10 */1 * ? *"
pub const EVERY_10_AM: &str = "0 0 10 */1 * ? *";
/// Execute every day at 11 AM: "0 0 11 */1 * ? *"
pub const EVERY_11_AM: &str = "0 0 11 */1 * ? *";
/// Execute every day at 12 PM (noon): "0 0 12 */1 * ? *"
pub const EVERY_12_PM: &str = "0 0 12 */1 * ? *";
/// Execute every day at 1 PM: "0 0 13 */1 * ? *"
pub const EVERY_1_PM: &str = "0 0 13 */1 * ? *";
/// Execute every day at 2 PM: "0 0 14 */1 * ? *"
pub const EVERY_2_PM: &str = "0 0 14 */1 * ? *";
/// Execute every day at 3 PM: "0 0 15 */1 * ? *"
pub const EVERY_3_PM: &str = "0 0 15 */1 * ? *";
/// Execute every day at 4 PM: "0 0 16 */1 * ? *"
pub const EVERY_4_PM: &str = "0 0 16 */1 * ? *";
/// Execute every day at 5 PM: "0 0 17 */1 * ? *"
pub const EVERY_5_PM: &str = "0 0 17 */1 * ? *";
/// Execute every day at 6 PM: "0 0 18 */1 * ? *"
pub const EVERY_6_PM: &str = "0 0 18 */1 * ? *";
/// Execute every day at 7 PM: "0 0 19 */1 * ? *"
pub const EVERY_7_PM: &str = "0 0 19 */1 * ? *";
/// Execute every day at 8 PM: "0 0 20 */1 * ? *"
pub const EVERY_8_PM: &str = "0 0 20 */1 * ? *";
/// Execute every day at 9 PM: "0 0 21 */1 * ? *"
pub const EVERY_9_PM: &str = "0 0 21 */1 * ? *";
/// Execute every day at 10 PM: "0 0 22 */1 * ? *"
pub const EVERY_10_PM: &str = "0 0 22 */1 * ? *";
/// Execute every day at 11 PM: "0 0 23 */1 * ? *"
pub const EVERY_11_PM: &str = "0 0 23 */1 * ? *";
/// Execute every day at 12 AM (midnight): "0 0 0 */1 * ? *"
pub const EVERY_12_AM: &str = "0 0 0 */1 * ? *";

/// Creates a run condition that checks if the cron expression has triggered.
///
/// This function returns a closure that can be used with Bevy's `run_if` system parameter
/// to conditionally execute systems based on cron schedules.
///
/// # Expression Format
///
/// The year field may be omitted from cron expressions.
///
/// | Field        | Values    | Description           |
/// |--------------|-----------|----------------------|
/// | sec          | 0-59      | Seconds              |
/// | min          | 0-59      | Minutes              |
/// | hour         | 0-23      | Hours (24-hour)      |
/// | day of month | 1-31      | Day of the month     |
/// | month        | 1-12      | Month                |
/// | day of week  | 1-7       | Day of the week      |
/// | year         | 1970-2100 | Year (optional)      |
///
/// # Examples
///
/// | Expression          | Description                    |
/// |--------------------|--------------------------------|
/// | `0/5 * * * * *`    | Every 5 seconds               |
/// | `0 * * * * *`      | Every minute                  |
/// | `0 5,10 * * * *`   | Every hour at 5 and 10 minutes|
/// | `0 0 1 * * *`      | Every day at 1:00:00 AM       |
///
/// # English Expressions
///
/// The function also supports English expressions via the `english-to-cron` crate:
///
/// | English Expression     | Equivalent Cron Expression |
/// |------------------------|---------------------------|
/// | `every 5 seconds`      | `0/5 * * * * *`          |
/// | `every minute`         | `0 * * * * *`            |
/// | `every hour`           | `0 0 * * * *`            |
/// | `every day`            | `0 0 0 */1 * *`          |
/// | `every day at 1 am`    | `0 0 1 */1 * *`          |
///
/// # Panics
///
/// Panics if the provided expression cannot be parsed as a valid cron expression.
pub fn schedule_passed(
    expression: &str,
) -> impl FnMut(Local<Option<DateTime<ChronoLocal>>>) -> bool {
    let expression = parse_expression(expression);
    let schedule = Schedule::from_str(&expression).expect("Failed to parse cron expression");

    move |mut last_trigger: Local<Option<DateTime<ChronoLocal>>>| {
        let now = ChronoLocal::now();

        match *last_trigger {
            Some(last) => {
                // If we have a previous trigger time, check for the next scheduled time after it
                if let Some(next_time) = schedule.after(&last).next()
                    && now >= next_time
                {
                    *last_trigger = Some(next_time);
                    return true;
                }
            }
            None => {
                // First time checking - find the next scheduled time
                if let Some(next_time) = schedule.upcoming(ChronoLocal).next() {
                    // If the next upcoming time is now or in the past, trigger immediately
                    if now >= next_time {
                        *last_trigger = Some(next_time);
                        return true;
                    } else {
                        // Set the last_trigger to a time just before the next scheduled time
                        // so we can properly track the next occurrence
                        *last_trigger = Some(next_time - chrono::Duration::milliseconds(1));
                    }
                }
            }
        }

        false
    }
}

/// A Bevy plugin that enables cron job functionality.
///
/// This plugin adds the necessary systems to check and trigger cron jobs
/// represented by `ScheduleTimer` components.
pub struct CronJobPlugin;

impl Plugin for CronJobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_schedule_timers);
    }
}

/// A component that represents a scheduled task using cron expressions.
///
/// This component can be attached to entities to create scheduled tasks.
/// When the schedule triggers, a `ScheduleArrived` event will be sent to the entity.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_cronjob::prelude::*;
///
/// fn setup(mut commands: Commands) {
///     commands
///         .spawn(ScheduleTimer::new("every 5 seconds"))
///         .observe(|_: On<ScheduleArrived>| {
///             info!("Timer triggered!");
///         });
/// }
/// ```
#[derive(Debug, Component)]
pub struct ScheduleTimer {
    /// The parsed cron schedule
    schedule: Schedule,
    /// The last time this schedule was triggered
    last_trigger: Option<DateTime<ChronoLocal>>,
}

impl ScheduleTimer {
    /// Creates a new `ScheduleTimer` with the given cron expression.
    ///
    /// # Arguments
    ///
    /// * `expression` - A cron expression string or English description
    ///
    /// # Panics
    ///
    /// Panics if the expression cannot be parsed as a valid cron expression.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bevy_cronjob::ScheduleTimer;
    ///
    /// // Using cron expression
    /// let timer1 = ScheduleTimer::new("0/5 * * * * ? *");
    ///
    /// // Using English expression
    /// let timer2 = ScheduleTimer::new("every 5 seconds");
    /// ```
    pub fn new(expression: &str) -> Self {
        let expression = parse_expression(expression);
        let schedule = Schedule::from_str(&expression).expect("Failed to parse cron expression");

        Self {
            schedule,
            last_trigger: None,
        }
    }

    /// Checks if the schedule should trigger based on the current time.
    ///
    /// This method updates the internal state and returns `true` if the schedule
    /// has triggered since the last check.
    ///
    /// # Returns
    ///
    /// `true` if the schedule should trigger, `false` otherwise.
    fn should_trigger(&mut self) -> bool {
        let now = ChronoLocal::now();

        match self.last_trigger {
            Some(last) => {
                // If we have a previous trigger time, check for the next scheduled time after it
                if let Some(next_time) = self.schedule.after(&last).next()
                    && now >= next_time
                {
                    self.last_trigger = Some(next_time);
                    return true;
                }
            }
            None => {
                // First time checking - find the next scheduled time
                // We check if there's a scheduled time in the past that we missed
                if let Some(next_time) = self.schedule.upcoming(ChronoLocal).next() {
                    // If the next upcoming time is now or in the past, trigger immediately
                    if now >= next_time {
                        self.last_trigger = Some(next_time);
                        return true;
                    } else {
                        // Set the last_trigger to a time just before the next scheduled time
                        // so we can properly track the next occurrence
                        self.last_trigger = Some(next_time - chrono::Duration::milliseconds(1));
                    }
                }
            }
        }

        false
    }
}

/// Parses a cron expression, handling both standard cron syntax and English expressions.
///
/// This function first checks if the expression contains alphabetic characters,
/// indicating it might be an English expression. If so, it attempts to convert
/// it to cron syntax using the `english-to-cron` crate.
///
/// # Arguments
///
/// * `expression` - The expression to parse (cron syntax or English)
///
/// # Returns
///
/// A String containing the parsed cron expression.
///
/// # Examples
///
/// ```rust
/// use bevy_cronjob::parse_expression;
///
/// assert_eq!(parse_expression("0/5 * * * * ? *"), "0/5 * * * * ? *");
/// // Note: This example assumes the english-to-cron conversion works
/// // assert_eq!(parse_expression("every 5 seconds"), "0/5 * * * * ? *");
/// ```
pub fn parse_expression(expression: &str) -> String {
    // Check if the expression contains alphabetic characters (indicating English)
    if expression.chars().any(|c| c.is_ascii_alphabetic()) {
        str_cron_syntax(expression).expect("Failed to parse English cron expression")
    } else {
        expression.to_string()
    }
}

/// System that checks all `ScheduleTimer` components and triggers events for schedules that should execute.
///
/// This system runs every frame and checks each entity with a `ScheduleTimer` component.
/// If any schedule should trigger, it sends a `ScheduleArrived` event to that entity.
///
/// The system is optimized to batch all triggered entities and send events in a single operation.
fn check_schedule_timers(mut query: Query<(Entity, &mut ScheduleTimer)>, mut commands: Commands) {
    // Collect all entities that should trigger to batch the event sending
    let triggered_entities: Vec<Entity> = query
        .iter_mut()
        .filter_map(|(entity, mut timer)| {
            if timer.should_trigger() {
                Some(entity)
            } else {
                None
            }
        })
        .collect();

    // Send events to all triggered entities individually
    for entity in triggered_entities {
        commands.trigger(ScheduleArrived { entity });
    }
}

/// Event sent when a scheduled task should execute.
///
/// This event is triggered automatically by the `CronJobPlugin` when a `ScheduleTimer`
/// component's schedule is ready to execute. The event is sent as a targeted event
/// to the specific entity that owns the `ScheduleTimer`.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_cronjob::prelude::*;
///
/// fn handle_schedule(trigger: On<ScheduleArrived>) {
///     info!("Schedule triggered for entity: {:?}", trigger.target());
/// }
/// ```
#[derive(EntityEvent)]
pub struct ScheduleArrived {
    #[event_target]
    pub entity: Entity,
}

/// Convenient re-exports for common functionality.
pub mod prelude {
    pub use crate::{CronJobPlugin, ScheduleArrived, ScheduleTimer, schedule_passed};
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that all predefined constants match their English equivalents.
    #[test]
    fn test_expression_constants() {
        assert_eq!(EVERY_5_SEC, str_cron_syntax("every 5 seconds").unwrap());
        assert_eq!(EVERY_10_SEC, str_cron_syntax("every 10 seconds").unwrap());
        assert_eq!(EVERY_30_SEC, str_cron_syntax("every 30 seconds").unwrap());
        assert_eq!(EVERY_MIN, str_cron_syntax("every minute").unwrap());
        assert_eq!(EVERY_5_MIN, str_cron_syntax("every 5 minutes").unwrap());
        assert_eq!(EVERY_10_MIN, str_cron_syntax("every 10 minutes").unwrap());
        assert_eq!(EVERY_30_MIN, str_cron_syntax("every 30 minutes").unwrap());
        assert_eq!(EVERY_HOUR, str_cron_syntax("every hour").unwrap());
        assert_eq!(EVERY_DAY, str_cron_syntax("every day").unwrap());

        // Test a few specific times
        assert_eq!(EVERY_1_AM, str_cron_syntax("every day at 1 am").unwrap());
        assert_eq!(EVERY_12_PM, str_cron_syntax("every day at 12 pm").unwrap());
        assert_eq!(EVERY_11_PM, str_cron_syntax("every day at 11 pm").unwrap());
    }

    /// Test that expression parsing works for both cron and English syntax.
    #[test]
    fn test_expression_parsing() {
        // Test cron expression pass-through
        let cron_expr = "0/5 * * * * ? *";
        assert_eq!(parse_expression(cron_expr), cron_expr);

        // Test English expression conversion
        let english_expr = "every 5 seconds";
        let parsed = parse_expression(english_expr);
        assert!(parsed.contains("0/5"));
    }

    /// Test that ScheduleTimer can be created with various expressions.
    #[test]
    fn test_schedule_timer_creation() {
        // Should not panic with valid expressions
        let _timer1 = ScheduleTimer::new("0/5 * * * * ? *");
        let _timer2 = ScheduleTimer::new("every minute");
    }

    /// Test that invalid expressions cause panics as expected.
    #[test]
    #[should_panic(expected = "Failed to parse English cron expression")]
    fn test_invalid_cron_expression() {
        let _timer = ScheduleTimer::new("invalid cron expression");
    }

    /// Test that ScheduleTimer triggers correctly on the first check.
    #[test]
    fn test_schedule_timer_first_trigger() {
        // Create a timer that should trigger every second
        let mut timer = ScheduleTimer::new("* * * * * ? *");

        // The first call to should_trigger() should set up the state properly
        // and potentially trigger if we're on a second boundary
        let first_result = timer.should_trigger();

        // After the first call, last_trigger should be set
        assert!(timer.last_trigger.is_some());

        // If it didn't trigger on the first call, it should be because we're not
        // at a second boundary. Let's verify the logic works by checking again
        // after a small delay simulation by manually setting the last_trigger to past
        if !first_result {
            timer.last_trigger = Some(ChronoLocal::now() - chrono::Duration::seconds(2));
            let second_result = timer.should_trigger();
            assert!(
                second_result,
                "Timer should trigger when last_trigger is in the past"
            );
        }
    }

    /// Test that the schedule_passed function works correctly.
    /// This test verifies the schedule logic by testing the underlying Schedule directly.
    #[test]
    fn test_schedule_passed_logic() {
        use cron::Schedule;
        use std::str::FromStr;

        // Test with a schedule that should trigger every second
        let schedule = Schedule::from_str("* * * * * ? *").unwrap();
        let now = ChronoLocal::now();

        // Test the upcoming method works
        let next_upcoming = schedule.upcoming(ChronoLocal).next();
        assert!(
            next_upcoming.is_some(),
            "Schedule should have upcoming times"
        );

        // Test the after method works
        let past_time = now - chrono::Duration::seconds(2);
        let next_after = schedule.after(&past_time).next();
        assert!(
            next_after.is_some(),
            "Schedule should have times after past time"
        );

        // Verify that upcoming and after can return different results
        if let (Some(_upcoming), Some(after)) = (next_upcoming, next_after) {
            // The next upcoming time should generally be >= now
            // The next after past_time should be > past_time
            assert!(
                after > past_time,
                "After time should be greater than reference time"
            );
        }
    }
}
