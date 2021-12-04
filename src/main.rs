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
use ui::input;

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
            let track_names = tracks::get_tracks()?;
            display::print_tracks(&track_names);

            let (selected_track, is_new) = input::select_track(&track_names);
            if is_new {
                println!("Create new Track: {}? (Y/n)", selected_track);
                let answer = prompt(" > ");
                if answer != "n" {
                    tracks::add_track(&selected_track)?;
                }
            } else {
                println!("Selected Track: {}", selected_track);
            }
        
            let time = input::select_time();
            let msg = input::select_message();
        
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
