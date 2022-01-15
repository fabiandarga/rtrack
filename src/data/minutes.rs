use std::fmt;

#[derive(Copy, Clone)]
pub struct Minutes(i64);

impl Minutes {
    pub fn new(int: i64) -> Minutes {
        Minutes(int)
    }
}

impl From<i64> for Minutes {
    fn from(num: i64) -> Self {
        Minutes(num)
    }
}

impl From<Minutes> for i64 {
    fn from(min: Minutes) -> Self {
        min.0
    }
}

impl fmt::Display for Minutes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}