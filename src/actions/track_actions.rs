use crate::parse_time;
use crate::types::Arguments;

use crate::ui::display;
use crate::ui::input;

pub fn get_track_name_from_user(track_names: &Vec<String>, arguments: &Arguments) -> (String, bool)  {
    match &arguments.track {
        Some(track) => {
            let is_new = !track_names.contains(&track);
            (track.to_owned(), is_new)
        }
        None => {
            display::print_tracks(&track_names);
            let input = input::select_track(&track_names);
            input
        }
    }
}

pub fn get_track_time_from_user(arguments: &Arguments) -> Option<u32> {
    let mut time: Option<u32> = match &arguments.time {
        Some(time_str) => parse_time(&time_str),
        None => None,
    };

    while time == None {
        let time_str = input::select_time();
        time = parse_time(&time_str);
    }
    time
}

pub fn get_msg_from_user(arguments: &Arguments) -> String {
    match &arguments.message {
        Some(message) => message.to_owned(),
        None => input::select_message(),
    }
}