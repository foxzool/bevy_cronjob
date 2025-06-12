# Changelog

## [0.6.1] - 2025-01-06 - Performance Optimizations and Critical Bug Fix

### Fixed
- **Critical Bug Fix**: Fixed schedule initialization logic where timers wouldn't trigger on first check
  - Previous logic used `schedule.after(now)` even for initial checks, which would never trigger
  - Now properly handles first-time initialization with correct state management
  - Schedules now work correctly from the first frame
- Improved time calculation accuracy by using appropriate schedule methods
- Better handling of edge cases in schedule triggering
- More robust expression parsing with clearer error messages

### Added
- Comprehensive English documentation and comments throughout the codebase
- Performance demonstration example (`examples/performance_demo.rs`)
- Extensive unit tests for expression parsing and validation
- Error handling examples in README
- Advanced usage patterns and examples

### Changed
- **Performance Optimizations**:
  - Replaced `schedule.upcoming()` with more efficient `schedule.after()` method
  - Implemented batched event sending for multiple triggered schedules
  - Optimized timer checking system using iterator-based functional programming
  - Reduced memory allocations by caching parsed expressions
- **Code Quality**:
  - Improved function and struct documentation with comprehensive examples
  - Enhanced error messages and panic descriptions
  - Better separation of concerns between parsing and execution logic
  - More descriptive variable names and function signatures
- **API Improvements**:
  - Renamed internal functions for better clarity (e.g., `try_english_pattern` → `parse_expression`)
  - Added more detailed documentation for all public APIs
  - Improved example code with better comments and structure

### Fixed
- Improved time calculation accuracy by using `DateTime::after()` instead of `upcoming()`
- Better handling of edge cases in schedule triggering
- More robust expression parsing with clearer error messages

## [0.6.0]

* bump `bevy_ecs` version to `0.16.0`
* use `ScheduleTimer` component for observer system

## [0.5.1]

* use `ScheduleTimer` component for observer system

## [0.5.0]

* bump `bevy_ecs` version to `0.15.0`
* upgrade `cron` to `0.13.0`
* support english pattern for cronjob by [english-to-cron](https://github.com/kaplanelad/english-to-cron)
* system run time is now in Local time

## [0.5.0-rc.1] - 2024-10-23

* bump bevy version to `0.15.0-rc.1`

## [0.4.0] - 2024-07-25

* bump bevy version to `0.14`

## [0.3.0] - 2024-02-18

* bump bevy version to `0.13`

## [0.2.0] - 2023-11-06

* bump bevy version to `0.12`

## 0.1.2

- update docs and readme
