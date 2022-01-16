#[macro_use]
extern crate serde;
extern crate chrono;
use chrono::prelude::*;
use std::{thread, time};
use uuid::Uuid;

mod data;
mod ui;
mod store;
mod models;
mod search;
mod arguments;
mod types;
mod actions;

use crate::timers::delete_timer;
use crate::display::{ print_timer_table, print_track_added };
use crate::types::{ Arguments, Mode };
use crate::actions::{ 
    handle_track_input,
    get_msg_from_user,
    get_track_time_from_user,
    get_stop_index_from_user
};
use crate::input::{ parse_time, select_mode };
use crate::arguments::{ get_clap_app, get_arguments };
use crate::models::{ TimeEntry, Timer };
use store::{ 
    tracks,
    time_entries,
    timers, 
    path_utils::{ ensure_config_dir_exists, get_data_dir, get_timer_data_dir }
};
use ui::{ prompt, display, input};

fn display_running_timers(timer_store: &pallet::Store<Timer>) {
    let entries_result = timers::get_all_timer_entries(&timer_store);

    match entries_result {
        Ok(entries) => {
            let timers: Vec<Timer> = entries.iter().map(|doc| doc.inner.clone()).collect();
            let wait_time = time::Duration::from_millis(100);
            loop {
                let now : DateTime<Local> = Local::now();
                print!("{esc}c", esc = 27 as char);
                print_timer_table(&timers, now);
                thread::sleep(wait_time);
            }
        },
        Err(_) => {}
    }
}

fn track_select_process(arguments: &Arguments)-> Result<String, Box<dyn std::error::Error>> {
    let track_names = tracks::get_tracks()?;
    let (selected_track, is_new) = actions::get_track_name_from_user(&track_names, &arguments);
    handle_track_input(&selected_track, is_new, &prompt)?;
    Ok(selected_track)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ensure_config_dir_exists()?;
    let app_conf = get_clap_app();
    let arguments = get_arguments(app_conf.get_matches());

    let data_path = get_data_dir();
    let db_path = data_path.join("db");
    let db = sled::open(db_path)?;
    let store: pallet::Store<TimeEntry> = pallet::Store::builder()
        .with_db(db)
        .with_index_dir(&data_path)
        .finish()?;

    let timer_data_path = get_timer_data_dir();
    let timer_db_path = timer_data_path.join("db");
    let timer_db = sled::open(timer_db_path)?;
    let timer_store: pallet::Store<Timer> = pallet::Store::builder()
        .with_db(timer_db)
        .with_index_dir(timer_data_path)
        .finish()?;

    let mut mode : Mode = arguments.mode.clone();
    if mode == Mode::None {
        mode = select_mode();
    }

    match mode {
        Mode::Track => {
            let track_name = track_select_process(&arguments)?;
            let id_str = Uuid::new_v4()
                .to_simple()
                .encode_lower(&mut Uuid::encode_buffer())
                .to_owned();

            println!("{}", id_str);
            let msg = get_msg_from_user(&arguments);
            let date : DateTime<Local> = Local::now();

            let timer = Timer::new(id_str, track_name, msg, date.timestamp());
            timers::add_timer(&timer_store, &timer)?;

            if arguments.display {
                display_running_timers(&timer_store);
            }
        }
        Mode::Add => {
            let track_name = track_select_process(&arguments)?;
            let time = get_track_time_from_user(&arguments);
            let msg = get_msg_from_user(&arguments);
            
            let entry = TimeEntry::from_date(track_name, time.unwrap(), msg, Local::now());

            time_entries::add_time_entry(&store, &entry)?;
         
            print_track_added(&entry);
        }
        Mode::Display => {
            display_running_timers(&timer_store);
        }
        Mode::Stop => {
            let entries_result = timers::get_all_timer_entries(&timer_store);
            match entries_result {
                Ok(entries) => {
                    let timers: Vec<Timer> = entries.iter().map(|doc| doc.inner.clone()).collect();
                    let now : DateTime<Local> = Local::now();
                    print_timer_table(&timers, now);

                    let index = get_stop_index_from_user(&arguments);

                    if entries.len() > index {
                        let timer_doc = &entries[index];
                        let entry = timer_doc.finish(Local::now());
                        time_entries::add_time_entry(&store, &entry)?;
                        print_track_added(&entry);

                        delete_timer(&timer_store, timer_doc.id)?;
                    } else {
                        println!("The selected timer was not found!");
                    }
                }
                Err(_) => {
                    println!("No running timers! Try 'rtrack --help' for more information.");
                }
            };
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
        Mode::None => {}
    }

   
    Ok(())
}
