use std::io::Stdout;
use chrono::{Local, DateTime};
use crossterm::{
    queue,
    style::Print, cursor::MoveToNextLine, event::{ KeyEvent, KeyCode }
};

use crate::{
    types::Mode,
    store::{
        timers::{TimerStore, get_all_timer_entries},
        time_entries::{
            TimeEntryStore, get_last_n_time_entries
        }
    },
    ui::display::{print_time_entry_table, print_timer_table}, models::Timer
};

pub enum UserCommand {
    ChangeMode(Mode)
}

pub fn handle_input(event: KeyEvent) -> Option<UserCommand> {
    match event.code {
        KeyCode::Char('t') => {
            return Some(UserCommand::ChangeMode(Mode::Track)); 
        }
        _ => {}
    }
    None 

}

pub fn render(mut out: &Stdout, timers: &TimerStore, time_entries: &TimeEntryStore) {
    // get entries info
    let last_time_entries = get_last_n_time_entries(time_entries, 3).unwrap();
      
    queue!(
        out,
        Print("=== RTrack ==="),
        MoveToNextLine(1),
        Print("Welcome to RTrack!"),
        MoveToNextLine(2),
        Print("Timers"),
        MoveToNextLine(1),
        Print("----------")
    ).expect("could not render Display Head");
    
    let timers_result = get_all_timer_entries(&timers).unwrap();
    let current_timers: Vec<Timer> = timers_result.iter().map(|doc| doc.inner.clone()).collect();
    let now : DateTime<Local> = Local::now();
    let timer_lines = print_timer_table(&current_timers, now);
    timer_lines.iter().for_each(|line| {
        queue!(out, MoveToNextLine(1), Print(line)).unwrap();
    });
    
    queue!(
        out,
        MoveToNextLine(1),
        MoveToNextLine(1),
        Print("[t] start new timer | [s] stop a timer | [p] pause a timer"),
        MoveToNextLine(2),
        Print("Entries"),
        MoveToNextLine(1),
        Print("----------")).expect("Could not render display mode");

    let time_entry_lines = print_time_entry_table(&last_time_entries);
    time_entry_lines.iter()
        .for_each(|line| {
            queue!(out, 
                   MoveToNextLine(1),
                   Print(line)).unwrap();
        });

    queue!(
        out,
        MoveToNextLine(2),
        Print("[l] list last entries | [r] search for entries | [a] add time entry ")
    ).expect("Could not render display mode");
}
