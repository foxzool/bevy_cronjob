[![crates.io](https://img.shields.io/crates/v/bevy_cronjob)](https://crates.io/crates/bevy_cronjob)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/seldom_pixel#license)
[![CI](https://github.com/foxzool/bevy_cronjob/workflows/CI/badge.svg)](https://github.com/foxzool/bevy_cronjob/actions)
[![Documentation](https://docs.rs/bevy_cronjob/badge.svg)](https://docs.rs/bevy_cronjob)

# bevy_cronjob

`bevy_cronjob` is a simple helper to run cronjob (at repeated schedule) in Bevy.

## Usage

``` rust, no_run
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

```

## Expression

the scheduling expression is base on [cron](https://github.com/zslayton/cron)

| sec  | min  | hour | day of month | month | day of week | year      |
|------|------|------|--------------|-------|-------------|-----------|
| *    | *    | *    | *            | *     | *           | *         |
| 0-59 | 0-59 | 0-23 | 1-23         | 1-12  | 1-7         | 1970-2100 |

Time is specified in UTC. Note that the year may be omitted.

Comma separated values such as `1,2,3` are allowed. For example, a schedule of `0,15,30,45 * * * * *`' would execute on
every 15 seconds.

Ranges can be specified with a dash. For example `1-5 * * * * *`' would execute on every second for the first 5 seconds
of a minute.

## Full List of Supported English Patterns

supported by [english-to-cron](https://github.com/kaplanelad/english-to-cron)

| English Phrase                                   | CronJob Syntax   |
|--------------------------------------------------|------------------|
| every 15 seconds                                 | 0/15 * * * * ? * |
| run every minute                                 | 0 * * * * ? *    |
| fire every day at 4:00 pm                        | 0 0 16 */1 * ? * |
| at 10:00 am                                      | 0 0 10 * * ? *   |
| run at midnight on the 1st and 15th of the month | 0 0 0 1,15 * ? * |
| On Sunday at 12:00                               | 0 0 12 ? * SUN * |
| 7pm every Thursday                               | 0 0 19 ? * THU * |
| midnight on Tuesdays                             | 0 0 ? * TUE *    |

## Supported Versions

| bevy | bevy_cronjob |
|------|--------------|
| 0.15 | 0.5          |
| 0.14 | 0.4          |
| 0.13 | 0.3          |
| 0.12 | 0.2          |
| 0.11 | 0.1          |

## License

Dual-licensed under either

- MIT
- Apache 2.0
