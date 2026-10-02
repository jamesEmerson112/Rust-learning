#[path = "../src/bin/c69_exercise.rs"]
#[allow(dead_code)]
mod c69_exercise;

use c69_exercise::store;

fn temp_db() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

#[test]
fn two_records_land_in_the_store() {
    let db = temp_db();
    store(&db, "stock:paracetamol", "500 mg tablets, shelf A3").unwrap();
    store(&db, "stock:insulin", "10 ml vials, fridge 2").unwrap();
    assert_eq!(db.len(), 2);
}

#[test]
fn stored_record_is_findable() {
    let db = temp_db();
    store(&db, "stock:paracetamol", "500 mg tablets, shelf A3").unwrap();
    assert!(db.contains_key("stock:paracetamol").unwrap());
    assert!(!db.contains_key("stock:unknown").unwrap());
}
