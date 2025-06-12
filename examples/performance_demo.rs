/// Performance demonstration example showing optimized scheduling features.
///
/// This example demonstrates:
/// - Efficient batch processing of multiple schedules
/// - Different scheduling patterns
/// - Performance-optimized timer management
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_app::ScheduleRunnerPlugin;
use bevy_cronjob::prelude::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
        )
        .add_plugins(LogPlugin::default())
        .add_plugins(CronJobPlugin)
        .add_systems(Startup, setup_performance_demo)
        // High-frequency systems using run conditions
        .add_systems(
            Update,
            high_frequency_task.run_if(schedule_passed("every 2 seconds")),
        )
        .add_systems(
            Update,
            medium_frequency_task.run_if(schedule_passed("every 10 seconds")),
        )
        .add_systems(
            Update,
            low_frequency_task.run_if(schedule_passed("every 30 seconds")),
        )
        .run();
}

/// Setup multiple schedule timers to demonstrate batch processing efficiency.
fn setup_performance_demo(mut commands: Commands) {
    info!("Setting up performance demonstration with multiple schedules...");

    // Create 10 entities with different schedules to show batch processing
    for i in 1..=10 {
        let schedule = match i % 4 {
            0 => "every 3 seconds",
            1 => "every 5 seconds",
            2 => "every 7 seconds",
            _ => "every 11 seconds",
        };

        commands
            .spawn((
                ScheduleTimer::new(schedule),
                Name::new(format!("Timer-{}", i)),
            ))
            .observe(move |trigger: Trigger<ScheduleArrived>| {
                info!(
                    "Batch timer {} triggered for entity {:?}",
                    i,
                    trigger.target()
                );
            });
    }

    // Create a complex schedule using cron expression
    commands
        .spawn((
            ScheduleTimer::new("0/13 * * * * ? *"), // Every 13 seconds
            Name::new("Complex-Timer"),
        ))
        .observe(|trigger: Trigger<ScheduleArrived>| {
            info!(
                "Complex cron schedule triggered for entity {:?}",
                trigger.target()
            );
        });

    // Create schedules with English expressions
    let english_schedules = vec!["every 15 seconds", "every 20 seconds", "every 25 seconds"];

    for (idx, schedule) in english_schedules.into_iter().enumerate() {
        commands
            .spawn((
                ScheduleTimer::new(schedule),
                Name::new(format!("English-Timer-{}", idx + 1)),
            ))
            .observe(move |trigger: Trigger<ScheduleArrived>| {
                info!(
                    "English schedule '{}' triggered for entity {:?}",
                    schedule,
                    trigger.target()
                );
            });
    }

    info!("Performance demo setup complete! Watch for efficient batch processing.");
}

/// High-frequency task that runs every 2 seconds using run conditions.
fn high_frequency_task() {
    info!("[HIGH FREQ] Task executed - runs every 2 seconds");
}

/// Medium-frequency task that runs every 10 seconds using run conditions.
fn medium_frequency_task() {
    info!("[MEDIUM FREQ] Task executed - runs every 10 seconds");
}

/// Low-frequency task that runs every 30 seconds using run conditions.
fn low_frequency_task() {
    info!("[LOW FREQ] Task executed - runs every 30 seconds");
}
