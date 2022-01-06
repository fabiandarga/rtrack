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
    let time_entries = map_results_to_time_entries(result);
    Ok(sort_by_timestamp_desc(time_entries))
}

pub fn find_by_dates(store: &TimeEntryStore, dates: DateQuery) -> Result<Vec<TimeEntry>, Box<dyn std::error::Error>> {
    let q = dates.to_string();
    let result = store.search(q.as_str())?;
    let time_entries = map_results_to_time_entries(result);
    Ok(sort_by_timestamp_desc(time_entries))
}

fn map_results_to_time_entries(result: Results<TimeEntry>) -> Vec<TimeEntry> {
    result.hits.iter().map(|h| h.doc.inner.clone()).collect()
}

fn sort_by_timestamp(entries: Vec<TimeEntry>, desc: bool) -> Vec<TimeEntry> {
    let mut copy = entries.clone();
    if desc {
        copy.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    } else {
        copy.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }
    copy
}

fn sort_by_timestamp_desc(entries: Vec<TimeEntry>) -> Vec<TimeEntry> {
    sort_by_timestamp(entries, true)
}

fn sort_by_timestamp_asc(entries: Vec<TimeEntry>) -> Vec<TimeEntry> {
    sort_by_timestamp(entries, false)
}