# chronos-parser-rs

[![Workflow Status](https://github.com/j5ik2o/chronos-parser-rs/workflows/ci/badge.svg)](https://github.com/j5ik2o/chronos-parser-rs/actions?query=workflow%3A%22ci%22)
[![crates.io](https://img.shields.io/crates/v/chronos-parser-rs.svg)](https://crates.io/crates/chronos-parser-rs)
[![docs.rs](https://docs.rs/chronos-parser-rs/badge.svg)](https://docs.rs/chronos-parser-rs)
[![Renovate](https://img.shields.io/badge/renovate-enabled-brightgreen.svg)](https://renovatebot.com)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/License-APACHE2.0-blue.svg)](https://opensource.org/licenses/apache-2-0)
[![](https://tokei.rs/b1/github/j5ik2o/chronos-parser-rs)](https://github.com/XAMPPRocky/tokei)

A Rust crate for CROND parser.

## Installation

Add the following configuration to `Cargo.toml`.

```toml
[dependencies]
chronos-parser-rs = "0.1.XXX"
```

## Usage

```rust
// Create a new cron schedule
let cron_schedule = CronSchedule::new("0-59/30 0-23/2 * * *").unwrap();

// Create a DateTime object representing January 1, 2021, 1:01:00 AM (UTC)
let dt: chrono::DateTime<Utc> = Utc.with_ymd_and_hms(2021, 1, 1, 1, 1, 0).unwrap();
// Return an iterator that calculates the scheduled execution times after this date and time
let iterator = cron_schedule.upcoming(dt);

// Print the next 5 scheduled execution times
for next_trigger in iterator.take(5) {
  println!("{}", next_trigger);
}
```

```stdout
2021-01-01T02:00:00Z
2021-01-01T02:30:00Z
2021-01-01T04:00:00Z
2021-01-01T04:30:00Z
2021-01-01T06:00:00Z
```

## Related Crates

- [j5ik2o/chronos-scheduler-rs](https://github.com/j5ik2o/chronos-scheduler-rs)
- [j5ik2o/oni-comb-rs](https://github.com/j5ik2o/oni-comb-rs)
- [j5ik2o/intervals-rs](https://github.com/j5ik2o/intervals-rs)

## Implementations for Scala

- [j5ik2o/chronos-scheduler-scala](https://github.com/j5ik2o/chronos-scheduler-scala)
- [j5ik2o/chronos-parser-scala](https://github.com/j5ik2o/chronos-parser-scala)
