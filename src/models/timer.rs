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
}