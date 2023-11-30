# chronos-parser-rs

[![Workflow Status](https://github.com/j5ik2o/chronos-parser-rs/workflows/ci/badge.svg)](https://github.com/j5ik2o/chronos-parser-rs/actions?query=workflow%3A%22ci%22)
[![crates.io](https://img.shields.io/crates/v/chronos-parser-rs.svg)](https://crates.io/crates/chronos-parser-rs)
[![docs.rs](https://docs.rs/chronos-parser-rs/badge.svg)](https://docs.rs/chronos-parser-rs)
[![Renovate](https://img.shields.io/badge/renovate-enabled-brightgreen.svg)](https://renovatebot.com)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![](https://tokei.rs/b1/github/j5ik2o/chronos-parser-rs)](https://github.com/XAMPPRocky/tokei)

A Rust crate for CROND parser.

## usage

```rust
let cron_schedule = CronSchedule::new("0-59/30 0-23/2 * * *").unwrap();

let dt: chrono::DateTime<Utc> = Utc.with_ymd_and_hms(2021, 1, 1, 1, 1, 0).unwrap();
let iterator = cron_schedule.upcoming(dt);

for next_trigger in itr.take(5) {
  println!("{}", next_trigger);
}
```

## Related Crates

- [j5ik2o/chronos-scheduler-rs](https://github.com/j5ik2o/chronos-scheduler-rs)
- [j5ik2o/oni-comb-rs](https://github.com/j5ik2o/oni-comb-rs)
- [j5ik2o/intervals-rs](https://github.com/j5ik2o/intervals-rs)

## Implementations for Scala

- [j5ik2o/chronos-scheduler-scala](https://github.com/j5ik2o/chronos-scheduler-scala)
- [j5ik2o/chronos-parser-scala](https://github.com/j5ik2o/chronos-parser-scala)
