use pom::parser::*;

use crate::Expr;
use crate::Expr::{ValueExpr, AnyValueExpr, PerExpr, NoOp, RangeExpr, ListExpr};

fn min_digit<'a>() -> Parser<'a, u8, Expr> {
  (one_of(b"12345") + one_of(b"0123456789") ).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
    | (sym(b'0') * one_of(b"0123456789") ).map(|e| ValueExpr(e - 48))
      | (one_of(b"0123456789") ).map(|e| ValueExpr(e - 48))
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

fn asterisk<'a>() -> Parser<'a, u8, Expr> {
  sym(b'*').map(|_| AnyValueExpr)
}

fn per<'a, PF>(p: PF) -> Parser<'a, u8, Expr>
where
  PF: Fn() -> Parser<'a, u8, Expr>,
{
  sym(b'/') * p()
}

fn asterisk_per<'a, PF>(p: PF) -> Parser<'a, u8, Expr>
where
  PF: Fn() -> Parser<'a, u8, Expr>,
{
  (asterisk() + per(p)).map(|(d, op)| PerExpr {
    digit: Box::from(d.clone()),
    option: Box::from(op.clone()),
  })
}

fn range_per<'a, PF>(p: PF) -> Parser<'a, u8, Expr> where PF: Fn() -> Parser<'a, u8, Expr>{
  per(p).opt().map(|e| match e {
    None => NoOp,
    Some(s) => s,
  })
}

fn range<'a, PF>(p: PF) -> Parser<'a, u8, Expr> where PF: Fn() -> Parser<'a, u8, Expr>{
  (p() - sym(b'-') + p() + range_per(p)).map(|((e1, e2), e3) | RangeExpr {
    from: Box::from(e1),
    to: Box::from(e2),
    per_option: Box::from(e3)
  })
}

fn list<'a, PF>(p: PF) -> Parser<'a, u8, Expr> where PF: Fn() -> Parser<'a, u8, Expr> {
  pom::parser::list(p(), sym(b',')).map(|e| match e {
    e if e.len() == 1 => e.get(0).unwrap().clone(),
    e => ListExpr(e),
  } )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_list() {
    //  (0..=59).enumerate().into_iter().fold("".to_string(), |s, (i, e)|{
    //    let e = if
    //    let result = format!("{}{}", s, e);
    //    result
    //  });
    //
    // for n2 in 1..=59 {
    //
    //   let s: &str = &format!("{:<02},{:<02}", n);
    //   let result = (list(|| min_digit()) - end()).parse(s.as_bytes()).unwrap();
    // }
  }

  #[test]
  fn test_range_per() {
    for n2 in 1..=59 {
      let option = n2/2;
      let n1 = n2 - 1;
      let s: &str = &format!("{:<02}-{:<02}/{:<02}", n1, n2, option);
      println!("{}",s);
      let result = (range(|| min_digit()) - end()).parse(s.as_bytes()).unwrap();
      assert_eq!(result, RangeExpr {
        from: Box::from(ValueExpr(n1)),
        to: Box::from(ValueExpr(n2)),
        per_option: Box::from(ValueExpr(option))
      });
    }
  }

  #[test]
  fn test_asterisk_per() {
    for n in 0..59 {
      let s: &str = &format!("*/{:<02}", n);
      let result = (asterisk_per(|| min_digit()) - end())
          .parse(s.as_bytes())
          .unwrap();
      assert_eq!(
        result,
        PerExpr {
          digit: Box::from(AnyValueExpr),
          option: Box::from(ValueExpr(n))
        }
      );
    }
  }

  #[test]
  fn test_per() {
    let result = (per(|| min_digit()) - end()).parse(b"/2").unwrap();
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
