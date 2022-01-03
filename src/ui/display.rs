use crate::Timer;
use crate::TimeEntry;
use chrono::{DateTime, TimeZone, Local, Datelike, Timelike};

pub fn print_tracks(tracks: &Vec<String>) {
    if tracks.len() != 0 {
        println!("Tracks: {:?}", tracks);
    } else {
        println!("No tracks, yet")
    }
}

pub fn print_time_entry(entry: &TimeEntry) {
    println!("{} | {} | {} | {}", entry.track, entry.date ,entry.minutes, entry.message);
}

pub fn print_time_entry_table(entries: &Vec<TimeEntry>) {
    println!("Track | Date   | Duration | message");
    entries.iter().for_each(|entry| { 
        print_time_entry(entry);
    });
}


pub fn print_timer(entry: &Timer, time: DateTime<Local>) {
    let start = Local.timestamp_millis(entry.start);
    let start_str = format!("{y}-{m}-{d} {h}:{min}:{s}",
    y = start.year(), m = start.month(), d = start.day(), h = start.hour(), min = start.minute(), s = start.second());
    let duration = time - start;
    println!("{} | {} | {} | {}", entry.track, duration.num_minutes(), start_str, entry.message);
}

pub fn print_timer_table(entries: &Vec<Timer>, time: DateTime<Local> ) {
    println!("Track | duration | start | message");
    entries.iter().for_each(|entry| { 
        print_timer(entry, time);
    });
}

pub fn print_selected_track(track_name: &str) {
    println!("Selected Track: {}", track_name);
}

pub fn print_track_added(entry: &TimeEntry) {
    println!("Added: {} \"{}\" {} Minutes", entry.track, entry.message, entry.minutes);
}