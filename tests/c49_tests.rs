#[path = "../src/bin/c49_exercise.rs"]
#[allow(dead_code)]
mod c49_exercise;

use c49_exercise::{from_json, to_json, ServiceEntry};

#[test]
fn round_trip() {
    let entry = ServiceEntry {
        technician: "Mai".to_string(),
        service: "Gel Manicure".to_string(),
        price: 4500,
    };
    let json = to_json(&entry);
    let loaded = from_json(&json);
    assert_eq!(loaded, entry);
}

#[test]
fn json_contains_fields() {
    let entry = ServiceEntry {
        technician: "Linh".to_string(),
        service: "Pedicure".to_string(),
        price: 3500,
    };
    let json = to_json(&entry);
    assert!(json.contains("Linh"));
    assert!(json.contains("Pedicure"));
    assert!(json.contains("3500"));
}
