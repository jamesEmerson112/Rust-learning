#[path = "../src/bin/c74_exercise.rs"]
#[allow(dead_code)]
mod c74_exercise;

use c74_exercise::import_and_total;

fn temp_db() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

#[test]
fn imports_csv_and_totals() {
    let path = "test_c74.csv";
    std::fs::write(path, "name,price\nGel,4500\nPedicure,3500\n").unwrap();
    let db = temp_db();
    let total = import_and_total(&db, path).unwrap();
    std::fs::remove_file(path).unwrap();
    assert_eq!(total, 8000);
    assert_eq!(db.len(), 2);
}

#[test]
fn missing_csv_is_err() {
    let db = temp_db();
    assert!(import_and_total(&db, "nonexistent_c74.csv").is_err());
}
