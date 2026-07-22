#[path = "../src/bin/c72_exercise.rs"]
#[allow(dead_code)]
mod c72_exercise;

use c72_exercise::{Intel, full_scan};

fn temp_vault() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

fn seed(db: &sled::Db, codename: &str, value: u32) {
    let bytes = serde_json::to_vec(&Intel { codename: codename.to_string(), value }).unwrap();
    db.insert(codename, bytes).unwrap();
}

#[test]
fn scan_enumerates_the_whole_haul() {
    let db = temp_vault();
    seed(&db, "GHOSTKEY", 64000);
    seed(&db, "BLACKOUT", 42000);
    seed(&db, "EXEC-DIRT", 18500);
    assert_eq!(
        full_scan(&db).unwrap(),
        vec![
            Intel { codename: "BLACKOUT".to_string(), value: 42000 },
            Intel { codename: "EXEC-DIRT".to_string(), value: 18500 },
            Intel { codename: "GHOSTKEY".to_string(), value: 64000 },
        ]
    );
}

#[test]
fn empty_vault_scans_empty() {
    let db = temp_vault();
    assert_eq!(full_scan(&db).unwrap(), vec![]);
}
