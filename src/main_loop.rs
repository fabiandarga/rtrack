use std::{sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}, time::{Instant, Duration}, io::{stdout, Write}};
use crossterm::{
    terminal::{ enable_raw_mode, disable_raw_mode, ClearType, Clear },
    event::{KeyEvent, KeyCode, Event, self},
    queue, execute,
    cursor::{MoveTo, Hide, MoveToNextLine, Show},
    style::Print
};

use crate::{types::Mode, modes::display_mode::UserCommand, store::{timers::TimerStore, time_entries::TimeEntryStore}};
use crate::modes::display_mode;
pub fn start_loop(timer_store: TimerStore, time_entry_store: TimeEntryStore)
    -> Result<(), Box<dyn std::error::Error>> {
  enable_raw_mode()?;
  let (tx, rx) = mpsc::channel();
  let mode = Arc::new(Mutex::new(Mode::Display));
  let input_handle = input_loop(tx, Arc::clone(&mode));
  
  action_loop(rx, Arc::clone(&mode));
  render_loop(Arc::clone(&mode));
  input_handle.join().unwrap();
  disable_raw_mode()?;
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
                        KeyCode::Esc | KeyCode::Char('q') => {
                            let mut out = stdout();
                            execute!(
                                out,
                                Show,
                                Clear(ClearType::All),
                                MoveTo(0, 0)
                            ).unwrap();
                            *mode.lock().unwrap() = Mode::Quit;
                        }
                        _ => {}
                    }
                    let mut res: Option<UserCommand> = None;
                    match *mode.lock().unwrap() {
                        Mode::Display => {
                            res = display_mode::handle_input(event);
                        }
                        _ => {}
                    }

                    if let Some(cmd) = res {
                        match cmd {
                            UserCommand::ChangeMode(new_mode) => {
                                *mode.lock().unwrap() = new_mode;
                            }
                        }
                    }
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
                Mode::Display => {
                    display_mode::render(&out);
                }
                Mode::Track => {
                   queue!(out, Print("TRACK")).unwrap();
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
