mod ui;
mod store;

use store::tracks;
use ui::prompt;
fn print_tracks() {
    if let Ok(tracks) = tracks::get_tracks() {
        if tracks.len() != 0 {
            println!("Tracks: {:?}", tracks);
        }
    };
}

fn select_track() -> String {
    let mut selected_track: Option<String> = None;
    while selected_track == None {
        let input = prompt("Choose Track (or add new): ");
        if !input.is_empty() {
            if let Err(err) = tracks::add_track(&input) {
                println!("Error: {:?}", err);
            };
            selected_track = Some(input);
        }
    }
    selected_track.unwrap()
}

fn main() {
    print_tracks();
    // let mut running = true;
    
    let selected_track = select_track();
    println!("Selected: {}", selected_track);
}
