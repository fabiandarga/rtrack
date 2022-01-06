use chrono::Duration;
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


pub fn print_timer(entry: &Timer, time: DateTime<Local>, index: usize) {
    let start = Local.timestamp(entry.start, 0);
    let start_str = format!("{y}-{m}-{d} {h}:{min}:{s}",
        y = start.year(), m = start.month(), d = start.day(),
        h = start.hour(), min = start.minute(), s = start.second());
    let diff = time.timestamp() - start.timestamp();
    let duration = Duration::seconds(diff);
    // format to display duration as "1d 3:43:05"
    let dur_str = format!("{}", duration.num_seconds());
    println!("# {} | {} | {} | {} | {}", index, entry.track, dur_str, start_str, entry.message);
}

pub fn print_timer_table(entries: &Vec<Timer>, time: DateTime<Local> ) {
    println!("no. | Track | duration | start | message");
    entries.iter().enumerate().for_each(|(index, entry)| { 
        print_timer(entry, time, index);
    });
}

pub fn print_selected_track(track_name: &str) {
    println!("Selected Track: {}", track_name);
}

pub fn print_track_added(entry: &TimeEntry) {
    println!("Added: {} \"{}\" {} Minutes", entry.track, entry.message, entry.minutes);
}