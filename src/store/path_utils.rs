use std::path::{ Path, PathBuf };

use home;

pub fn get_config_dir() -> PathBuf {
    match home::home_dir() {
        Some(path) => path.join(".rtrack"),
        None => Path::new("./").to_path_buf(),
    }
}

pub fn get_tracks_file_path() -> PathBuf {
    let config_path = get_config_dir();
    let path = config_path.join("tracks.rtr");
    path
}