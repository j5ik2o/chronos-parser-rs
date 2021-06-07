use pom::parser::*;

use crate::Expr;
use crate::Expr::{
  AnyValueExpr, ListExpr, NoOp, PerExpr, RangeExpr, ValueExpr, LastValueExpr, CronExpr,
};

fn min_digit<'a>() -> Parser<'a, u8, Expr> {
  (one_of(b"12345") + one_of(b"0123456789")).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
    | (sym(b'0') * one_of(b"0123456789")).map(|e| ValueExpr(e - 48))
    | (one_of(b"0123456789")).map(|e| ValueExpr(e - 48))
}

fn hour_digit<'a>() -> Parser<'a, u8, Expr> {
  (sym(b'2') + one_of(b"0123")).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
    | (sym(b'1') + one_of(b"0123456789")).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
    | sym(b'0') * one_of(b"0123456789").map(|e| ValueExpr(e - 48))
    | one_of(b"0123456789").map(|e| ValueExpr(e - 48))
}

fn day_digit<'a>() -> Parser<'a, u8, Expr> {
  (sym(b'3') + one_of(b"01")).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
    | (one_of(b"12") + one_of(b"0123456789")).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
    | (sym(b'0') * one_of(b"123456789")).map(|e| ValueExpr(e - 48))
    | one_of(b"123456789").map(|e| ValueExpr(e - 48))
}

fn month_digit<'a>() -> Parser<'a, u8, Expr> {
  (sym(b'1') + one_of(b"012")).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
    | (sym(b'0') * one_of(b"123456789")).map(|e| ValueExpr(e - 48))
    | one_of(b"123456789").map(|e| ValueExpr(e - 48))
}

fn day_of_week_digit<'a>() -> Parser<'a, u8, Expr> {
  seq(b"SUN").map(|_| ValueExpr(1))
    | seq(b"MON").map(|_| ValueExpr(2))
    | seq(b"TUE").map(|_| ValueExpr(3))
    | seq(b"WED").map(|_| ValueExpr(4))
    | seq(b"THU").map(|_| ValueExpr(5))
    | seq(b"FRI").map(|_| ValueExpr(6))
    | seq(b"SAT").map(|_| ValueExpr(7))
    | sym(b'L').map(|_| LastValueExpr)
}

fn day_of_week_text<'a>() -> Parser<'a, u8, Expr> {
  one_of(b"1234567").map(|e| ValueExpr(e as u8 - 48))
}

fn asterisk<'a>() -> Parser<'a, u8, Expr> {
  sym(b'*').map(|_| AnyValueExpr)
}

fn per(p: Parser<u8, Expr>) -> Parser<u8, Expr> {
  sym(b'/') * p
}

fn asterisk_per(p: Parser<u8, Expr>) -> Parser<u8, Expr> {
  (asterisk() + per(p)).map(|(d, op)| PerExpr {
    digit: Box::from(d.clone()),
    option: Box::from(op.clone()),
  })
}

fn range_per(p: Parser<u8, Expr>) -> Parser<u8, Expr> {
  per(p).opt().map(|e| match e {
    None => NoOp,
    Some(s) => s,
  })
}

fn range<'a, PF>(p: PF) -> Parser<'a, u8, Expr>
where
  PF: Fn() -> Parser<'a, u8, Expr>,
{
  (p() - sym(b'-') + p() + range_per(p())).map(|((e1, e2), e3)| RangeExpr {
    from: Box::from(e1),
    to: Box::from(e2),
    per_option: Box::from(e3),
  })
}

fn list(p: Parser<u8, Expr>) -> Parser<u8, Expr> {
  pom::parser::list(p, sym(b',')).map(|e| match e {
    e if e.len() == 1 => e.get(0).unwrap().clone(),
    e => ListExpr(e),
  })
}

fn digit_instruction<'a, PF>(p: PF) -> Parser<'a, u8, Expr>
where
  PF: Fn() -> Parser<'a, u8, Expr>,
{
  asterisk_per(p()) | asterisk() | list(range(|| p()) | p())
}

