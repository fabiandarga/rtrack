use crate::Timer;
use crate::TimeEntry;

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


pub fn print_timer(entry: &Timer) {
    println!("{} | {} | {}", entry.track, entry.message, entry.start);
}

pub fn print_timer_table(entries: &Vec<Timer>) {
    println!("Track | message   | start");
    entries.iter().for_each(|entry| { 
        print_timer(entry);
    });
}

pub fn print_selected_track(track_name: &str) {
    println!("Selected Track: {}", track_name);
}

pub fn print_track_added(entry: &TimeEntry) {
    println!("Added: {} \"{}\" {} Minutes", entry.track, entry.message, entry.minutes);
}