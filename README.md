[![crates.io](https://img.shields.io/crates/v/bevy_cronjob)](https://crates.io/crates/bevy_cronjob)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/seldom_pixel#license)
[![crates.io](https://img.shields.io/crates/d/bevy_cronjob)](https://crates.io/crates/bevy_cronjob)
[![Documentation](https://docs.rs/bevy_cronjob/badge.svg)](https://docs.rs/bevy_cronjob)

# bevy_cronjob

`bevy_cronjob` is a simple helper to run cronjobs (at repeated schedule) in Bevy.

## Usage

``` rust
use std::time::Duration;
use bevy::{ MinimalPlugins};
use bevy::app::{App, PluginGroup, ScheduleRunnerPlugin, Update};
use bevy::log::{info, LogPlugin};

use bevy_ecs::prelude::{IntoSystemConfigs};
use bevy_cronjob::schedule_passed;

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
        )
        .add_plugins(LogPlugin::default())
        .add_systems(Update, print_per_5_sec.run_if(schedule_passed("0/5 * * * * *")))
        .add_systems(Update, print_per_min.run_if(schedule_passed("0 * * * * *")))
        .add_systems(Update, print_per_hour.run_if(schedule_passed("0 0 * * * *")))
        .run()
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
```

## Expression

the scheduling expression is base on [cron](https://github.com/zslayton/cron)

| sec  | min  | hour | day of month | month | day of week | year      |
|------|------|------|--------------|-------|-------------|-----------|
| *    | *    | *    | *            | *     | *           | *         |
| 0-59 | 0-59 | 0-23 | 1-23         | 1-12  | 1-7         | 1970-2100 |

Time is specified in UTC. Note that the year may be omitted.

Comma separated values such as `1,2,3` are allowed. For example, a schedule of `0,15,30,45 * * * * *`' would execute on every 15 seconds.

Ranges can be specified with a dash. For example `1-5 * * * * *`' would execute on every second for the first 5 seconds of a minute.

## Supported Versions

| bevy | bevy_cronjob |
|------|--------------|
| 0.11 | 0.1          |

## License

Dual-licensed under either

- MIT
- Apache 2.0
