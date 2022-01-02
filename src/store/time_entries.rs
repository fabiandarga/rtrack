use crate::search::DateQuery;
use pallet::search::Results;
use crate::models::TimeEntry;

type TimeEntryStore = pallet::Store<TimeEntry>;

pub fn add_time_entry(store: &TimeEntryStore, entry: &TimeEntry) -> Result<(), Box<dyn std::error::Error>> {
    let _ = store.create(entry)?;
    Ok(())
}

pub fn get_all_time_entries(store: &TimeEntryStore) -> Result<Vec<TimeEntry>, Box<dyn std::error::Error>> {
    let result = store.search("*")?;
    Ok(map_results_to_time_entries(result))
}

pub fn find_by_dates(store: &TimeEntryStore, dates: DateQuery) -> Result<Vec<TimeEntry>, Box<dyn std::error::Error>> {
    let q = dates.to_string();
    let result = store.search(q.as_str())?;
    Ok(map_results_to_time_entries(result))
}

fn map_results_to_time_entries(result: Results<TimeEntry>) -> Vec<TimeEntry> {
    result.hits.iter().map(|h| h.doc.inner.clone()).collect()
}