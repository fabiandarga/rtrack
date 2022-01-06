use chrono::DateTime;
use chrono::{ TimeZone, Datelike };

#[derive(Serialize, Deserialize, Debug, pallet::DocumentLike, Clone)]
#[pallet(tree_name = "time_entries")]
pub struct TimeEntry {
    #[pallet(default_search_field)]
    pub track: String,
    #[pallet(index_field_type = "u64")]
    pub minutes: u32,
    #[pallet(default_search_field)]
    pub message: String,
    #[pallet(default_search_field)]
    pub date: String,
    #[pallet(default_search_field)]
    pub timestamp: i64,
}

impl TimeEntry {
    pub fn new(track: String, minutes: u32, message: String, date:String, timestamp: i64) -> TimeEntry {
        TimeEntry {
            track,
            minutes,
            message,
            date,
            timestamp
        }
    }
    pub fn from_date<T>(track: String, minutes:  u32, message: String, date: DateTime<T>,) -> TimeEntry where T: TimeZone {
        let date_str = format!("{}-{:0>2}-{:0>2}", date.year(), date.month(), date.day());
        TimeEntry::new(track, minutes, message, date_str, date.timestamp())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Local;
use super::*;
    #[test]
    fn test_from_date() {
        let date : DateTime<Local> = Local.ymd(2022, 1, 7).and_hms(12, 0, 0);
        let ts = date.timestamp();
        let time_entry = TimeEntry::from_date("track".to_owned(), 11, "test".to_owned(), date);
        assert_eq!(time_entry.timestamp, ts);
        assert_eq!(time_entry.date, "2022-01-07");
    }
}