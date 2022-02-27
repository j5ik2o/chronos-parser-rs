use chrono::{DateTime, TimeZone};
use intervals_rs::LimitValue;

use crate::{CronInterval, Expr, CronSpecification, CronParser, CronIntervalIterator};
use std::marker::PhantomData;

pub struct CronSchedule<Tz>
where
  Tz: TimeZone,
{
  expr: Expr,
  phantom: PhantomData<Tz>,
}

impl<Tz: TimeZone> CronSchedule<Tz> {
  pub fn new(source: &str) -> Self {
    let expr = CronParser::parse(source).unwrap();
    Self {
      expr,
      phantom: PhantomData,
    }
  }

  pub fn cron_interval(&self, start: DateTime<Tz>) -> CronInterval<Tz, CronSpecification> {
    let spec = CronSpecification::new(self.expr.clone());
    CronInterval::new(LimitValue::Limit(start), LimitValue::Limitless, spec)
  }

  pub fn upcoming(&self, start: DateTime<Tz>) -> CronIntervalIterator<Tz, CronSpecification> {
    self.cron_interval(start.clone()).iter(start.timezone())
  }
}

#[cfg(test)]
mod tests {
  use chrono::{TimeZone, Utc};
  use intervals_rs::LimitValue;

  use crate::{CronInterval, CronParser, CronSchedule, CronSpecification};

  #[test]
  fn test_iterator() {
    let dt = Utc.ymd(2021, 1, 1).and_hms(1, 1, 0);

    let cs = CronSchedule::new("0-59/30 0-23/2 * * *");
    let itr = cs.upcoming(dt);

    itr.take(5).for_each(|e| println!("{:?}", e));
  }
}