use std::fmt;

#[derive(Copy, Clone)]
pub struct Days(i64);

impl Days {
    pub fn new(int: i64) -> Days {
        Days(int)
    }
}

impl From<i64> for Days {
    fn from(num: i64) -> Self {
        Days(num)
    }
}

impl From<Days> for i64 {
    fn from(days: Days) -> Self {
        days.0
    }
}

impl fmt::Display for Days {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}