use pom::parser::*;

use crate::Expr;
use crate::Expr::ValueExpr;

fn min_digit<'a>() -> Parser<'a, u8, Expr> {
  (one_of(b"12345") + one_of(b"0123456789")).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
    | (sym(b'0').opt() * one_of(b"0123456789")).map(|e| ValueExpr(e - 48))
}

fn hour_digit<'a>() -> Parser<'a, u8, Expr> {
  (sym(b'2') + one_of(b"0123")).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
    | (sym(b'1') + one_of(b"0123456789")).map(|(e1, e2)| ValueExpr((e1 - 48) * 10 + e2 - 48))
    | sym(b'0').opt() * one_of(b"0123456789").map(|e| ValueExpr(e - 48))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_min_digit() {
    for n in 0..59 {
      let s: &str = &format!("{:<02}", n);
      let result = min_digit().parse(s.as_bytes()).unwrap();
      assert_eq!(result, ValueExpr(n));
    }
  }

  #[test]
  fn test_hour_digit() {
    for n in 0..23 {
      let s: &str = &format!("{:<02}", n);
      let result: Expr = hour_digit().parse(s.as_bytes()).unwrap();
      assert_eq!(result, ValueExpr(n));
    }
  }
}
