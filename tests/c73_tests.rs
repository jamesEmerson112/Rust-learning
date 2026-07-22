#[path = "../src/bin/c73_exercise.rs"]
#[allow(dead_code)]
mod c73_exercise;

use c73_exercise::{Intel, shortlist, shortlist_codenames};

fn temp_vault() -> sled::Db {
    sled::Config::new().temporary(true).open().unwrap()
}

fn seed(db: &sled::Db, codename: &str, value: u32) {
    let bytes = serde_json::to_vec(&Intel { codename: codename.to_string(), value }).unwrap();
    db.insert(codename, bytes).unwrap();
}

#[test]
fn shortlist_keeps_only_premium_intel() {
    let db = temp_vault();
    seed(&db, "GHOSTKEY", 64000);
    seed(&db, "BLACKOUT", 42000);
    seed(&db, "EXEC-DIRT", 18500);
    assert_eq!(
        shortlist(&db, 40000).unwrap(),
        vec![
            Intel { codename: "BLACKOUT".to_string(), value: 42000 },
            Intel { codename: "GHOSTKEY".to_string(), value: 64000 },
        ]
    );
}

#[test]
fn codenames_project_just_the_names() {
    let db = temp_vault();
    seed(&db, "GHOSTKEY", 64000);
    seed(&db, "BLACKOUT", 42000);
    seed(&db, "EXEC-DIRT", 18500);
    assert_eq!(
        shortlist_codenames(&db, 40000).unwrap(),
        vec!["BLACKOUT".to_string(), "GHOSTKEY".to_string()]
    );
}
