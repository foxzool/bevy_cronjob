/// Test example to verify that schedule triggers work correctly from the start.
///
/// This example demonstrates that the fixed logic properly handles:
/// - First-time initialization of schedules
/// - Immediate triggering when appropriate
/// - Proper state management across multiple checks
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_app::ScheduleRunnerPlugin;
use bevy_cronjob::prelude::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 10.0, // Run at 10 FPS for easier observation
            ))),
        )
        .add_plugins(LogPlugin::default())
        .add_plugins(CronJobPlugin)
        .add_systems(Startup, setup_trigger_tests)
        // Test run conditions that should trigger immediately or soon
        .add_systems(
            Update,
            test_every_second.run_if(schedule_passed("* * * * * ? *")),
        )
        .add_systems(
            Update,
            test_every_2_seconds.run_if(schedule_passed("0/2 * * * * ? *")),
        )
        .run();
}

/// Setup schedule timers that should trigger quickly for testing.
fn setup_trigger_tests(mut commands: Commands) {
    info!("Setting up trigger tests...");
    info!("Watch for immediate or quick triggering of schedules.");

    // Timer that should trigger every second
    commands
        .spawn((
            ScheduleTimer::new("* * * * * ? *"),
            Name::new("Every-Second-Timer"),
        ))
        .observe(|trigger: Trigger<ScheduleArrived>| {
            info!(
                "✅ Every-second timer triggered for entity {:?}",
                trigger.target()
            );
        });

    // Timer that should trigger every 2 seconds
    commands
        .spawn((
            ScheduleTimer::new("0/2 * * * * ? *"),
            Name::new("Every-2-Seconds-Timer"),
        ))
        .observe(|trigger: Trigger<ScheduleArrived>| {
            info!(
                "✅ Every-2-seconds timer triggered for entity {:?}",
                trigger.target()
            );
        });

    // Timer using English expression
    commands
        .spawn((
            ScheduleTimer::new("every 3 seconds"),
            Name::new("Every-3-Seconds-English"),
        ))
        .observe(|trigger: Trigger<ScheduleArrived>| {
            info!(
                "✅ Every-3-seconds (English) timer triggered for entity {:?}",
                trigger.target()
            );
        });

    info!("Trigger test setup complete! Schedules should start triggering soon.");
}

/// Test system using run condition - should trigger every second.
fn test_every_second() {
    info!("🔥 Run condition: Every second system triggered");
}

/// Test system using run condition - should trigger every 2 seconds.
fn test_every_2_seconds() {
    info!("🔥 Run condition: Every 2 seconds system triggered");
}
