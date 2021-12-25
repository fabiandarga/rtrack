#[macro_use]
extern crate serde;

extern crate chrono;

use crate::arguments::get_clap_app;
use crate::arguments::get_arguments;
use chrono::prelude::*;

mod ui;
mod store;
mod models;
mod search;
mod arguments;
mod types;

use crate::models::TimeEntry;
use store::{ tracks, time_entries };
use store::path_utils::{ ensure_config_dir_exists, get_data_dir };
use ui::prompt;
use ui::display;
use ui::input;
use types::Mode;

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
    ensure_config_dir_exists()?;
    let app_conf = get_clap_app();
    let arguments = get_arguments(app_conf.get_matches());

    let data_path = get_data_dir();
    let db_path = data_path.join("db");
    let db = sled::open(db_path)?;
    let store: pallet::Store<TimeEntry> = 
    pallet::Store::builder().with_db(db).with_index_dir(data_path).finish()?;

    let mut mode : Mode = arguments.mode;
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
            let date : DateTime<Local> = Local::now();
            let date_str= format!("{}-{}-{}", date.year(), date.month(), date.day());
        
            let entry = TimeEntry::new(selected_track, time, msg, date_str, date.timestamp());
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
        Mode::Search => {
            println!("---");
            println!("Search by date\n");
            println!("Enter year, year-month or year-month-day for exact matches.");
            println!("Or enter a range. e.g. \"2020-04:2021-04\".\n");
            let date_query = input::choose_date()?;
            let entries = time_entries::find_by_dates(&store, date_query)?;
            display::print_time_entry_table(&entries);
        }
        _ => {}
    }

   
    Ok(())
}
