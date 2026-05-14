use serde::{Deserialize, Serialize};

// TODO: Add #[derive(Debug, Serialize, Deserialize, PartialEq)] to ServiceEntry.
#[derive(Debug, PartialEq)]
pub struct ServiceEntry {
    pub technician: String,
    pub service: String,
    pub price: u32,
}

pub fn to_json(entry: &ServiceEntry) -> String {
    // TODO: Serialize entry to a JSON string using serde_json::to_string.
    let _ = entry;
    String::new()
}

pub fn from_json(json: &str) -> ServiceEntry {
    // TODO: Deserialize a ServiceEntry from the JSON string.
    let _ = json;
    ServiceEntry {
        technician: String::new(),
        service: String::new(),
        price: 0,
    }
}

fn main() {
    let entry = ServiceEntry {
        technician: "Trang".to_string(),
        service: "Pedicure".to_string(),
        price: 3500,
    };
    let json = to_json(&entry);
    println!("{json}");
}
