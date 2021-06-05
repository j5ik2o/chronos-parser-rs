use combine::*;
use combine::parser::char::*;
use crate::Expr::ValueExpr;
use crate::Expr;

pub fn sun<'a, Input>() -> impl Parser<Input, Output = Expr>
    where
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("SUN", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| { ValueExpr(1) })
}

pub fn mon<'a, Input>() -> impl Parser<Input, Output = Expr>
    where
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("MON", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| { ValueExpr(1) })
}

pub fn tue<'a, Input>() -> impl Parser<Input, Output = Expr>
    where
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("TUE", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| { ValueExpr(1) })
}

pub fn wed<'a, Input>() -> impl Parser<Input, Output = Expr>
    where
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("WED", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| { ValueExpr(1) })
}

pub fn thu<'a, Input>() -> impl Parser<Input, Output = Expr>
    where
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("THU", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| { ValueExpr(1) })
}

pub fn fri<'a, Input>() -> impl Parser<Input, Output = Expr>
    where
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("FRI", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| { ValueExpr(1) })
}

pub fn sat<'a, Input>() -> impl Parser<Input, Output = Expr>
    where
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("SAT", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| { ValueExpr(1) })
}

pub fn last<'a, Input>() -> impl Parser<Input, Output = Expr>
    where
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("L", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| { ValueExpr(1) })
}

pub fn day_of_week_text<'a, Input>() -> impl Parser<Input, Output = Expr>
    where
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    sun().or(mon()).or(tue()).or(wed()).or(thu()).or(fri()).or(sat()).or(last())
}

#[cfg(test)]
mod tests {
    use combine::*;
    use crate::{sun, day_of_week_text};
    use crate::Expr::ValueExpr;

    #[test]
    fn test_day_of_week_text() {
        let result = day_of_week_text().parse("SUN").unwrap().0;
        assert_eq!(result, ValueExpr(1));
    }

    #[test]
    fn test_sun() {
        let result = sun().parse("SUN").unwrap().0;
        assert_eq!(result, ValueExpr(1));
        let result = sun().parse("sun").unwrap().0;
        assert_eq!(result, ValueExpr(1));
    }
}