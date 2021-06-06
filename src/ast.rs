pub trait ExprVisitor<T> {
  fn visit(&self, e: &impl ExprAcceptor) -> T;
}

pub trait ExprAcceptor {
  fn accept<T>(&self, visitor: impl ExprVisitor<T>) -> T
  where
    Self: Sized,
  {
    visitor.visit(self)
  }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
  NoOp,
  ValueExpr(u8),
  LastValueExpr,
  AnyValueExpr,
  PerExpr {
    digit: Box<Expr>,
    option: Box<Expr>,
  },
  RangeExpr {
    from: Box<Expr>,
    to: Box<Expr>,
    per_option: Box<Expr>,
  },
  ListExpr {
    exprs: Vec<Expr>,
  },
  CronExpr {
    mins: Box<Expr>,
    hours: Box<Expr>,
    days: Box<Expr>,
    months: Box<Expr>,
    day_of_weeks: Box<Expr>,
  },
}

impl ExprAcceptor for Expr {}
