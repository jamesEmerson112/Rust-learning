#[path = "../src/bin/c70_exercise.rs"]
#[allow(dead_code)]
mod c70_exercise;

use c70_exercise::get_value;

fn temp_db() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

#[test]
fn gets_existing_value() {
    let db = temp_db();
    db.insert("k", "hello".as_bytes()).unwrap();
    assert_eq!(get_value(&db, "k").unwrap(), Some("hello".to_string()));
}

#[test]
fn missing_key_is_none() {
    let db = temp_db();
    assert_eq!(get_value(&db, "nope").unwrap(), None);
}
