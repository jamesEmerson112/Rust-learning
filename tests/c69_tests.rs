#[path = "../src/bin/c69_exercise.rs"]
#[allow(dead_code)]
mod c69_exercise;

use c69_exercise::stash;

fn temp_vault() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

#[test]
fn two_shards_land_in_the_vault() {
    let db = temp_vault();
    stash(&db, "vault:blueprints", "aegis-9 tower schematics").unwrap();
    stash(&db, "vault:payroll", "executive shell accounts").unwrap();
    assert_eq!(db.len(), 2);
}

#[test]
fn stashed_shard_is_findable() {
    let db = temp_vault();
    stash(&db, "vault:blueprints", "aegis-9 tower schematics").unwrap();
    assert!(db.contains_key("vault:blueprints").unwrap());
    assert!(!db.contains_key("vault:decoy").unwrap());
}
