#[path = "../src/bin/c48_exercise.rs"]
#[allow(dead_code)]
mod c48_exercise;

use c48_exercise::{save_daily_log, Service};

#[test]
fn writes_log_file() {
    let path = "test_daily_c48.csv";
    let entries = vec![
        Service { technician: "Mai".to_string(), service: "Gel Manicure".to_string(), price: 4500 },
        Service { technician: "Linh".to_string(), service: "Pedicure".to_string(), price: 3500 },
    ];
    save_daily_log(path, &entries).unwrap();
    let content = std::fs::read_to_string(path).unwrap();
    std::fs::remove_file(path).unwrap();
    assert!(content.contains("technician,service,price"));
    assert!(content.contains("Mai,Gel Manicure,4500"));
    assert!(content.contains("Linh,Pedicure,3500"));
}

#[test]
fn empty_entries() {
    let path = "test_daily_empty_c48.csv";
    save_daily_log(path, &[]).unwrap();
    let content = std::fs::read_to_string(path).unwrap();
    std::fs::remove_file(path).unwrap();
    assert_eq!(content, "");
}
