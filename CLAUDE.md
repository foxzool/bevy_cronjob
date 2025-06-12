# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Commands

- **Build the project**: `cargo build`
- **Run tests**: `cargo test`
- **Run example**: `cargo run --example cronjobs`
- **Check code formatting**: `cargo fmt --check`
- **Format code**: `cargo fmt`
- **Run linter**: `cargo clippy`
- **Generate documentation**: `cargo doc --open`

## Architecture Overview

This is a Bevy plugin crate that provides cronjob scheduling functionality. The core architecture consists of:

### Main Components
- **CronJobPlugin**: The main plugin that registers the schedule checking system
- **ScheduleTimer**: A component that holds cron expressions and tracks schedule state
- **ScheduleArrived**: An event triggered when a scheduled time passes
- **schedule_passed()**: A run condition function for systems

### Two Usage Patterns
1. **Run Condition Pattern**: Use `schedule_passed("expression")` as a run condition for systems
2. **Observer Pattern**: Spawn `ScheduleTimer` components and observe `ScheduleArrived` events

### Expression Support
- Standard cron expressions: `"0/5 * * * * ? *"` (every 5 seconds)
- English patterns via english-to-cron: `"every 5 seconds"`, `"every hour"`
- Predefined constants: `EVERY_5_SEC`, `EVERY_MIN`, `EVERY_HOUR`, etc.

### Time Handling
- Uses `chrono::Local` time (not UTC as mentioned in README)
- Schedule checking happens via `check_schedule_timer` system in Update schedule
- State is tracked per-schedule to avoid duplicate triggers

The plugin integrates with Bevy's ECS by using Local resources for state management in run conditions and the observer pattern for entity-based scheduling.