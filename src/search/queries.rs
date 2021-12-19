use chrono::prelude::*;

pub struct DateQuery {
    from: String,
    to: Option<String>,
    date_field: String,
    timestamp_field: String,
}

impl DateQuery {
    pub fn new(from: String, to: Option<String>) -> DateQuery {
        DateQuery {
            from,
            to,
            date_field: "date".to_owned(),
            timestamp_field: "timestamp".to_owned(),
        }
    }
    pub fn with_field_names(&self, date_field: String, timestamp_field: String) -> DateQuery {
        DateQuery {
            from: self.from.clone(),
            to: self.to.clone(),
            date_field,
            timestamp_field,
        }
    }
}

impl ToString for DateQuery {
    fn to_string(&self) -> String {
        if let Some(to) = &self.to {
            let from_ts = str_to_date(&self.from).timestamp();
            let to_ts = str_to_date(&to).timestamp();
            // search between to timestamps
            format!("{name}:>{from} AND {name}:<{to}", name=self.timestamp_field, to=to_ts, from=from_ts)
        } else {
            // search date string match
            format!("{name}:{from}", name=self.date_field, from=self.from)
        }
    }
}

fn str_to_date(date_str: &str) -> DateTime<Local> {
    let split: Vec<u32> = date_str.split("-").map(|str| str.parse::<u32>().unwrap()).collect();
    match split.len() {
        3 => Local.ymd(split[0] as i32, split[1], split[2]).and_hms(0, 0, 0),
        2 => Local.ymd(split[0] as i32, split[1], 1).and_hms(0, 0, 0),
        1 => Local.ymd(split[0] as i32, 1, 1).and_hms(0, 0, 0),
        _ => Local::now(),
    }
}