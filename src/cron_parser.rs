use combine::*;
use combine::parser::char::*;

use crate::Expr;
use crate::Expr::{LastValueExpr, ValueExpr};

pub fn sun<'a, Input>() -> impl Parser<Input, Output=Expr>
    where
        Input: Stream<Token=char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("SUN", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| ValueExpr(1))
}

pub fn mon<'a, Input>() -> impl Parser<Input, Output=Expr>
    where
        Input: Stream<Token=char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("MON", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| ValueExpr(2))
}

pub fn tue<'a, Input>() -> impl Parser<Input, Output=Expr>
    where
        Input: Stream<Token=char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("TUE", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| ValueExpr(3))
}

pub fn wed<'a, Input>() -> impl Parser<Input, Output=Expr>
    where
        Input: Stream<Token=char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("WED", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| ValueExpr(4))
}

pub fn thu<'a, Input>() -> impl Parser<Input, Output=Expr>
    where
        Input: Stream<Token=char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("THU", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| ValueExpr(5))
}

pub fn fri<'a, Input>() -> impl Parser<Input, Output=Expr>
    where
        Input: Stream<Token=char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("FRI", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| ValueExpr(6))
}

pub fn sat<'a, Input>() -> impl Parser<Input, Output=Expr>
    where
        Input: Stream<Token=char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("SAT", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| ValueExpr(7))
}

pub fn last<'a, Input>() -> impl Parser<Input, Output=Expr>
    where
        Input: Stream<Token=char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    string_cmp("L", |l, r| l.eq_ignore_ascii_case(&r)).map(|_| LastValueExpr)
}

pub fn day_of_week_text<'a, Input>() -> impl Parser<Input, Output=Expr>
    where
        Input: Stream<Token=char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    attempt(sun())
        .or(attempt(mon()))
        .or(attempt(tue()))
        .or(attempt(wed()))
        .or(attempt(thu()))
        .or(attempt(fri()))
        .or(attempt(sat()))
        .or(attempt(last()))
}

pub fn day_of_week_digit<'a, Input>() -> impl Parser<Input, Output=Expr>
    where
        Input: Stream<Token=char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    digit('1', '7').map(|e| ValueExpr(e as u32 - 48))
}

pub fn min_digit<'a, Input>() -> impl Parser<Input, Output=Expr>
    where
        Input: Stream<Token=char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    let m_00_09 = char('0').with(digit('0' , '9'))
        .or(digit('0', '9')).map(|e| ValueExpr(e as u32 - 48));
    let m_10_59 = digit('1', '5').and(digit('0', '9')).map(|(e1, e2) | {
        let n = (e1 as u32 - 48) * 10 + (e2 as u32 - 48);
        ValueExpr(n)
    });
    attempt(m_10_59).or(attempt(m_00_09))
}

pub fn digit<'a, Input>(min: char, max: char) -> impl Parser<Input, Output=char>
    where
        Input: Stream<Token=char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position> {
    satisfy_map(move |c: char| {
        if c.is_digit(10) {
            if min <= c && c <= max {
                Some(c)
            } else {
                None
            }
        } else {
            None
        }
    }).expected("digit")
}


#[cfg(test)]
mod tests {
    use combine::*;

    use super::*;

    #[test]
    fn test_min_digit() {
        for n in 0..59 {
            let s: &str = &format!("{:<02}", n);
            let result: Expr = min_digit().parse(s).unwrap().0;
            assert_eq!(result, ValueExpr(n));
        }
    }

    #[test]
    fn test_day_of_week_digit() {
        for n in 1..7 {
            let s: &str = &n.to_string();
            let result: Expr = day_of_week_digit().parse(s).unwrap().0;
            assert_eq!(result, ValueExpr(n));
        }
    }

    #[test]
    fn test_day_of_week_text() {
        let result = day_of_week_text().parse("SUN").unwrap().0;
        assert_eq!(result, ValueExpr(1));
        let result = day_of_week_text().parse("MON").unwrap().0;
        assert_eq!(result, ValueExpr(2));
        let result = day_of_week_text().parse("TUE").unwrap().0;
        assert_eq!(result, ValueExpr(3));
        let result = day_of_week_text().parse("WED").unwrap().0;
        assert_eq!(result, ValueExpr(4));
        let result = day_of_week_text().parse("THU").unwrap().0;
        assert_eq!(result, ValueExpr(5));
        let result = day_of_week_text().parse("FRI").unwrap().0;
        assert_eq!(result, ValueExpr(6));
        let result = day_of_week_text().parse("SAT").unwrap().0;
        assert_eq!(result, ValueExpr(7));
        let result = day_of_week_text().parse("L").unwrap().0;
        assert_eq!(result, LastValueExpr);
    }

    #[test]
    fn test_sun() {
        let result = sun().parse("SUN").unwrap().0;
        assert_eq!(result, ValueExpr(1));
        let result = sun().parse("sun").unwrap().0;
        assert_eq!(result, ValueExpr(1));
    }
}