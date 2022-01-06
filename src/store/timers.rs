use pallet::Document;
use crate::Timer;
use pallet::search::Results;

type TimerStore = pallet::Store<Timer>;

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