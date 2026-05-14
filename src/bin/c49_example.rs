use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ServiceEntry {
    technician: String,
    service: String,
    price: u32,
}

fn main() {
    let entry = ServiceEntry {
        technician: "Mai".to_string(),
        service: "Gel Manicure".to_string(),
        price: 4500,
    };

    let json = serde_json::to_string_pretty(&entry).unwrap();
    println!("Serialized:\n{json}");

    let loaded: ServiceEntry = serde_json::from_str(&json).unwrap();
    println!("Deserialized: {loaded:?}");
}
