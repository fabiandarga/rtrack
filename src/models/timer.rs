use chrono::TimeZone;
use chrono::DateTime;
use crate::TimeEntry;

#[derive(Serialize, Deserialize, Debug, pallet::DocumentLike, Clone)]
#[pallet(tree_name = "timer")]
pub struct Timer {
    pub id: String,
    pub track: String,
    pub message: String,
    pub start: i64,
}

impl Timer {
    pub fn new(id: String, track: String, message: String, start: i64) -> Timer {
        Timer {
            id,
            track,
            message,
            start,
        }
    }
    pub fn finish<T>(&self, final_time: DateTime<T>) -> TimeEntry where T: TimeZone {
        let diff = final_time.timestamp() - &self.start; // if possible test this step

        TimeEntry::from_date(
            self.track.to_owned(),
            diff as u32,
            self.message.to_owned(),
            final_time
        )
    }
}
