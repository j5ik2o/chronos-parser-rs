use chrono::{DateTime, TimeZone};

use crate::{CronEvaluator, Expr};

pub trait Specification<T> {
  fn is_satisfied_by(&self, arg: &T) -> bool;
}

#[derive(Clone)]
pub struct CronSpecification<'a> {
  expr: &'a Expr,
}

impl<'a> CronSpecification<'a> {
  pub fn new(expr: &'a Expr) -> Self {
    Self { expr }
  }
}

impl<'a, Tz: TimeZone> Specification<DateTime<Tz>> for CronSpecification<'a> {
  fn is_satisfied_by(&self, datetime: &DateTime<Tz>) -> bool {
    CronEvaluator::new(datetime).eval(self.expr)
  }
}
