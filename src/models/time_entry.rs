
#[derive(Serialize, Deserialize, Debug, pallet::DocumentLike, Clone)]
#[pallet(tree_name = "time_entries")]
pub struct TimeEntry {
    #[pallet(default_search_field)]
    track: String,
    #[pallet(index_field_type = "u64")]
    minutes: u32,
    #[pallet(default_search_field)]
    message: String,
}

impl TimeEntry {
    pub fn new(track: String, minutes: u32, message: String) -> TimeEntry {
        TimeEntry {
            track,
            minutes,
            message,
        }
    }
}