// #![feature(generic_associated_types)]
#![allow(incomplete_features)]
#![allow(dead_code)]
#![feature(impl_trait_in_bindings)]
#![feature(box_patterns)]
extern crate pom;

pub use ast::*;
pub use cron_evaluator::*;
pub use cron_parser::*;
pub use cron_specification::*;

mod ast;
mod cron_evaluator;
mod cron_parser;
mod cron_specification;

#[cfg(test)]
mod tests {
  use chrono::NaiveDate;

  #[test]
  fn test_main() {
    let year = 2018;
    for (m, d) in (1..=12).map(|m| {
      (
        m,
        if m == 12 {
          NaiveDate::from_ymd(year + 1, 1, 1)
        } else {
          NaiveDate::from_ymd(year, m + 1, 1)
        }
        .signed_duration_since(NaiveDate::from_ymd(year, m, 1))
        .num_days(),
      )
    }) {
      println!("days {} in month {}", d, m);
    }
  }
}
