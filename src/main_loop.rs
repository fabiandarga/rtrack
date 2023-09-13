use std::{sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread::{self, Thread, JoinHandle}, time::{Instant, Duration}, io::{stdout, Write}};

use chrono::{DateTime, Local };
use crossterm::{
    terminal::{ enable_raw_mode, disable_raw_mode, ClearType, Clear },
    event::{KeyEvent, KeyCode, Event, self},
    queue,
    cursor::{MoveTo, Hide, MoveToNextLine},
    style::Print
};

use crate::types::Mode;

pub fn start_loop() -> Result<(), Box<dyn std::error::Error>> {
  enable_raw_mode()?;
  let (tx, rx) = mpsc::channel();
  let mode = Arc::new(Mutex::new(Mode::Display));
  let input_handle = input_loop(tx, Arc::clone(&mode));
  let action_hanlde = action_loop(rx, Arc::clone(&mode));
  let render_handle = render_loop(Arc::clone(&mode));
  //disable_raw_mode()?;
  input_handle.join().unwrap();
  Ok(())
}

enum LoopEvent<I> {
    Input(I),
    Tick,
}

fn input_loop(tx: Sender<LoopEvent<KeyEvent>>, mode: Arc<Mutex<Mode>>) -> JoinHandle<()> {
    thread::spawn(move || {
        let tick_rate = Duration::from_millis(200);

        let mut last_tick = Instant::now();
        while *mode.lock().unwrap() != Mode::Quit {
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
    })
}

fn action_loop(rx: Receiver<LoopEvent<KeyEvent>>, mode: Arc<Mutex<Mode>>)
    -> JoinHandle<()>
    {
    thread::spawn(move || {
        while *mode.lock().unwrap() != Mode::Quit {
            match rx.recv().unwrap() {
                LoopEvent::Input(event) => {
                    match event.code {
                        KeyCode::Char('s') => { print!("S"); },
                        KeyCode::Char('q') => { *mode.lock().unwrap() = Mode::Quit; }
                        KeyCode::Char(char) => {
                            println!("key: {}", char);
                        }
                        _ => {}
                    };
                }
                LoopEvent::Tick => {}
            };

        }
    })
}

pub fn render_loop(mode: Arc<Mutex<Mode>>) -> JoinHandle<()> {
    thread::spawn(move || {
        while *mode.lock().unwrap() != Mode::Quit {
            let mut out = stdout();
            queue!(
                out,
                Hide,
                Clear(ClearType::All),
                MoveTo(0, 0),
            ).unwrap();
            match *mode.lock().unwrap() {
                Mode::Quit => {
                   queue!(out, Print("QUIT")).unwrap();
                } 
               _ => {
                   queue!(out, Print("OTHER")).unwrap();
               } 
            }
            
            queue!(out, MoveToNextLine(1), Print("test :)")).unwrap();
            out.flush().unwrap();

            thread::sleep(Duration::from_millis(200));
        }
    })
}