fn instruction<'a>() -> Parser<'a, u8, Expr> {
  (digit_instruction(|| min_digit()) - sym(b' ') + digit_instruction(|| hour_digit()) - sym(b' ')
    + digit_instruction(|| day_digit())
    - sym(b' ')
    + digit_instruction(|| month_digit())
    - sym(b' ')
    + digit_instruction(|| day_of_week_text() | day_of_week_digit()))
  .map(|((((mins, hours), days), months), day_of_weeks)| CronExpr {
    mins: Box::from(mins),
    hours: Box::from(hours),
    days: Box::from(days),
    months: Box::from(months),
    day_of_weeks: Box::from(day_of_weeks),
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_instruction() {
    let result = (instruction() - end()).parse(b"* * * * *").unwrap();
    assert_eq!(
      result,
      CronExpr {
        mins: Box::from(AnyValueExpr),
        hours: Box::from(AnyValueExpr),
        days: Box::from(AnyValueExpr),
        months: Box::from(AnyValueExpr),
        day_of_weeks: Box::from(AnyValueExpr)
      }
    );
    let result = (instruction() - end()).parse(b"1 1 1 1 1").unwrap();
    assert_eq!(
      result,
      CronExpr {
        mins: Box::from(ValueExpr(1)),
        hours: Box::from(ValueExpr(1)),
        days: Box::from(ValueExpr(1)),
        months: Box::from(ValueExpr(1)),
        day_of_weeks: Box::from(ValueExpr(1))
      }
    );
  }

  #[test]
  fn test_digit_instruction() {
    let result = (digit_instruction(|| min_digit()) - end())
      .parse(b"*")
      .unwrap();
    assert_eq!(result, AnyValueExpr);
    let result = (digit_instruction(|| min_digit()) - end())
      .parse(b"*/2")
      .unwrap();
    assert_eq!(
      result,
      PerExpr {
        digit: Box::from(AnyValueExpr),
        option: Box::from(ValueExpr(2))
      }
    );
    let result = (digit_instruction(|| min_digit()) - end())
      .parse(b"1-10/2")
      .unwrap();
    assert_eq!(
      result,
      RangeExpr {
        from: Box::from(ValueExpr(1)),
        to: Box::from(ValueExpr(10)),
        per_option: Box::from(ValueExpr(2))
      }
    );
    let result = (digit_instruction(|| min_digit()) - end())
      .parse(b"1,2,3")
      .unwrap();
    assert_eq!(
      result,
      ListExpr(vec![ValueExpr(1), ValueExpr(2), ValueExpr(3)])
    );
    let result = (digit_instruction(|| min_digit()) - end())
      .parse(b"1")
      .unwrap();
    assert_eq!(result, ValueExpr(1));
  }

  #[test]
  fn test_list() {
    let s = (0..=59)
      .map(|v| v.to_string())
      .collect::<Vec<_>>()
      .join(",");
    let result = (list(min_digit()) - end()).parse(s.as_bytes()).unwrap();
    let values = (0..=59).map(|v| ValueExpr(v)).collect::<Vec<_>>();
    assert_eq!(result, ListExpr(values));
  }

  #[test]
  fn test_range_per() {
    for n2 in 1..=59 {
      let option = n2 / 2;
      let n1 = n2 - 1;
      let s: &str = &format!("{:<02}-{:<02}/{:<02}", n1, n2, option);
      println!("{}", s);
      let result = (range(|| min_digit()) - end()).parse(s.as_bytes()).unwrap();
      assert_eq!(
        result,
        RangeExpr {
          from: Box::from(ValueExpr(n1)),
          to: Box::from(ValueExpr(n2)),
          per_option: Box::from(ValueExpr(option)),
        }
      );
    }
  }

  #[test]
  fn test_asterisk_per() {
    for n in 0..59 {
      let s: &str = &format!("*/{:<02}", n);
      let result = (asterisk_per(min_digit()) - end())
        .parse(s.as_bytes())
        .unwrap();
      assert_eq!(
        result,
        PerExpr {
          digit: Box::from(AnyValueExpr),
          option: Box::from(ValueExpr(n)),
        }
      );
    }
  }

  #[test]
  fn test_per() {
    let result = (per(min_digit()) - end()).parse(b"/2").unwrap();
    assert_eq!(result, ValueExpr(2));
  }

  #[test]
  fn test_min_digit() {
    for n in 0..59 {
      let s: &str = &format!("{:<02}", n);
      let result = (min_digit() - end()).parse(s.as_bytes()).unwrap();
      assert_eq!(result, ValueExpr(n));
    }
    let result = (min_digit() - end()).parse(b"60");
    assert_eq!(result.is_err(), true);
  }

  #[test]
  fn test_hour_digit() {
    for n in 0..=23 {
      if n < 10 {
        let s: &str = &n.to_string();
        let result: Expr = (hour_digit() - end()).parse(s.as_bytes()).unwrap();
        assert_eq!(result, ValueExpr(n));
      }
      let s: &str = &format!("{:<02}", n);
      let result: Expr = (hour_digit() - end()).parse(s.as_bytes()).unwrap();
      assert_eq!(result, ValueExpr(n));
    }
    let result = (hour_digit() - end()).parse(b"24");
    assert_eq!(result.is_err(), true);
  }

  #[test]
  fn test_day_digit() {
    for n in 1..=31 {
      if n < 10 {
        let s: &str = &n.to_string();
        let result: Expr = (day_digit() - end()).parse(s.as_bytes()).unwrap();
        assert_eq!(result, ValueExpr(n));
      }
      let s: &str = &format!("{:<02}", n);
      let result: Expr = (day_digit() - end()).parse(s.as_bytes()).unwrap();
      assert_eq!(result, ValueExpr(n));
    }
    let result = (day_digit() - end()).parse(b"32");
    assert_eq!(result.is_err(), true);
  }

  #[test]
  fn test_month_digit() {
    for n in 1..=12 {
      if n < 10 {
        let s: &str = &n.to_string();
        let result: Expr = (month_digit() - end()).parse(s.as_bytes()).unwrap();
        assert_eq!(result, ValueExpr(n));
      }
      let s: &str = &format!("{:<02}", n);
      let result: Expr = (month_digit() - end()).parse(s.as_bytes()).unwrap();
      assert_eq!(result, ValueExpr(n));
    }
    let result = (month_digit() - end()).parse(b"13");
    assert_eq!(result.is_err(), true);
  }
}
