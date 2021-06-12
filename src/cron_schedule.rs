use chrono::{DateTime, TimeZone};
use intervals_rs::LimitValue;

use crate::{CronInterval, CronParser, CronSpecification, Expr};

pub struct CronSchedule<Tz>
where
  Tz: TimeZone,
{
  expr: Expr,
  timezone: Tz,
}

impl<Tz: TimeZone> CronSchedule<Tz> {
  pub fn new(source: &str, timezone: Tz) -> Self {
    let expr = CronParser::parse(source).unwrap();
    Self { expr, timezone }
  }

  pub fn cron_interval(&self, start: DateTime<Tz>) -> CronInterval<Tz, CronSpecification> {
    CronInterval::new(
      LimitValue::Limit(start.clone()),
      LimitValue::Limitless,
      CronSpecification::new(&self.expr),
      self.timezone.clone(),
    )
  }
}
