[![crates.io](https://img.shields.io/crates/v/bevy_cronjob)](https://crates.io/crates/bevy_cronjob)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/foxzool/bevy_cronjob#license)
[![CI](https://github.com/foxzool/bevy_cronjob/workflows/CI/badge.svg)](https://github.com/foxzool/bevy_cronjob/actions)
[![Documentation](https://docs.rs/bevy_cronjob/badge.svg)](https://docs.rs/bevy_cronjob)

# bevy_cronjob

A simple, efficient, and reliable helper for running scheduled tasks (cronjobs) in Bevy applications.

## ✨ Features

- 🕐 **Flexible Scheduling**: Support for both cron expressions and natural English descriptions
- 🎯 **Dual Usage Patterns**: System run conditions or component-based scheduling with observers
- 📅 **Local Timezone**: All schedules operate in local timezone for intuitive behavior
- 🚀 **Performance Optimized**: Efficient batching, minimal allocations, and smart state management
- 🔧 **Easy Integration**: Simple plugin setup with comprehensive examples
- 🛡️ **Reliable**: Fixed initialization bugs ensuring schedules work from the first frame
- 📚 **Well Documented**: Extensive documentation with practical examples

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
bevy_cronjob = "0.6"
```

Basic usage:

```rust
use bevy::prelude::*;
use bevy_cronjob::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CronJobPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            my_system.run_if(schedule_passed("every 5 seconds")),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(ScheduleTimer::new("every 10 seconds"))
        .observe(|_: Trigger<ScheduleArrived>| {
            info!("Timer triggered!");
        });
}

fn my_system() {
    info!("This runs every 5 seconds");
}
```

## 📋 Usage Patterns

### Pattern 1: System Run Conditions (Simple & Lightweight)

Perfect for simple, stateless scheduled tasks:

```rust
use bevy_cronjob::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CronJobPlugin)
        .add_systems(
            Update,
            (
                save_game.run_if(schedule_passed("every 30 seconds")),
                cleanup_cache.run_if(schedule_passed("every 5 minutes")),
                daily_backup.run_if(schedule_passed("every day at 3 am")),
            ),
        )
        .run();
}

fn save_game() {
    info!("Auto-saving game...");
}

fn cleanup_cache() {
    info!("Cleaning up cache...");
}

fn daily_backup() {
    info!("Running daily backup...");
}
```

**Pros**: Minimal setup, integrates seamlessly with Bevy's scheduling
**Cons**: Each run condition maintains separate state

### Pattern 2: Component-Based with Observers (Flexible & Powerful)

Ideal for complex scheduling needs with entity-specific logic:

```rust
use bevy_cronjob::prelude::*;

#[derive(Component)]
struct GameStats {
    score: u32,
    level: u32,
}

fn setup(mut commands: Commands) {
    // Create entities with different schedules
    commands
        .spawn((
            ScheduleTimer::new("every 1 minute"),
            GameStats { score: 0, level: 1 },
            Name::new("Score Reporter"),
        ))
        .observe(report_score);

    commands
        .spawn((
            ScheduleTimer::new("every 10 seconds"),
            Name::new("Health Monitor"),
        ))
        .observe(check_player_health);
}

fn report_score(
    trigger: Trigger<ScheduleArrived>,
    query: Query<&GameStats>,
) {
    if let Ok(stats) = query.get(trigger.entity()) {
        info!("Current score: {}, Level: {}", stats.score, stats.level);
    }
}

fn check_player_health(trigger: Trigger<ScheduleArrived>) {
    info!("Checking player health for entity: {:?}", trigger.entity());
}
```

**Pros**: Entity-specific data, flexible event handling, better for complex logic
**Cons**: Slightly more setup required

## 📅 Schedule Expression Formats

### Cron Expressions

Standard cron syntax based on the [cron](https://github.com/zslayton/cron) crate:

| Field        | Values    | Description                    |
|--------------|-----------|--------------------------------|
| Second       | 0-59      | Seconds                        |
| Minute       | 0-59      | Minutes                        |
| Hour         | 0-23      | Hours (24-hour format)         |
| Day of Month | 1-31      | Day of the month               |
| Month        | 1-12      | Month (1=January, 12=December) |
| Day of Week  | 1-7       | Day of the week (1=Monday)     |
| Year         | 1970-2100 | Year (optional)                |

**Special Characters**:

- `*` - Any value
- `?` - No specific value (for day fields)
- `/` - Step values (e.g., `0/5` = every 5 units)
- `,` - List separator (e.g., `1,3,5`)
- `-` - Range (e.g., `1-5`)

**Examples**:

```rust
"0/5 * * * * ? *"       // Every 5 seconds
"0 * * * * ? *"         // Every minute at second 0
"0 0 * * * ? *"         // Every hour at minute 0
"0 0 0 * * ? *"         // Every day at midnight
"0 0 9 * * MON-FRI *"   // Every weekday at 9 AM
"0 0 0 1 * ? *"         // First day of every month
"0 30 14 * * ? *"       // Every day at 2:30 PM
```

### English Expressions

Natural language scheduling powered by [english-to-cron](https://github.com/kaplanelad/english-to-cron):

| English Expression                             | Equivalent Cron    | Description                |
|------------------------------------------------|--------------------|----------------------------|
| `every 15 seconds`                             | `0/15 * * * * ? *` | Every 15 seconds           |
| `every minute`                                 | `0 * * * * ? *`    | Every minute               |
| `every hour`                                   | `0 0 * * * ? *`    | Every hour                 |
| `every day`                                    | `0 0 0 */1 * ? *`  | Every day at midnight      |
| `every day at 4:00 pm`                         | `0 0 16 */1 * ? *` | Every day at 4 PM          |
| `at 10:00 am`                                  | `0 0 10 * * ? *`   | Every day at 10 AM         |
| `run at midnight on the 1st and 15th of month` | `0 0 0 1,15 * ? *` | 1st and 15th of each month |
| `On Sunday at 12:00`                           | `0 0 12 ? * SUN *` | Every Sunday at noon       |
| `7pm every Thursday`                           | `0 0 19 ? * THU *` | Every Thursday at 7 PM     |
| `midnight on Tuesdays`                         | `0 0 0 ? * TUE *`  | Every Tuesday at midnight  |

## 🔧 Predefined Constants

Common schedules are available as constants for convenience:

```rust
use bevy_cronjob::*;

// Frequent intervals
EVERY_5_SEC     // "0/5 * * * * ? *"
EVERY_10_SEC    // "0/10 * * * * ? *"
EVERY_30_SEC    // "0/30 * * * * ? *"
EVERY_MIN       // "0 * * * * ? *"
EVERY_5_MIN     // "0 0/5 * * * ? *"
EVERY_30_MIN    // "0 0/30 * * * ? *"
EVERY_HOUR      // "0 0 * * * ? *"
EVERY_DAY       // "0 0 0 */1 * ? *"

// Specific daily times
EVERY_1_AM      // "0 0 1 */1 * ? *"
EVERY_6_AM      // "0 0 6 */1 * ? *"
EVERY_12_PM     // "0 0 12 */1 * ? *"
EVERY_6_PM      // "0 0 18 */1 * ? *"
EVERY_11_PM     // "0 0 23 */1 * ? *"
// ... and many more
```

Usage:

```rust
.add_systems(
Update,
backup_system.run_if(schedule_passed(EVERY_DAY)),
)
```

## 🎯 Advanced Examples

### Game-Specific Scheduling

```rust
use bevy_cronjob::prelude::*;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct SpawnTimer;

fn setup_game_schedules(mut commands: Commands) {
    // Spawn enemies every 30 seconds
    commands
        .spawn((
            ScheduleTimer::new("every 30 seconds"),
            SpawnTimer,
            Name::new("Enemy Spawner"),
        ))
        .observe(spawn_enemy_wave);

    // Save game progress every 5 minutes
    commands
        .spawn(ScheduleTimer::new("every 5 minutes"))
        .observe(|_: Trigger<ScheduleArrived>| {
            info!("Auto-saving game progress...");
            // Save game logic here
        });

    // Daily challenges reset at midnight
    commands
        .spawn(ScheduleTimer::new("every day at 12 am"))
        .observe(reset_daily_challenges);

    // Weekend bonus events
    commands
        .spawn(ScheduleTimer::new("0 0 18 ? * FRI *")) // Friday 6 PM
        .observe(|_: Trigger<ScheduleArrived>| {
            info!("Weekend bonus event started!");
        });
}

fn spawn_enemy_wave(
    trigger: Trigger<ScheduleArrived>,
    mut commands: Commands,
) {
    info!("Spawning enemy wave for spawner: {:?}", trigger.entity());

    // Spawn multiple enemies
    for i in 0..5 {
        commands.spawn((
            Enemy,
            Name::new(format!("Enemy-{}", i)),
            // Add enemy components...
        ));
    }
}

fn reset_daily_challenges(trigger: Trigger<ScheduleArrived>) {
    info!("Resetting daily challenges...");
    // Reset challenge progress
}
```

### Server Maintenance Scheduling

```rust
use bevy_cronjob::prelude::*;

fn setup_server_maintenance(mut commands: Commands) {
    // Log server stats every minute
    commands
        .spawn(ScheduleTimer::new("every minute"))
        .observe(log_server_stats);

    // Clean up disconnected players every 5 minutes
    commands
        .spawn(ScheduleTimer::new("every 5 minutes"))
        .observe(cleanup_disconnected_players);

    // Database backup every day at 3 AM
    commands
        .spawn(ScheduleTimer::new("every day at 3 am"))
        .observe(backup_database);

    // Weekly server restart (Sunday 4 AM)
    commands
        .spawn(ScheduleTimer::new("0 0 4 ? * SUN *"))
        .observe(schedule_server_restart);
}

fn log_server_stats(trigger: Trigger<ScheduleArrived>) {
    info!("Server uptime check - Entity: {:?}", trigger.entity());
    // Log memory usage, player count, etc.
}

fn cleanup_disconnected_players(trigger: Trigger<ScheduleArrived>) {
    info!("Cleaning up disconnected players...");
    // Remove inactive player entities
}

fn backup_database(trigger: Trigger<ScheduleArrived>) {
    info!("Starting database backup...");
    // Backup logic
}

fn schedule_server_restart(trigger: Trigger<ScheduleArrived>) {
    info!("Scheduling server restart for maintenance...");
    // Graceful restart logic
}
```

### Dynamic Scheduling

```rust
use bevy_cronjob::prelude::*;

#[derive(Component)]
struct DifficultyLevel(u32);

fn setup_dynamic_scheduling(mut commands: Commands) {
    commands.spawn((
        DifficultyLevel(1),
        Name::new("Game Manager"),
    ));
}

fn adjust_spawn_rate(
    mut commands: Commands,
    query: Query<(Entity, &DifficultyLevel)>,
    existing_timers: Query<Entity, With<ScheduleTimer>>,
) {
    for (entity, difficulty) in query.iter() {
        // Remove old spawn timers
        for timer_entity in existing_timers.iter() {
            commands.entity(timer_entity).despawn();
        }

        // Create new timer based on difficulty
        let spawn_interval = match difficulty.0 {
            1 => "every 30 seconds",
            2 => "every 20 seconds",
            3 => "every 10 seconds",
            4 => "every 5 seconds",
            _ => "every 2 seconds",
        };

        commands
            .spawn(ScheduleTimer::new(spawn_interval))
            .observe(move |_: Trigger<ScheduleArrived>| {
                info!("Spawning enemies at difficulty level {}", difficulty.0);
            });
    }
}
```

## 🛡️ Error Handling and Validation

### Safe Schedule Creation

```rust
use cron::Schedule;
use std::str::FromStr;
use bevy_cronjob::*;

fn safe_schedule_creation(expression: &str) -> Result<ScheduleTimer, String> {
    // Parse expression first (handles English to cron conversion)
    let cron_expr = if expression.chars().any(|c| c.is_ascii_alphabetic()) {
        str_cron_syntax(expression)
            .map_err(|e| format!("Invalid English expression '{}': {}", expression, e))?
    } else {
        expression.to_string()
    };

    // Validate cron expression
    Schedule::from_str(&cron_expr)
        .map_err(|e| format!("Invalid cron expression '{}': {}", cron_expr, e))?;

    Ok(ScheduleTimer::new(expression))
}

fn setup_with_validation(mut commands: Commands) {
    match safe_schedule_creation("every 5 seconds") {
        Ok(timer) => {
            commands
                .spawn(timer)
                .observe(|_: Trigger<ScheduleArrived>| {
                    info!("Safe timer triggered!");
                });
        }
        Err(e) => {
            error!("Failed to create schedule: {}", e);
        }
    }
}
```

### Runtime Schedule Management

```rust
use bevy_cronjob::prelude::*;

#[derive(Component)]
struct ScheduleConfig {
    expression: String,
    enabled: bool,
}

fn manage_schedules_system(
    mut commands: Commands,
    config_query: Query<(Entity, &ScheduleConfig), Changed<ScheduleConfig>>,
    timer_query: Query<Entity, With<ScheduleTimer>>,
) {
    for (entity, config) in config_query.iter() {
        // Remove existing timer if any
        if let Ok(timer_entity) = timer_query.get(entity) {
            commands.entity(timer_entity).despawn();
        }

        // Add new timer if enabled
        if config.enabled {
            match safe_schedule_creation(&config.expression) {
                Ok(timer) => {
                    commands.entity(entity).insert(timer);
                }
                Err(e) => {
                    warn!("Invalid schedule for entity {:?}: {}", entity, e);
                }
            }
        }
    }
}
```

## 🚀 Performance Optimizations

The crate includes several performance optimizations for production use:

### 1. Fixed Initialization Logic

- **Problem**: Previous versions had a bug where schedules wouldn't trigger on the first check
- **Solution**: Proper state initialization handling for immediate and future triggers

### 2. Efficient Time Calculations

- Uses optimized schedule queries for better performance
- Smart state management to minimize unnecessary calculations

### 3. Batched Event Processing

- Multiple triggered schedules are processed in a single batch operation
- Reduces system call overhead for high-frequency schedules

### 4. Memory Efficient

- Expression parsing done once during setup, not on every check
- Minimal allocations during runtime execution

### 5. Scalable Architecture

- Handles many concurrent schedules efficiently
- Iterator-based processing for better CPU cache usage

## 🧪 Testing

The crate includes comprehensive tests. Run them with:

```bash
cargo test
```

For manual testing, run the examples:

```bash
# Basic functionality
cargo run --example cronjobs

# Performance demonstration
cargo run --example performance_demo

# Trigger testing
cargo run --example trigger_test
```

## 🔄 Migration Guide

### From v0.5.x to v0.6.x

No breaking changes! The API remains the same, but with important improvements:

1. **Bug Fix**: Schedules now work correctly from the first frame
2. **Performance**: Better batching and time calculations
3. **Documentation**: Comprehensive English documentation

Simply update your `Cargo.toml`:

```toml
[dependencies]
bevy_cronjob = "0.6"
```

## 📊 Supported Bevy Versions

| Bevy Version | bevy_cronjob Version | 
|--------------|----------------------|
| 0.16         | 0.6                  | 
| 0.15         | 0.5                  | 
| 0.14         | 0.4                  | 
| 0.13         | 0.3                  | 

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Report Bugs**: Open an issue with a clear description and minimal reproduction case
2. **Suggest Features**: Propose new features with use cases and API design
3. **Submit PRs**: Fork, create a feature branch, and submit a pull request
4. **Improve Docs**: Help make the documentation even better

### Development Setup

```bash
git clone https://github.com/foxzool/bevy_cronjob.git
cd bevy_cronjob
cargo test
cargo run --example cronjobs
```

### Guidelines

- Follow Rust naming conventions
- Add tests for new features
- Update documentation for API changes
- Keep performance in mind for runtime code

## 📜 License

This project is dual-licensed under either:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- **Apache License 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license when using this crate in your projects.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

## 📚 Additional Resources

- [Cron Expression Reference](https://crontab.guru/) - Interactive cron expression builder
- [Bevy Documentation](https://bevyengine.org/learn/) - Learn more about Bevy
- [Examples Directory](examples/) - More usage examples
- [API Documentation](https://docs.rs/bevy_cronjob) - Complete API reference

---

**Made with ❤️ for the Bevy community**
