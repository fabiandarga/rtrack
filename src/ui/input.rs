use crate::ui::prompt;
use crate::search::DateQuery;

pub fn select_track(tracks: &Vec<String>) -> (String, bool) {
    let mut input: String = "".to_owned();
    let mut is_new = false;

    while input.is_empty() {
        input = prompt("Choose Track (or add new): ");
        if !input.is_empty() {
            is_new = !tracks.iter().any(|track| track == &input);
        }
    }
    (input, is_new)
}

pub fn select_time() -> u32 {
    let mut time: Option<u32> = None;
    while time == None {
        let input = prompt("Add minutes: ");
        if !input.is_empty() {
            time = match input.parse::<u32>() {
                Ok(number) => Some(number),
                _ => None,
            }
        }
    }
    time.unwrap()
}

pub fn select_message() -> String {
    let input = prompt("Add Message (optional): ");
    input
}

pub fn choose_date() -> Result<DateQuery, String> {
    let input = prompt("Filter by date (year, month or day): ");
    let split = input.split(":");
    let vec: Vec<&str> = split.map(|s| s.trim()).collect();
    match vec.len() {
        2 => {
            let from = vec[0].to_owned();
            let to = vec[1].to_owned();
            Ok(DateQuery::new(from, Some(to)))
        }
        1 => {
            let from = vec[0].to_owned();
            Ok(DateQuery::new(from, None))
        }
        _ => {
            Err("Wrong Date Format".to_owned())
        }
    }
}