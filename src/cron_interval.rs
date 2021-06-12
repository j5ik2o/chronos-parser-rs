use std::marker::PhantomData;

use chrono::{DateTime, Duration, TimeZone};
use intervals_rs::{Interval, LimitValue};

use crate::{Specification};

#[derive(Clone)]
pub struct CronInterval<Tz: TimeZone, S: Specification<DateTime<Tz>> + Clone> {
  underlying: Interval<i64>,
  cron_specification: S,
  phantom: PhantomData<Tz>,
}

impl<Tz: TimeZone, S: Specification<DateTime<Tz>> + Clone> CronInterval<Tz, S> {
  pub fn new(
    start_value: LimitValue<DateTime<Tz>>,
    end_value: LimitValue<DateTime<Tz>>,
    cron_specification: S,
  ) -> Self {
    let start = match start_value {
      LimitValue::Limitless => LimitValue::Limitless,
      LimitValue::Limit(v) => LimitValue::Limit(v.timestamp_millis()),
    };
    let end = match end_value {
      LimitValue::Limitless => LimitValue::Limitless,
      LimitValue::Limit(v) => LimitValue::Limit(v.timestamp_millis()),
    };
    Self {
      underlying: Interval::closed(start, end),
      cron_specification,
      phantom: PhantomData,
    }
  }
  pub fn iter(&self, timezone: Tz) -> CronIntervalIterator<Tz, S> {
    let dt: DateTime<Tz> = match self.underlying.as_lower_limit().as_value() {
      Ok(&v) => timezone.timestamp_millis(v),
      Err(..) => panic!(),
    };
    CronIntervalIterator {
      timezone,
      next: dt,
      cron_interval: self,
    }
  }
}

#[derive(Clone)]
pub struct CronIntervalIterator<'a, Tz: TimeZone, S: Specification<DateTime<Tz>> + Clone> {
  timezone: Tz,
  next: DateTime<Tz>,
  cron_interval: &'a CronInterval<Tz, S>,
}

impl<'a, Tz: TimeZone, S: Specification<DateTime<Tz>> + Clone> CronIntervalIterator<'a, Tz, S> {
  fn end_value(&self) -> Option<DateTime<Tz>> {
    let end_value = if self.cron_interval.underlying.has_upper_limit() {
      let result = match self.cron_interval.underlying.as_upper_limit().as_value() {
        Ok(&v) => self.timezone.timestamp_millis(v),
        Err(..) => panic!(),
      };
      Some(result)
    } else {
      None
    };
    end_value
  }
}

impl<'a, Tz: TimeZone, S: Specification<DateTime<Tz>> + Clone> Iterator
  for CronIntervalIterator<'a, Tz, S>
{
  type Item = DateTime<Tz>;

  fn next(&mut self) -> Option<Self::Item> {
    let current = self.next.clone();
    self.next = self.next.clone() + Duration::minutes(1);
    match self.end_value() {
      None => {
        if self
          .cron_interval
          .cron_specification
          .is_satisfied_by(current.clone())
        {
          Some(current)
        } else {
          self.next()
        }
      }
      Some(end) => {
        if end >= current
        {
          if
          self
              .cron_interval
              .cron_specification
              .is_satisfied_by(current.clone()) {
            Some(current)
          } else {
            self.next()
          }
        } else {
          None
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use chrono::{TimeZone, Utc};
  use intervals_rs::LimitValue;

  use crate::{CronInterval, CronSpecification, Expr};

  #[test]
  fn test() {
    let dt = Utc.ymd(2021, 1, 1).and_hms(1, 1, 0);

    let interval = CronInterval::new(
      LimitValue::Limit(dt),
      LimitValue::Limitless,
      CronSpecification::new(Expr::CronExpr {
        mins: Box::from(Expr::RangeExpr {
          from: Box::from(Expr::ValueExpr(1)),
          to: Box::from(Expr::ValueExpr(5)),
          per_option: Box::from(Expr::NoOp),
        }),
        hours: Box::from(Expr::AnyValueExpr),
        days: Box::from(Expr::AnyValueExpr),
        months: Box::from(Expr::AnyValueExpr),
        day_of_weeks: Box::from(Expr::AnyValueExpr),
      }),
    );
    let itr = interval.iter(Utc);
    itr.take(5).for_each(|e| println!("{:?}", e));
  }
}
