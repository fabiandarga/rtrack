use std::io::{ Error, Read, Write };
use std::fs::File;

use crate::path_utils;

pub fn get_tracks() -> Result<Vec<String>, Error> {
    let path = path_utils::get_tracks_file_path();
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let vec = buffer.split(";")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_owned())
        .collect::<Vec<String>>();

    Ok(vec)
}

pub fn add_track(track_name: &str) -> Result<(), String> {
    if track_name.is_empty() {
        return Err("add_track - track_name may not be empty".to_owned());
    }
    if let Ok(mut tracks) = get_tracks() {
        if tracks.iter().any(|s| s == track_name) {
            return Err(format!("add_track - track with this name already exists: {}", track_name));
        }
        tracks.push(track_name.to_owned());
        
        return match save_tracks(tracks) {
            Ok(_) => Ok(()),
            Err(_) => Err("add_track - could not save track data".to_owned()),
        }
    } else {
        return Err("add_track - could not read track data".to_owned());
    };
}

pub fn save_tracks(tracks:Vec<String>) -> Result<(), Error> {
    let path = path_utils::get_tracks_file_path();
    let mut f = File::create(path).unwrap();

    write!(&mut f, "{}", tracks.join(";"))?;
    Ok(())
}