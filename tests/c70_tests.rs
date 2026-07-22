#[path = "../src/bin/c70_exercise.rs"]
#[allow(dead_code)]
mod c70_exercise;

use c70_exercise::retrieve;

fn temp_vault() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

#[test]
fn stashed_shard_comes_back() {
    let db = temp_vault();
    db.insert("vault:blueprints", "aegis-9 tower schematics".as_bytes()).unwrap();
    assert_eq!(
        retrieve(&db, "vault:blueprints").unwrap(),
        Some("aegis-9 tower schematics".to_string())
    );
}

#[test]
fn burned_intel_is_none() {
    let db = temp_vault();
    assert_eq!(retrieve(&db, "vault:burned").unwrap(), None);
}
