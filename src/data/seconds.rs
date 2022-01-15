use std::fmt;
use super::{ Minutes, Hours, Days };

pub const SECONDS_PER_MINUTE: i64 = 60;
pub const SECONDS_PER_HOUR: i64 = 60 * 60;
pub const SECONDS_PER_DAY: i64 = SECONDS_PER_HOUR * 24;

#[derive(Copy, Clone)]
pub struct Seconds(i64);

impl Seconds {
    pub fn new(int: i64) -> Seconds {
        Seconds(int)
    }
}

impl From<i64> for Seconds {
    fn from(num: i64) -> Self {
        Seconds(num)
    }
}

impl From<Seconds> for i64 {
    fn from(secs: Seconds) -> Self {
        secs.0
    }
}

impl fmt::Display for Seconds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}



impl Seconds {
    pub fn to_days_with_rest(self) -> (Days, Seconds) {
        divide_and_rest(self.into(), SECONDS_PER_DAY)
    }
    pub fn to_hours_with_rest(self) -> (Hours, Seconds) {
        divide_and_rest(self.into(), SECONDS_PER_HOUR)
    }
    pub fn to_minutes_with_rest(self) -> (Minutes, Seconds) {
        divide_and_rest(self.into(), SECONDS_PER_MINUTE)
    }
}

fn divide_and_rest<T:From<i64>>(seconds: Seconds, divisor: i64) -> (T, Seconds) {
    let divident: i64 = seconds.into();
    let rest = divident % divisor;
    let quotient = (divident - rest) / divisor;
    (quotient.into(), rest.into())
}
