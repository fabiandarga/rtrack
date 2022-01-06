use crate::types::Mode;

pub struct Arguments {
    pub mode: Mode,
    pub track: Option<String>,
    pub message: Option<String>,
    pub time: Option<String>,
    pub index: Option<usize>,
    pub display: bool,
}

impl Arguments {
    pub fn default() -> Arguments {
        return Arguments {
            mode: Mode::None,
            track: None,
            message: None,
            time: None,
            index: None,
            display: false,
        }
    }
}