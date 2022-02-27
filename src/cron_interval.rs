use std::marker::PhantomData;
use std::rc::Rc;

use chrono::{DateTime, Duration, TimeZone};
use intervals_rs::{Interval, LimitValue};

use crate::Specification;

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
      next: dt.clone(),
      curr: dt,
      cron_interval: Rc::new(self.clone()),
    }
  }

}

#[derive(Clone)]
pub struct CronIntervalIterator<Tz: TimeZone, S: Specification<DateTime<Tz>> + Clone> {
  timezone: Tz,
  curr: DateTime<Tz>,
  next: DateTime<Tz>,
  cron_interval: Rc<CronInterval<Tz, S>>,
}

impl<'a, Tz: TimeZone, S: Specification<DateTime<Tz>> + Clone> Iterator
  for CronIntervalIterator<Tz, S>
{
  type Item = DateTime<Tz>;

  fn next(&mut self) -> Option<Self::Item> {
    self.curr = self.next.clone();
    self.next = self.next.clone() + Duration::minutes(1);
    match self.end_value() {
      None => {
        self.proceed_next();
        let curr: DateTime<Tz> = self.curr.clone();
        Some(curr)
      }
      Some(end) => {
        if end >= self.curr {
          self.proceed_next();
          let curr: DateTime<Tz> = self.curr.clone();
          Some(curr)
        } else {
          None
        }
      }
    }
  }
}

impl<'a, Tz: TimeZone, S: Specification<DateTime<Tz>> + Clone> CronIntervalIterator<Tz, S> {

  pub fn timezone(&self) -> &Tz {
    &self.timezone
  }

  pub fn cron_interval(&self) -> Rc<CronInterval<Tz, S>> {
    self.cron_interval.clone()
  }

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

  fn proceed_next(&mut self) {
    while !self
        .cron_interval
        .cron_specification
        .is_satisfied_by(&self.curr)
    {
      self.curr = self.next.clone();
      self.next = self.next.clone() + Duration::minutes(1);
    }
  }

}

#[cfg(test)]
mod tests {
  use chrono::{TimeZone, Utc};
  use intervals_rs::LimitValue;

  use crate::{CronInterval, CronParser, CronSpecification};

  #[test]
  fn test_iterator() {
    let dt = Utc.ymd(2021, 1, 1).and_hms(1, 1, 0);

    let expr = CronParser::parse("0-59/30 0-23/2 * * *").unwrap();
    let interval = CronInterval::new(
      LimitValue::Limit(dt),
      LimitValue::Limitless,
      CronSpecification::new(expr),
    );
    let itr = interval.iter(Utc);
    itr.take(5).for_each(|e| println!("{:?}", e));
  }
}
