use chrono::{Datelike, DateTime, NaiveDate, Timelike, TimeZone};
use crate::Expr;

pub struct CronEvaluator<Tz: TimeZone> {
    instant: DateTime<Tz>,
}

pub struct Environment {
    now: u8,
    max: u8,
}

impl Environment {
    pub fn new(now: u8, max: u8) -> Self {
        Self { now, max }
    }
}

fn get_days_from_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
        .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
        .num_days()
}

impl<Tz: TimeZone> CronEvaluator<Tz> {
    pub fn new(instant: DateTime<Tz>) -> Self {
        Self { instant }
    }

    pub fn eval(&self, ast: &Expr) -> bool {
        match ast {
            Expr::CronExpr {
                box mins,
                box hours,
                box months,
                box days,
                box day_of_weeks,
            } => {
                let last_day = get_days_from_month(self.instant.date().year(), self.instant.date().month());
                let fmins = self.visit0(
                    &Environment::new(self.instant.time().minute() as u8, 59),
                    mins,
                );
                let fhours = self.visit0(
                    &Environment::new(self.instant.time().hour() as u8, 23),
                    hours,
                );
                let fdays = self.visit0(
                    &Environment::new(self.instant.date().day() as u8, last_day as u8),
                    days,
                );
                let fmonths = self.visit0(
                    &Environment::new(self.instant.date().month() as u8, 12),
                    months,
                );
                let fday_of_weeks = self.visit0(
                    &Environment::new(self.instant.time().minute() as u8, 7),
                    day_of_weeks,
                );
                fmins && fhours && fdays && fmonths && fday_of_weeks
            }
            _ => false
        }
    }

    fn visit1(&self, env: &Environment, ast: &Expr) -> bool {
        match ast {
            Expr::AnyValueExpr => true,
            Expr::LastValueExpr if env.now == env.max => true,
            Expr::ValueExpr(n) if env.now == *n => true,
            Expr::ListExpr(list) => list.iter().any(|e| self.visit0(env, e)),
            Expr::RangeExpr {
                from: box Expr::ValueExpr(start),
                to: box Expr::ValueExpr(end),
                per_option,
            } => match per_option {
                box Expr::NoOp if *start <= env.now && env.now <= *end => true,
                box Expr::ValueExpr(per) => (*start as usize..=*end as usize)
                    .step_by(*per as usize)
                    .into_iter()
                    .any(|e| e == env.now as usize),
                _ => false,
            },
            Expr::PerExpr {
                digit: box Expr::AnyValueExpr,
                option: box Expr::ValueExpr(per),
            } => (0usize..=(env.max as usize))
                .step_by(*per as usize)
                .into_iter()
                .any(|e| e == env.now as usize),
            _ => false,
        }
    }

    fn visit0(&self, env: &Environment, ast: &Expr) -> bool {
        self.visit1(env, ast)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Utc, TimeZone};

    use crate::cron_evaluator::CronEvaluator;
    use crate::Expr;

    #[test]
    fn test_anytime() {
        let date_time = Utc.ymd(2021, 1, 1).and_hms(1, 1, 1);
        let cron_evaluator = CronEvaluator::new(date_time);
        let expr = Expr::CronExpr {
            mins: Box::from(Expr::AnyValueExpr),
            hours: Box::from(Expr::AnyValueExpr),
            days: Box::from(Expr::AnyValueExpr),
            months: Box::from(Expr::AnyValueExpr),
            day_of_weeks: Box::from(Expr::AnyValueExpr),
        };
        let result = cron_evaluator.eval(&expr);
        assert!(result)
    }

    #[test]
    fn test_point_time() {
        let date_time = Utc.ymd(2021, 1, 1).and_hms(1, 1, 1);
        let cron_evaluator = CronEvaluator::new(date_time);
        let expr = Expr::CronExpr {
            mins: Box::from(Expr::ValueExpr(1)),
            hours: Box::from(Expr::ValueExpr(1)),
            days: Box::from(Expr::ValueExpr(1)),
            months: Box::from(Expr::ValueExpr(1)),
            day_of_weeks: Box::from(Expr::AnyValueExpr),
        };
        let result = cron_evaluator.eval(&expr);
        assert!(result)
    }


}
