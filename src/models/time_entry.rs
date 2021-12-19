
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
}