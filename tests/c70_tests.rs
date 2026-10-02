#[path = "../src/bin/c70_exercise.rs"]
#[allow(dead_code)]
mod c70_exercise;

use c70_exercise::fetch;

fn temp_db() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

#[test]
fn stored_record_comes_back() {
    let db = temp_db();
    db.insert("stock:paracetamol", "500 mg tablets, shelf A3".as_bytes()).unwrap();
    assert_eq!(
        fetch(&db, "stock:paracetamol").unwrap(),
        Some("500 mg tablets, shelf A3".to_string())
    );
}

#[test]
fn unstocked_drug_is_none() {
    let db = temp_db();
    assert_eq!(fetch(&db, "stock:unknown").unwrap(), None);
}
