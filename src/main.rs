#[macro_use]
extern crate serde;

mod ui;
mod store;
mod models;

use crate::models::TimeEntry;
use store::{ tracks, time_entries };
use ui::prompt;
use store::path_utils;
use ui::display;


fn print_tracks() {
    if let Ok(tracks) = tracks::get_tracks() {
        if tracks.len() != 0 {
            println!("Tracks: {:?}", tracks);
        }
    };
}

fn select_track() -> String {
    let mut selected_track: Option<String> = None;
    while selected_track == None {
        let input = prompt("Choose Track (or add new): ");
        if !input.is_empty() {
            if let Err(err) = tracks::add_track(&input) {
                println!("Error: {:?}", err);
            };
            selected_track = Some(input);
        }
    }
    selected_track.unwrap()
}

fn select_time() -> u32 {
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

fn select_message() -> String {
    let input = prompt("Add Message (optional): ");
    input
}

#[derive(Debug, PartialEq)]
enum Mode {
    Add,
    ShowLast,
    Search,
    None,
}

fn select_mode() -> Mode {
    println!("[a] add entry / [l] last entries / [s] search");
    let input = prompt(" > ");
    match input.trim() {
        "a" => {
            Mode::Add
        }
        "l" => {
            Mode::ShowLast
        }
        "s" => {
            Mode::Search
        }
        _ => {
            Mode::None
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = path_utils::get_config_dir().join("db");
    let db = sled::open(db_path)?;
    let store: pallet::Store<TimeEntry> = 
    pallet::Store::builder().with_db(db).with_index_dir(path_utils::get_config_dir()).finish()?;

    let mut mode : Mode = Mode::None;
    while mode == Mode::None {
        mode = select_mode();
    }

    match mode {
        Mode::Add => {
            print_tracks();

            let selected_track = select_track();
            println!("Selected: {}", selected_track);
        
            let time = select_time();
            let msg = select_message();
        
            let entry = TimeEntry::new(selected_track, time, msg);
            time_entries::add_time_entry(&store, &entry)?;
        }
        Mode::ShowLast => {
            let limit = 3;
            let entries = time_entries::get_all_time_entries(&store)?;
            let last_n : Vec<TimeEntry> = entries.iter().take(limit).map(|e| { e.clone() }).collect();

            println!("---");
            println!("Last {} entries", limit);
            println!("---");

            display::print_time_entry_table(&last_n);
        }
        _ => {}
    }

   
    Ok(())
}
