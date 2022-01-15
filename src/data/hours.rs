use std::fmt;

#[derive(Copy, Clone)]
pub struct Hours(i64);

impl Hours {
    pub fn new(int: i64) -> Hours {
        Hours(int)
    }
}

impl From<i64> for Hours {
    fn from(num: i64) -> Self {
        Hours(num)
    }
}

impl From<Hours> for i64 {
    fn from(hours: Hours) -> Self {
        hours.0
    }
}

impl fmt::Display for Hours {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
