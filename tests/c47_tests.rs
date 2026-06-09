#[path = "../src/bin/c47_exercise.rs"]
#[allow(dead_code)]
mod c47_exercise;

use c47_exercise::{load_price_list, Service};

#[test]
fn reads_price_list() {
    let path = "test_services_c47.csv";
    std::fs::write(path, "name,price\nGel Manicure,4500\nPedicure,3500\n").unwrap();
    let list = load_price_list(path).unwrap();
    std::fs::remove_file(path).unwrap();
    assert_eq!(
        list,
        vec![
            Service { name: "Gel Manicure".to_string(), price: 4500 },
            Service { name: "Pedicure".to_string(), price: 3500 },
        ]
    );
}

#[test]
fn missing_file_is_err() {
    let result = load_price_list("nonexistent_c47.csv");
    assert!(result.is_err());
}
