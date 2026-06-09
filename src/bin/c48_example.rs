use csv::Writer;
use serde::Serialize;
use std::fs;

// serialize() writes the header row (from the field names) before the first
// record, then one CSV row per entry. No format!, no manual string building.
#[derive(Debug, Serialize)]
struct Service {
    technician: String,
    service: String,
    price: u32,
}

fn save_daily_log(path: &str, entries: &[Service]) -> Result<(), String> {
    let mut wtr = Writer::from_path(path).map_err(|e| format!("cannot write file: {e}"))?;
    for entry in entries {
        wtr.serialize(entry).map_err(|e| format!("cannot write row: {e}"))?;
    }
    wtr.flush().map_err(|e| format!("cannot flush: {e}"))?;
    Ok(())
}

fn main() {
    let entries = vec![
        Service { technician: "Mai".to_string(), service: "Gel Manicure".to_string(), price: 4500 },
        Service { technician: "Linh".to_string(), service: "Pedicure".to_string(), price: 3500 },
    ];
    save_daily_log("daily_log_example.csv", &entries).unwrap();
    let content = fs::read_to_string("daily_log_example.csv").unwrap();
    println!("{content}");
    let _ = fs::remove_file("daily_log_example.csv");
}
