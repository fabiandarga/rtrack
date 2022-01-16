use crate::data::{ Seconds, SECONDS_PER_DAY, SECONDS_PER_HOUR, SECONDS_PER_MINUTE };

pub fn seconds_to_hr_short(seconds: Seconds) -> String {
    let int_secs: i64 = seconds.into();
    if int_secs >= SECONDS_PER_DAY {
        let (days, rest) = seconds.to_days_with_rest();
        let (hours, rest) = rest.to_hours_with_rest();
        let (minutes, rest) = rest.to_minutes_with_rest();
        return format!("{d}d {h:0>2}h {m:0>2}m {s:0>2}s", s=rest, m=minutes, h=hours, d=days);
    }
    if int_secs >= SECONDS_PER_HOUR {
        let (hours, rest) = seconds.to_hours_with_rest();
        let (minutes, rest) = rest.to_minutes_with_rest();
        return format!("{h:0>2}h {m:0>2}m {s:0>2}s", s=rest, m=minutes, h=hours);
    }
    if int_secs >= SECONDS_PER_MINUTE {
        let (minutes, seconds) = seconds.to_minutes_with_rest();
        return format!("{m:0>2}m {s:0>2}s", s=seconds, m=minutes);
    }
    return format!("{s:0>2}s", s=seconds);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{ Seconds };
    #[test]
    fn test_seconds_to_hr_short_with_1_second() {
        let seconds = Seconds::new(1);
        let res = seconds_to_hr_short(seconds);
        assert_eq!(res, "01s".to_string());
    }
    #[test]
    fn test_seconds_to_hr_short_with_59_seconds() {
        let res = seconds_to_hr_short(Seconds::new(59));
        assert_eq!(res, "59s".to_string());
    }
    #[test]
    fn test_seconds_to_hr_short_with_61_seconds() {
        let res = seconds_to_hr_short(Seconds::new(61));
        assert_eq!(res, "01m 01s".to_string());
    }
    #[test]
    fn test_seconds_to_hr_short_with_hours() {
        let seconds = 2*60*60 + 22*60; // 2 hours + 22 minutes
        let res = seconds_to_hr_short(Seconds::new(seconds));
        assert_eq!(res, "02h 22m 00s".to_string());
    }
    #[test]
    fn test_seconds_to_hr_short_with_max_hours() {
        let seconds = 23*60*60 + 59*60 + 59; // 23 hours + 59 minutes + 59s
        let res = seconds_to_hr_short(Seconds::new(seconds));
        assert_eq!(res, "23h 59m 59s".to_string());
    }
    #[test]
    fn test_seconds_to_hr_short_with_days() {
        let seconds = 3*60*60*24 + 23*60; // 3 days + 23 minutes
        let res = seconds_to_hr_short(Seconds::new(seconds));
        assert_eq!(res, "3d 00h 23m 00s");
    }
}