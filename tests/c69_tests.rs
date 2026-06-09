#[path = "../src/bin/c69_exercise.rs"]
#[allow(dead_code)]
mod c69_exercise;

use c69_exercise::put;

fn temp_db() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

#[test]
fn insert_stores_entries() {
    let db = temp_db();
    put(&db, "svc:gel", "4500").unwrap();
    put(&db, "svc:pedicure", "3500").unwrap();
    assert_eq!(db.len(), 2);
    assert!(db.contains_key("svc:gel").unwrap());
}

#[test]
fn missing_key_absent() {
    let db = temp_db();
    put(&db, "svc:gel", "4500").unwrap();
    assert!(!db.contains_key("svc:nope").unwrap());
}
