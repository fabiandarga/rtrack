use crossterm::cursor::MoveTo;
use crossterm::terminal::Clear;
use std::io::stdout;
use chrono::Local;
use chrono::DateTime;
use crate::Timer;
use crate::print_timer_table;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::time::Instant;
use std::thread;
use crossterm::execute;
use crossterm::event;
use crossterm::terminal::ClearType;
use crossterm::event::{ Event, KeyEvent, KeyCode, KeyModifiers };
use std::sync::mpsc;
use crossterm::terminal::{ enable_raw_mode, disable_raw_mode };
use std::io::Write;

enum LoopEvent<I> {
    Input(I),
    Tick,
}

#[derive(PartialEq)]
enum Mode {
    Main,
    Stop,
}

pub fn run_display_mode(timers: Vec<Timer>) 
    -> Result<(), Box<dyn std::error::Error>>
{
    enable_raw_mode()?;
    let (tx, rx) = mpsc::channel();
    input_loop(tx);
    render_loop(rx, timers)?;
    disable_raw_mode()?;
    Ok(())
}

fn input_loop(tx: Sender<LoopEvent<KeyEvent>>) {
    thread::spawn(move || {
        let tick_rate = Duration::from_millis(200);

        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let Event::Key(key) = event::read().expect("can read events") {
                    tx.send(LoopEvent::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(LoopEvent::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
}

fn render_loop(rx: Receiver<LoopEvent<KeyEvent>>, timers: Vec<Timer>)
    -> Result<(), Box<dyn std::error::Error>>
    {
    let mut active_mode = Mode::Main;
    let mut index_to_stop: Option<u32> = None;

    loop {
        match rx.recv()? {
            LoopEvent::Input(event) => {
                match event.code {
                    KeyCode::Char('s') => active_mode = Mode::Stop,
                    KeyCode::Char('q') => { break; }
                    KeyCode::Char('c') => {
                        if event.modifiers.contains(KeyModifiers::CONTROL) {
                            break; // Break the ui loop
                        }
                    }
                    KeyCode::Char(char) => {
                        if active_mode == Mode::Stop {
                            let n: u32 = char.to_digit(10).expect("invalid input");
                            index_to_stop = Some(n);
                            break;
                        }
                    }
                    _ => {}
                };
            }
            LoopEvent::Tick => {}
        };

        let now : DateTime<Local> = Local::now();
        execute!(
            stdout(),
            Clear(ClearType::All),
            MoveTo(0, 0),
        )?;
        disable_raw_mode()?;
        print_timer_table(&timers, now);
        enable_raw_mode()?;
        
        if active_mode == Mode::Main {
            write!(stdout(), "Press [s] to stop a timer; [q] to quit\n");
        } else {
            write!(stdout(), "Which timer to delete? Press a number [0-9].\n");
        }
    }
    if index_to_stop != None {
        write!(stdout(), "delete {}\n", index_to_stop.unwrap());
    }
    Ok(())
}