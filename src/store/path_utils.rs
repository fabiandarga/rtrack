use std::path::{ Path, PathBuf };
use std::fs;

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

pub fn get_data_dir() -> PathBuf {
    let config_path = get_config_dir();
    let path = config_path.join("data");
    path
}

pub fn ensure_config_dir_exists() -> std::io::Result<()> {
    let path = get_config_dir();
    fs::create_dir_all(&path)?;
    let data_path = get_data_dir();
    fs::create_dir_all(&data_path)?;
    Ok(())
}