use crossterm::event::{ read, Event, KeyEvent };
use crossterm::terminal::{ enable_raw_mode, disable_raw_mode };
use std::io::{ Error, ErrorKind };

// Let the user press a single key, without the need to press return.
// return a key event for that one key.
pub fn read_key_input() -> Result<KeyEvent, Error> {
    enable_raw_mode()?;
    let res = read()?;
    disable_raw_mode()?;
    match res {
        Event::Key(ke) => {
            return Ok(ke);
        },
        _ => return Err(Error::new(ErrorKind::Other, format!("No Input")))
    };
}