use crate::TimeEntry;

pub fn print_time_entry(entry: &TimeEntry) {
    println!("{} | {} | {}", entry.track, entry.minutes, entry.message);
}

pub fn print_time_entry_table(entries: &Vec<TimeEntry>) {
    println!("Track | Duration | message");
    entries.iter().for_each(|entry| { 
        print_time_entry(entry);
    });
}