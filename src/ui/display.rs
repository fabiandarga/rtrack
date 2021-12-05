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