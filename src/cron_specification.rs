use std::marker::PhantomData;

use chrono::{DateTime, TimeZone};

use crate::{CronEvaluator, Expr};

pub struct CronSpecification<Tz: TimeZone>(PhantomData<Tz>);

impl<Tz: TimeZone> CronSpecification<Tz> {
    pub fn never() -> impl Fn(DateTime<Tz>) -> bool {
        |_| false
    }

    pub fn of(expr: Expr) -> impl Fn(DateTime<Tz>) -> bool {
        move |instant| CronEvaluator::new(instant).eval(&expr)
    }
}