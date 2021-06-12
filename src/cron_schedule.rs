use chrono::{DateTime, TimeZone};
use intervals_rs::LimitValue;

use crate::{CronInterval, Expr, CronSpecification, CronParser};
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

  // pub fn upcoming(&self, start: DateTime<Tz>) -> CronIntervalIterator<'_, Tz, CronSpecification> {
  //   self.cron_interval(start).iter(start.timezone())
  // }
}
