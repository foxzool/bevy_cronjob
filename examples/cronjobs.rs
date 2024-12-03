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
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            print_per_5_sec.run_if(schedule_passed("every 5 seconds")),
        )
        .add_systems(
            Update,
            print_per_min.run_if(schedule_passed("every 1 minute")),
        )
        .add_systems(Update, print_per_hour.run_if(schedule_passed("every hour")))
        .run();
}

fn print_per_5_sec() {
    info!("system run every 5 sec")
}

fn print_per_min() {
    info!("system run every minute")
}

fn print_per_hour() {
    info!("system run every hour")
}

fn setup(mut commands: Commands) {
    commands
        .spawn(ScheduleTimer::new("every 3 seconds"))
        .observe(|_: Trigger<ScheduleArrived>| {
            info!("3 seconds passed");
        });
}
