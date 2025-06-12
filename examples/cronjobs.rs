use bevy::{log::LogPlugin, prelude::*};
use bevy_app::ScheduleRunnerPlugin;
use bevy_cronjob::prelude::*;
use std::time::Duration;

fn main() {
    App::new()
        // Configure minimal plugins with 60 FPS for demonstration
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
        )
        // Add logging support
        .add_plugins(LogPlugin::default())
        // Add the cron job plugin
        .add_plugins(CronJobPlugin)
        // Setup initial entities and observers
        .add_systems(Startup, setup)
        // Add systems that run based on schedule conditions
        .add_systems(
            Update,
            print_every_5_seconds.run_if(schedule_passed("every 5 seconds")),
        )
        .add_systems(
            Update,
            print_every_minute.run_if(schedule_passed("every 1 minute")),
        )
        .add_systems(
            Update,
            print_every_hour.run_if(schedule_passed("every hour")),
        )
        .run();
}

/// System that runs every 5 seconds using a schedule condition.
fn print_every_5_seconds() {
    info!("System executed: every 5 seconds");
}

/// System that runs every minute using a schedule condition.
fn print_every_minute() {
    info!("System executed: every minute");
}

/// System that runs every hour using a schedule condition.
fn print_every_hour() {
    info!("System executed: every hour");
}

/// Setup function that creates entities with schedule timers and observers.
///
/// This demonstrates the component-based approach using `ScheduleTimer` components
/// and observer systems that respond to `ScheduleArrived` events.
fn setup(mut commands: Commands) {
    info!("Setting up cron job entities...");

    // Create an entity with a 3-second timer using an observer
    commands
        .spawn(ScheduleTimer::new("every 3 seconds"))
        .observe(|trigger: Trigger<ScheduleArrived>| {
            info!("Entity {:?}: 3 seconds have passed", trigger.target());
        });

    // Create an entity with a 10-second timer using a named observer function
    commands
        .spawn(ScheduleTimer::new("every 10 seconds"))
        .observe(handle_ten_second_timer);

    // Create an entity with a specific time schedule (every day at a specific time)
    // Note: This will only trigger once per day at the specified time
    commands
        .spawn(ScheduleTimer::new("every day at 9 am"))
        .observe(|trigger: Trigger<ScheduleArrived>| {
            info!("Entity {:?}: Daily 9 AM task executed", trigger.target());
        });

    // Create an entity with a custom cron expression (every 7 seconds)
    commands
        .spawn(ScheduleTimer::new("0/7 * * * * ? *"))
        .observe(|trigger: Trigger<ScheduleArrived>| {
            info!(
                "Entity {:?}: Custom 7-second cron job executed",
                trigger.target()
            );
        });

    info!("Cron job entities created successfully!");
}

/// Named observer function for handling 10-second timer events.
///
/// This demonstrates how to use a named function as an observer,
/// which can be useful for more complex logic or reusable handlers.
fn handle_ten_second_timer(trigger: Trigger<ScheduleArrived>) {
    info!(
        "Named observer triggered for entity {:?}: 10 seconds elapsed",
        trigger.target()
    );

    // You can add more complex logic here, such as:
    // - Querying other components on the same entity
    // - Modifying the entity's components
    // - Spawning new entities
    // - Triggering other events
}
