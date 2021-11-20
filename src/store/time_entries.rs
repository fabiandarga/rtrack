use crate::models::TimeEntry;

pub fn add_time_entry(store: &pallet::Store<TimeEntry>, entry: &TimeEntry) -> Result<(), Box<dyn std::error::Error>>  {
    let _ = store.create(entry)?;
    Ok(())
}

pub fn get_all_time_entries(store: &pallet::Store<TimeEntry>) -> Result<Vec<TimeEntry>, Box<dyn std::error::Error>> {
    let result = store.search("*")?;
    let hits = result.hits;
    let entries = hits.iter().map(|h| h.doc.inner.clone()).collect();
    Ok(entries)
}