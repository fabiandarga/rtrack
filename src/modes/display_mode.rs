use std::io::Stdout;
use crossterm::{
    queue,
    style::Print, cursor::MoveToNextLine, event::{ KeyEvent, KeyCode }
};

use crate::types::Mode;

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

pub fn render(mut out: &Stdout) {
    // get entries info
    queue!(
        out,
        Print("=== RTrack ==="),
        MoveToNextLine(1),
        Print("Welcome to RTrack!"),
        MoveToNextLine(2),
        Print("Timers"),
        MoveToNextLine(1),
        Print("----------"),
        MoveToNextLine(1),
        Print("[t] start new timer | [s] stop a timer | [p] pause a timer"),
        MoveToNextLine(2),
        Print("Entries"),
        MoveToNextLine(1),
        Print("----------"),
        MoveToNextLine(1),
        Print("[l] list last entries | [r] search for entries | [a] add time entry ")
    ).expect("Could not render display mode");
}
