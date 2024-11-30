use std::time::Duration;

use bevy::{
    app::{App, PluginGroup, ScheduleRunnerPlugin, Update},
    log::{info, LogPlugin},
    MinimalPlugins,
};
use bevy_ecs::prelude::IntoSystemConfigs;

use bevy_cronjob::{schedule_passed};

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
        )
        .add_plugins(LogPlugin::default())
        .add_systems(
            Update,
            print_per_5_sec.run_if(schedule_passed("every 5 seconds")),
        )
        .add_systems(Update, print_per_min.run_if(schedule_passed("every 1 minute")))
        .add_systems(Update, print_per_hour.run_if(schedule_passed("every hour")))
        .run();
}

fn print_per_5_sec() {
    info!("print every 5 sec")
}

fn print_per_min() {
    info!("print every minute")
}

fn print_per_hour() {
    info!("print every hour")
}
