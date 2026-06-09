#[allow(unused_imports)]
use csv::Writer;
use serde::Serialize;
use std::fs;

#[derive(Debug, Serialize)]
pub struct Service {
    pub technician: String,
    pub service: String,
    pub price: u32,
}

pub fn save_daily_log(path: &str, entries: &[Service]) -> Result<(), String> {
    // TODO: Create a csv::Writer::from_path(path) (map_err to String).
    // For each entry, wtr.serialize(entry)? — the header row is written
    // automatically from the struct's field names. Finish with wtr.flush().
    let _ = (path, entries);
    Ok(())
}

fn main() {
    let entries = vec![Service {
        technician: "Mai".to_string(),
        service: "Gel".to_string(),
        price: 4500,
    }];
    save_daily_log("daily_log_test.csv", &entries).unwrap();
    let _ = fs::remove_file("daily_log_test.csv");
}
