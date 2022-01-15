const seconds_per_minute: i32  = 60;
const seconds_per_hour: i32  = 60 * 60;

fn divide_and_rest(divident: i64, divisor: i64) -> (i64, i64) {
    let rest = divident % divisor;
    let quotient = (divident - rest) / divisor;
    (quotient, rest)
}

fn seconds_to_hours_with_rest(seconds: i64) -> (i64, i64) {
    divide_and_rest(seconds, seconds_per_hour as i64)
}

fn seconds_to_minutes_with_rest(seconds: i64) -> (i64, i64) {
    divide_and_rest(seconds, seconds_per_minute as i64)
}

pub fn seconds_to_hr_short(seconds: i64) -> String {
    if seconds >= 60*60 {
        let (hours, rest) = seconds_to_hours_with_rest(seconds);
        let (minutes, rest) = seconds_to_minutes_with_rest(rest);
        return format!("{h:0>2}h {m:0>2}m {s:0>2}s", s=rest, m=minutes, h=hours);
    }
    if seconds >= 60 {
        let minutes = (seconds as f32 / 60_f32).floor();
        let seconds = seconds % 60;
        return format!("{m:0>2}m {s:0>2}s", s=seconds, m=minutes);
    }
    return format!("{s:0>2}s", s=seconds);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_seconds_to_hr_short_with_1_second() {
        let seconds = 1;
        let res = seconds_to_hr_short(seconds);
        assert_eq!(res, "01s".to_string());
    }
    #[test]
    fn test_seconds_to_hr_short_with_59_seconds() {
        let res = seconds_to_hr_short(59);
        assert_eq!(res, "59s".to_string());
    }
    #[test]
    fn test_seconds_to_hr_short_with_61_seconds() {
        let res = seconds_to_hr_short(61);
        assert_eq!(res, "01m 01s".to_string());
    }
    #[test]
    fn test_seconds_to_hr_short_with_hours() {
        let seconds = 2*60*60 + 22*60; // 2hours + 22 minutes
        let res = seconds_to_hr_short(seconds);
        assert_eq!(res, "02h 22m 00s".to_string());
    }
    #[test]
    fn test_seconds_to_hr_short_with_max_hours() {
        let seconds = 23*60*60 + 59*60 + 59; // 23hours + 59 minutes + 59s
        let res = seconds_to_hr_short(seconds);
        assert_eq!(res, "23h 59m 59s".to_string());
    }
}