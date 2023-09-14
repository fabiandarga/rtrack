use std::path::PathBuf;

use pallet::Document;
use crate::Timer;
use pallet::search::Results;

pub type TimerStore = pallet::Store<Timer>;

pub fn build_timer_store(data_path: PathBuf) -> Result<TimerStore, Box<dyn std::error::Error>> {
    let timer_db_path = data_path.join("db");
    let timer_db = sled::open(timer_db_path)?;
    let store = pallet::Store::builder()
        .with_db(timer_db)
        .with_index_dir(data_path)
        .finish()?;
    Ok(store)
}

pub fn add_timer(store: &TimerStore, entry: &Timer) -> Result<(), Box<dyn std::error::Error>> {
    let _ = store.create(entry)?;
    Ok(())
}

pub fn get_all_timer_entries(store: &TimerStore) -> Result<Vec<Document<Timer>>, Box<dyn std::error::Error>> {
    let result = store.search("*")?;
    Ok(map_results_to_timer_entries(result))
}

fn map_results_to_timer_entries(result: Results<Timer>) -> Vec<Document<Timer>> {
    result.hits.iter().map(|h| h.doc.clone()).collect()
}

pub fn delete_timer(store: &TimerStore, id: u64) -> Result<(), Box<dyn std::error::Error>> {
    store.delete(id)?;
    Ok(())
}
